//! Lexer for parsing format descriptions.

use alloc::borrow::ToOwned as _;
use alloc::string::String;
use alloc::vec::Vec;

use super::format_item::{Item, ident_eq};
use super::{
    Error, Location, Span, Spanned, SpannedValue, WithLocation, WithLocationValue as _, unused,
};
use crate::error::InvalidFormatDescription;
use crate::hint;
use crate::internal_macros::{const_try_opt, try_likely_ok};

#[must_use]
enum Context {
    Component,
    Literal,
}

impl Context {
    #[inline]
    const fn is_component(&self) -> bool {
        matches!(self, Self::Component)
    }

    #[inline]
    const fn is_literal(&self) -> bool {
        matches!(self, Self::Literal)
    }
}

enum NextModifier<'a> {
    Modifier(Modifier<'a>),
    TrailingWhitespace(Spanned<&'a str>),
    None,
}

/// An iterator over the lexed tokens.
pub(super) struct Lexer<'input, const VERSION: u8> {
    input: &'input [u8],
    depth: u8,
    byte_pos: u32,
}

impl<'input, const VERSION: u8> Lexer<'input, VERSION> {
    /// Parse the string into a series of [`Token`]s.
    ///
    /// `VERSION` controls the version of the format description that is being parsed.
    ///
    /// - When `VERSION` is 1, `[[` is the only escape sequence, resulting in a literal `[`. For the
    ///   start of a nested format description, a single `[` is used and is _never_ part of the
    ///   escape sequence. For example, `[optional [[day]]]` will lex successfully, ultimately
    ///   resulting in a component named `optional` with the nested component `day`.
    /// - When `VERSION` is 2 or 3, all escape sequences begin with `\`. The only characters that
    ///   may currently follow are `\`, `[`, and `]`, all of which result in the literal character.
    ///   All other characters result in a lex error.
    #[inline]
    pub(super) const fn new(input: &'input str) -> Self {
        Self {
            input: input.as_bytes(),
            depth: 0,
            byte_pos: 0,
        }
    }

    /// Advance the input by the given number of bytes.
    #[inline]
    fn advance(&mut self, bytes: u32) {
        self.input = &self.input[bytes as usize..];
        self.byte_pos += bytes;
    }

    /// Whether the lexer is currently parsing a component or a literal.
    #[inline]
    const fn context(&self) -> Context {
        if self.depth.is_multiple_of(2) {
            Context::Literal
        } else {
            Context::Component
        }
    }

    /// Consume the next token if it is a component item that is whitespace.
    #[inline]
    fn consume_whitespace(&mut self) -> Option<Spanned<&'input str>> {
        debug_assert!(self.context().is_component());

        let bytes = self
            .input
            .iter()
            .take_while(|byte| byte.is_ascii_whitespace())
            .count() as u32;

        if bytes == 0 {
            return None;
        }

        let start_loc = Location {
            byte: self.byte_pos,
        };
        let end_loc = Location {
            byte: self.byte_pos + bytes,
        };

        // Safety: Runtime format descriptions always originate with a string passed as a parameter
        // and we have only consumed full codepoints, ensuring that a valid string remains.
        let value = unsafe { str::from_utf8_unchecked(&self.input[..bytes as usize]) };
        self.advance(bytes);

        Some(value.spanned(start_loc.to(end_loc)))
    }

    /// Consume the next token if it is a component item that is not whitespace.
    #[inline]
    fn consume_component_part(&mut self) -> Option<Spanned<&'input str>> {
        debug_assert!(self.context().is_component());

        let bytes = self
            .input
            .iter()
            .take_while(|byte| !byte.is_ascii_whitespace() && !matches!(byte, b'\\' | b'[' | b']'))
            .count() as u32;

        if bytes == 0 {
            hint::cold_path();
            return None;
        }

        let start_loc = Location {
            byte: self.byte_pos,
        };
        let end_loc = Location {
            byte: self.byte_pos + bytes,
        };

        // Safety: Runtime format descriptions always originate with a string passed as a parameter
        // and we have only consumed full codepoints, ensuring that a valid string remains.
        let value = unsafe { str::from_utf8_unchecked(&self.input[..bytes as usize]) };
        self.advance(bytes);

        Some(value.spanned(start_loc.to(end_loc)))
    }

    /// Consume the next token if it is a closing bracket.
    #[inline]
    fn consume_closing_bracket(&mut self) -> Option<Location> {
        if self.input.first() != Some(&b']') {
            hint::cold_path();
            return None;
        }

        self.depth -= 1;

        let location = Location {
            byte: self.byte_pos,
        };
        self.advance(1);
        Some(location)
    }

    /// Consume the next token if it is a component name. The caller is expected to be inside a
    /// component header.
    #[inline]
    fn consume_component_name(
        &mut self,
        opening_bracket: Location,
    ) -> Result<Spanned<&'input str>, Error> {
        let leading_whitespace = self.consume_whitespace().is_some();

        let Some(name) = self.consume_component_part() else {
            hint::cold_path();
            let location = if leading_whitespace {
                opening_bracket.offset(1)
            } else {
                opening_bracket
            };
            return Err(Error {
                _inner: unused(location.error("expected component name")),
                public: InvalidFormatDescription::MissingComponentName {
                    index: location.byte as usize,
                },
            });
        };

        Ok(name)
    }

    #[inline]
    fn consume_modifier(&mut self) -> Result<NextModifier<'input>, Error> {
        let Some(whitespace) = self.consume_whitespace() else {
            hint::cold_path();
            return Ok(NextModifier::None);
        };

        let Some(token) = self.consume_component_part() else {
            hint::cold_path();
            return Ok(NextModifier::TrailingWhitespace(whitespace));
        };

        let modifier = try_likely_ok!(self.modifier_from_token(token));
        Ok(NextModifier::Modifier(modifier))
    }

    /// Parse a component.
    fn consume_component(
        &mut self,
        opening_bracket: Location,
    ) -> Result<Item<'input, VERSION>, Error> {
        match self.depth.checked_add(1) {
            Some(depth) => self.depth = depth,
            None => {
                hint::cold_path();
                return Err(Error {
                    _inner: unused(opening_bracket.error("too much nesting")),
                    public: InvalidFormatDescription::NotSupported {
                        what: "highly-nested format description",
                        context: "",
                        index: opening_bracket.byte as usize,
                    },
                });
            }
        };
        // consume the opening bracket, which was checked prior to calling this method
        self.advance(1);

        let name = try_likely_ok!(self.consume_component_name(opening_bracket));
        let modifiers = try_likely_ok!(Modifiers::parse(self));

        let mut nested_format_descriptions = Vec::new();
        while self.is_nested_description_start()
            && let Ok(description) = self.consume_nested(modifiers.end_location())
        {
            nested_format_descriptions.push(description);
        }

        if modifiers.trailing_whitespace.is_some()
            && let Some(first_nested) = nested_format_descriptions.first_mut()
        {
            first_nested.leading_whitespace = modifiers.trailing_whitespace;
        }

        if modifiers.trailing_whitespace.is_none() || !nested_format_descriptions.is_empty() {
            self.consume_whitespace();
        }

        let Some(closing_bracket) = self.consume_closing_bracket() else {
            hint::cold_path();
            return Err(Error {
                _inner: unused(opening_bracket.error("unclosed bracket")),
                public: InvalidFormatDescription::UnclosedOpeningBracket {
                    index: opening_bracket.byte as usize,
                },
            });
        };

        if let Some(first_nested_fd) = nested_format_descriptions.first()
            && first_nested_fd.leading_whitespace.is_none()
        {
            hint::cold_path();
            return Err(Error {
                _inner: unused(
                    opening_bracket
                        .to(closing_bracket)
                        .error("missing leading whitespace before nested format description"),
                ),
                public: InvalidFormatDescription::Expected {
                    what: "whitespace before nested format description",
                    index: first_nested_fd.opening_bracket.byte as usize,
                },
            });
        }

        if ident_eq::<VERSION>(*name, "optional") {
            hint::cold_path();
            return Item::optional_from_parts(
                opening_bracket,
                &modifiers.modifiers,
                nested_format_descriptions,
                closing_bracket,
            );
        }

        if ident_eq::<VERSION>(*name, "first") {
            hint::cold_path();
            if !modifiers.modifiers.is_empty() {
                hint::cold_path();
                let modifier = &modifiers.modifiers[0];
                return Err(Error {
                    _inner: unused(modifier.key_span().error("invalid modifier key")),
                    public: InvalidFormatDescription::InvalidModifier {
                        value: (*modifier.key).to_owned(),
                        index: modifier.key.location.byte as usize,
                    },
                });
            }

            if version!(3..) && nested_format_descriptions.is_empty() {
                hint::cold_path();
                return Err(Error {
                    _inner: unused(opening_bracket.to(closing_bracket).error(
                        "the `first` component requires at least one nested format description",
                    )),
                    public: InvalidFormatDescription::Expected {
                        what: "at least one nested format description",
                        index: closing_bracket.byte as usize,
                    },
                });
            }

            let items = nested_format_descriptions
                .into_iter()
                .map(|nested_format_description| nested_format_description.items)
                .collect();

            return Ok(Item::First {
                value: items,
                span: opening_bracket.to(closing_bracket),
            });
        }

        if !nested_format_descriptions.is_empty() {
            hint::cold_path();
            return Err(Error {
                _inner: unused(
                    opening_bracket
                        .to(closing_bracket)
                        .error("this component does not support nested format descriptions"),
                ),
                public: InvalidFormatDescription::NotSupported {
                    what: "nested format descriptions",
                    context: "on this component",
                    index: opening_bracket.byte as usize,
                },
            });
        }

        let component = try_likely_ok!(super::format_item::component_from_ast::<VERSION>(
            &name,
            &modifiers.modifiers
        ));

        Ok(Item::Component(component))
    }

    /// Parse a nested format description. The location provided is the most recent one consumed.
    #[inline]
    fn consume_nested(
        &mut self,
        last_location: Location,
    ) -> Result<NestedFormatDescription<'input, VERSION>, Error> {
        let leading_whitespace = self.consume_whitespace();

        let opening_bracket = {
            match self.depth.checked_add(1) {
                Some(depth) => self.depth = depth,
                None => {
                    hint::cold_path();
                    return Err(Error {
                        _inner: unused(last_location.error("too much nesting")),
                        public: InvalidFormatDescription::NotSupported {
                            what: "highly-nested format description",
                            context: "",
                            index: last_location.byte as usize,
                        },
                    });
                }
            }
            let location = Location {
                byte: self.byte_pos,
            };
            self.advance(1);
            location
        };

        let mut items = Vec::new();
        loop {
            // If we're in a literal context and the next byte is a closing bracket, stop so that we
            // can consume it.
            if self.context().is_literal() && self.input.first() == Some(&b']') {
                break;
            }

            let Some(token) = self.next() else {
                break;
            };
            items.push(try_likely_ok!(token));
        }

        let Some(closing_bracket) = self.consume_closing_bracket() else {
            hint::cold_path();
            return Err(Error {
                _inner: unused(opening_bracket.error("unclosed bracket")),
                public: InvalidFormatDescription::UnclosedOpeningBracket {
                    index: opening_bracket.byte as usize,
                },
            });
        };

        Ok(NestedFormatDescription {
            leading_whitespace,
            opening_bracket,
            items,
            closing_bracket,
        })
    }

    fn modifier_from_token(&self, token: Spanned<&'input str>) -> Result<Modifier<'input>, Error> {
        let Some(colon_index) = token.bytes().position(|b| b == b':') else {
            hint::cold_path();
            return Err(Error {
                _inner: unused(token.span.error("modifier must be of the form `key:value`")),
                public: InvalidFormatDescription::InvalidModifier {
                    value: (*token).to_owned(),
                    index: token.span.start.byte as usize,
                },
            });
        };
        let key = &token[..colon_index];
        let value = &token[colon_index + 1..];

        if key.is_empty() {
            hint::cold_path();
            return Err(Error {
                _inner: unused(token.span.shrink_to_start().error("expected modifier key")),
                public: InvalidFormatDescription::InvalidModifier {
                    value: String::new(),
                    index: token.span.start.byte as usize,
                },
            });
        }
        if value.is_empty() {
            hint::cold_path();
            return Err(Error {
                _inner: unused(token.span.shrink_to_end().error("expected modifier value")),
                public: InvalidFormatDescription::InvalidModifier {
                    value: String::new(),
                    index: token.span.start.byte as usize + colon_index,
                },
            });
        }

        Ok(Modifier {
            key: key.with_location(token.span.start),
            value,
        })
    }

    /// Check whether the next tokens start a nested format description. Does not consume any
    /// input.
    ///
    /// Note that this call is strictly an optimization, as checking the error path on
    /// `parse_nested` is sufficient for knowing if a nested format description is present. This
    /// method avoids the overhead of constructing an error only to throw it away.
    #[inline]
    fn is_nested_description_start(&self) -> bool {
        debug_assert!(self.context().is_component());

        let Some(index) = self
            .input
            .iter()
            .position(|&byte| !byte.is_ascii_whitespace())
        else {
            return false;
        };

        self.input[index] == b'['
            && (version!(2..)
                || self.context().is_component()
                || self.input.get(index + 1) != Some(&b'['))
    }

    #[inline]
    fn consume_literal(&mut self) -> &'input str {
        let bytes = self
            .input
            .iter()
            .take_while(|&&byte| byte != b'[' && byte != b']' && (version!(1) || byte != b'\\'))
            .count() as u32;

        // Safety: A string was passed to this function, and only UTF-8 has been consumed,
        // leaving behind a string known to begin at a character boundary.
        let value = unsafe { str::from_utf8_unchecked(&self.input[..bytes as usize]) };
        self.advance(bytes);

        value
    }

    #[inline]
    fn consume_backslash_escape_sequence(
        &mut self,
        location: Location,
    ) -> Result<&'input str, Error> {
        let backslash_loc = location;

        Ok(match self.input.get(1) {
            Some(b'\\' | b'[' | b']') => {
                // The escaped character is emitted as-is.
                // Safety: We know that this is either a left bracket, right bracket, or
                // backslash.
                let char = unsafe { str::from_utf8_unchecked(&self.input[1..2]) };
                self.advance(2);
                char
            }
            Some(_) => {
                hint::cold_path();
                let loc = Location {
                    byte: self.byte_pos + 1,
                };
                return Err(Error {
                    _inner: unused(loc.error("invalid escape sequence")),
                    public: InvalidFormatDescription::Expected {
                        what: "valid escape sequence",
                        index: loc.byte as usize,
                    },
                });
            }
            None => {
                hint::cold_path();
                return Err(Error {
                    _inner: unused(backslash_loc.error("unexpected end of input")),
                    public: InvalidFormatDescription::Expected {
                        what: "valid escape sequence",
                        index: backslash_loc.byte as usize,
                    },
                });
            }
        })
    }
}

impl<'input, const VERSION: u8> Iterator for Lexer<'input, VERSION> {
    type Item = Result<Item<'input, VERSION>, Error>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let byte = *const_try_opt!(self.input.first());

        let location = Location {
            byte: self.byte_pos,
        };

        match byte {
            b'[' if version!(1) && self.input.get(1) == Some(&b'[') => {
                self.advance(2);
                Some(Ok(Item::Literal("[")))
            }
            b'[' => Some(self.consume_component(location)),
            b']' if version!(3..) => {
                hint::cold_path();
                Some(Err(Error {
                    _inner: unused(location.error("right brackets must be escaped")),
                    public: InvalidFormatDescription::Expected {
                        what: "right bracket to be escaped",
                        index: location.byte as usize,
                    },
                }))
            }
            b']' => {
                self.advance(1);
                Some(Ok(Item::Literal("]")))
            }
            b'\\' if version!(2..) => Some(
                self.consume_backslash_escape_sequence(location)
                    .map(Item::Literal),
            ),
            _ => Some(Ok(Item::Literal(self.consume_literal()))),
        }
    }
}

/// A format description that is nested within another format description.
pub(super) struct NestedFormatDescription<'a, const VERSION: u8> {
    /// Whitespace between the end of the previous item and the opening bracket.
    pub(super) leading_whitespace: Option<Spanned<&'a str>>,
    /// Where the opening bracket was in the format string.
    pub(super) opening_bracket: Location,
    /// The items within the nested format description.
    pub(super) items: Vec<Item<'a, VERSION>>,
    /// Where the closing bracket was in the format string.
    pub(super) closing_bracket: Location,
}

/// A modifier for a component.
pub(super) struct Modifier<'a> {
    /// The key of the modifier.
    pub(super) key: WithLocation<&'a str>,
    /// The value of the modifier.
    pub(super) value: &'a str,
}

impl Modifier<'_> {
    #[inline]
    pub(super) fn key_value_span(&self) -> Span {
        self.key
            .location
            .with_length(self.key.len() + self.value.len() + 1)
    }

    #[inline]
    pub(super) fn key_span(&self) -> Span {
        self.key.location.with_length(self.key.len())
    }

    #[inline]
    pub(super) fn value_span(&self) -> Span {
        self.key
            .location
            .offset(self.key.len() as u32 + 1)
            .with_length(self.value.len())
    }
}

pub(super) struct Modifiers<'a> {
    pub(super) modifiers: Vec<Modifier<'a>>,
    pub(super) trailing_whitespace: Option<Spanned<&'a str>>,
}

impl<'a> Modifiers<'a> {
    /// Parse modifiers until there are none left. Returns the modifiers along with any trailing
    /// whitespace after the last modifier.
    #[inline]
    pub(super) fn parse<const VERSION: u8>(tokens: &mut Lexer<'a, VERSION>) -> Result<Self, Error> {
        let mut modifiers = Vec::new();
        loop {
            match try_likely_ok!(tokens.consume_modifier()) {
                NextModifier::Modifier(modifier) => modifiers.push(modifier),
                NextModifier::TrailingWhitespace(whitespace) => {
                    return Ok(Self {
                        modifiers,
                        trailing_whitespace: Some(whitespace),
                    });
                }
                NextModifier::None => {
                    return Ok(Self {
                        modifiers,
                        trailing_whitespace: None,
                    });
                }
            }
        }
    }

    #[inline]
    pub(super) fn end_location(&self) -> Location {
        match &*self.modifiers {
            [] => Location::DUMMY,
            [.., modifier] => modifier.value_span().end,
        }
    }
}
