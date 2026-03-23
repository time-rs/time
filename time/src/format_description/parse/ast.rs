//! AST for parsing format descriptions.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter;

use super::{Error, Location, Span, Spanned, SpannedValue, Unused, lexer, unused};
use crate::error;
use crate::format_description::FormatDescriptionVersion;
use crate::internal_macros::bug;

/// One part of a complete format description.
pub(super) enum Item<'a> {
    /// A literal string, formatted and parsed as-is.
    ///
    /// This should never be present inside a nested format description.
    Literal(Spanned<&'a [u8]>),
    /// A sequence of brackets. The first acts as the escape character.
    ///
    /// This should never be present if the lexer has `BACKSLASH_ESCAPE` set to `true`.
    EscapedBracket {
        /// The first bracket.
        _first: Unused<Location>,
        /// The second bracket.
        _second: Unused<Location>,
    },
    /// Part of a type, along with its modifiers.
    Component {
        version: FormatDescriptionVersion,
        /// Where the opening bracket was in the format string.
        _opening_bracket: Unused<Location>,
        /// Whitespace between the opening bracket and name.
        _leading_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
        /// The name of the component.
        name: Spanned<&'a [u8]>,
        /// The modifiers for the component.
        modifiers: Box<[Modifier<'a>]>,
        /// Whitespace between the modifiers and closing bracket.
        _trailing_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
        /// Where the closing bracket was in the format string.
        _closing_bracket: Unused<Location>,
    },
    /// An optional sequence of items.
    Optional {
        /// Where the opening bracket was in the format string.
        opening_bracket: Location,
        /// Whitespace between the opening bracket and "optional".
        _leading_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
        /// The "optional" keyword.
        _optional_kw: Unused<Spanned<&'a [u8]>>,
        /// The modifiers for the optional description.
        modifiers: Box<[Modifier<'a>]>,
        /// Whitespace between either the "optional" keyword or modifiers and the opening bracket
        /// of the nested description.
        _whitespace_after_modifiers: Unused<Option<Spanned<&'a [u8]>>>,
        /// The items within the optional sequence.
        nested_format_description: NestedFormatDescription<'a>,
        /// Where the closing bracket was in the format string.
        closing_bracket: Location,
    },
    /// The first matching parse of a sequence of items.
    First {
        /// Where the opening bracket was in the format string.
        opening_bracket: Location,
        /// Whitespace between the opening bracket and "first".
        _leading_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
        /// The "first" keyword.
        _first_kw: Unused<Spanned<&'a [u8]>>,
        /// The modifiers for the optional description.
        modifiers: Box<[Modifier<'a>]>,
        /// Whitespace between either the "first" keyword or modifiers and the opening bracket of
        /// the nested description.
        _whitespace_after_modifiers: Unused<Option<Spanned<&'a [u8]>>>,
        /// The sequences of items to try.
        nested_format_descriptions: Box<[NestedFormatDescription<'a>]>,
        /// Where the closing bracket was in the format string.
        closing_bracket: Location,
    },
}

/// A format description that is nested within another format description.
pub(super) struct NestedFormatDescription<'a> {
    /// Where the opening bracket was in the format string.
    pub(super) _opening_bracket: Unused<Location>,
    /// The items within the nested format description.
    pub(super) items: Box<[Item<'a>]>,
    /// Where the closing bracket was in the format string.
    pub(super) _closing_bracket: Unused<Location>,
    /// Whitespace between the closing bracket and the next item.
    pub(super) _trailing_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
}

/// A modifier for a component.
pub(super) struct Modifier<'a> {
    /// Whitespace preceding the modifier.
    pub(super) _leading_whitespace: Unused<Spanned<&'a [u8]>>,
    /// The key of the modifier.
    pub(super) key: Spanned<&'a [u8]>,
    /// Where the colon of the modifier was in the format string.
    pub(super) _colon: Unused<Location>,
    /// The value of the modifier.
    pub(super) value: Spanned<&'a [u8]>,
}

impl<'a> Modifier<'a> {
    fn from_leading_whitespace_and_token(
        leading_whitespace: Spanned<&'a [u8]>,
        token: Spanned<&'a [u8]>,
    ) -> Result<Self, Error> {
        let Some(colon_index) = token.iter().position(|&b| b == b':') else {
            return Err(Error {
                _inner: unused(token.span.error("modifier must be of the form `key:value`")),
                public: error::InvalidFormatDescription::InvalidModifier {
                    value: String::from_utf8_lossy(*token).into_owned(),
                    index: token.span.start.byte as usize,
                },
            });
        };
        let key = &token[..colon_index];
        let value = &token[colon_index + 1..];

        if key.is_empty() {
            return Err(Error {
                _inner: unused(token.span.shrink_to_start().error("expected modifier key")),
                public: error::InvalidFormatDescription::InvalidModifier {
                    value: String::new(),
                    index: token.span.start.byte as usize,
                },
            });
        }
        if value.is_empty() {
            return Err(Error {
                _inner: unused(token.span.shrink_to_end().error("expected modifier value")),
                public: error::InvalidFormatDescription::InvalidModifier {
                    value: String::new(),
                    index: token.span.start.byte as usize + colon_index,
                },
            });
        }

        Ok(Self {
            _leading_whitespace: unused(leading_whitespace),
            key: key.spanned(
                token
                    .span
                    .start
                    .to(token.span.start.offset(colon_index as u32)),
            ),
            _colon: unused(token.span.start.offset(colon_index as u32)),
            value: value.spanned(
                token
                    .span
                    .start
                    .offset(colon_index as u32 + 1)
                    .to(token.span.end),
            ),
        })
    }

    pub(super) const fn key_value_span(&self) -> Span {
        self.key.span.start.to(self.value.span.end)
    }
}

/// Parse the provided tokens into an AST.
#[inline]
pub(super) fn parse<'item, 'iter, I>(
    version: FormatDescriptionVersion,
    tokens: &'iter mut lexer::Lexed<I>,
) -> impl Iterator<Item = Result<Item<'item>, Error>> + use<'item, 'iter, I>
where
    'item: 'iter,
    I: Iterator<Item = Result<lexer::Token<'item>, Error>>,
{
    parse_inner(version, false, tokens)
}

/// Parse the provided tokens into an AST. The const generic indicates whether the resulting
/// [`Item`] will be used directly or as part of a [`NestedFormatDescription`].
#[inline]
fn parse_inner<'item, I>(
    version: FormatDescriptionVersion,
    nested: bool,
    tokens: &mut lexer::Lexed<I>,
) -> impl Iterator<Item = Result<Item<'item>, Error>> + use<'_, 'item, I>
where
    I: Iterator<Item = Result<lexer::Token<'item>, Error>>,
{
    iter::from_fn(move || {
        if nested && tokens.peek_closing_bracket().is_some() {
            return None;
        }

        let next = match tokens.next()? {
            Ok(token) => token,
            Err(err) => return Some(Err(err)),
        };

        Some(match next {
            lexer::Token::Literal(Spanned { value: _, span: _ }) if nested => {
                bug!("literal should not be present in nested description")
            }
            lexer::Token::Literal(value) => Ok(Item::Literal(value)),
            lexer::Token::Bracket {
                kind: lexer::BracketKind::Opening,
                location,
            } => {
                if version.is_v1()
                    && let Some(second_location) = tokens.next_if_opening_bracket()
                {
                    Ok(Item::EscapedBracket {
                        _first: unused(location),
                        _second: unused(second_location),
                    })
                } else {
                    parse_component(version, location, tokens)
                }
            }
            lexer::Token::Bracket {
                kind: lexer::BracketKind::Closing,
                location: _,
            } if nested => {
                bug!("closing bracket should be caught by the `if` statement")
            }
            lexer::Token::Bracket {
                kind: lexer::BracketKind::Closing,
                location: _,
            } => {
                bug!("closing bracket should have been consumed by `parse_component`")
            }
            lexer::Token::ComponentPart {
                kind: _, // whitespace is significant in nested components
                value,
            } if nested => Ok(Item::Literal(value)),
            lexer::Token::ComponentPart { kind: _, value: _ } => {
                bug!("component part should have been consumed by `parse_component`")
            }
        })
    })
}

/// Parse a component. This assumes that the opening bracket has already been consumed.
fn parse_component<'a, I>(
    version: FormatDescriptionVersion,
    opening_bracket: Location,
    tokens: &mut lexer::Lexed<I>,
) -> Result<Item<'a>, Error>
where
    I: Iterator<Item = Result<lexer::Token<'a>, Error>>,
{
    let leading_whitespace = tokens.next_if_whitespace();

    let Some(name) = tokens.next_if_not_whitespace() else {
        let span = match leading_whitespace {
            Some(Spanned { value: _, span }) => span,
            None => opening_bracket.to_self(),
        };
        return Err(Error {
            _inner: unused(span.error("expected component name")),
            public: error::InvalidFormatDescription::MissingComponentName {
                index: span.start.byte as usize,
            },
        });
    };

    if *name == b"optional" {
        let modifiers = Modifiers::parse(true, tokens)?;
        let nested = parse_nested(version, modifiers.span().end, tokens)?;

        let Some(closing_bracket) = tokens.next_if_closing_bracket() else {
            return Err(Error {
                _inner: unused(opening_bracket.error("unclosed bracket")),
                public: error::InvalidFormatDescription::UnclosedOpeningBracket {
                    index: opening_bracket.byte as usize,
                },
            });
        };

        if modifiers.trailing_whitespace.is_none() {
            if let Some(modifier) = modifiers.modifiers.last() {
                return Err(Error {
                    _inner: unused(
                        modifier
                            .value
                            .span
                            .shrink_to_end()
                            .error("expected whitespace between modifiers and nested description"),
                    ),
                    public: error::InvalidFormatDescription::Expected {
                        what: "whitespace between modifiers and nested description",
                        index: modifier.value.span.end.byte as usize,
                    },
                });
            } else {
                return Err(Error {
                    _inner: unused(
                        name.span
                            .shrink_to_end()
                            .error("expected whitespace between `optional` and nested description"),
                    ),
                    public: error::InvalidFormatDescription::Expected {
                        what: "whitespace between `optional` and nested description",
                        index: name.span.end.byte as usize,
                    },
                });
            }
        }

        return Ok(Item::Optional {
            opening_bracket,
            _leading_whitespace: unused(leading_whitespace),
            _optional_kw: unused(name),
            modifiers: modifiers.modifiers,
            _whitespace_after_modifiers: unused(modifiers.trailing_whitespace),
            nested_format_description: nested,
            closing_bracket,
        });
    }

    if *name == b"first" {
        let modifiers = Modifiers::parse(true, tokens)?;

        let mut nested_format_descriptions = Vec::new();
        while let Ok(description) = parse_nested(version, modifiers.span().end, tokens) {
            nested_format_descriptions.push(description);
        }

        if version.is_at_least_v3() && nested_format_descriptions.is_empty() {
            return Err(Error {
                _inner: unused(
                    modifiers
                        .span()
                        .shrink_to_end()
                        .error("expected at least one nested description"),
                ),
                public: error::InvalidFormatDescription::Expected {
                    what: "at least one nested description",
                    index: modifiers.span().end.byte as usize,
                },
            });
        }

        let Some(closing_bracket) = tokens.next_if_closing_bracket() else {
            return Err(Error {
                _inner: unused(opening_bracket.error("unclosed bracket")),
                public: error::InvalidFormatDescription::UnclosedOpeningBracket {
                    index: opening_bracket.byte as usize,
                },
            });
        };

        if modifiers.trailing_whitespace.is_none() {
            if let Some(modifier) = modifiers.modifiers.last() {
                return Err(Error {
                    _inner: unused(
                        modifier
                            .value
                            .span
                            .shrink_to_end()
                            .error("expected whitespace between modifiers and nested descriptions"),
                    ),
                    public: error::InvalidFormatDescription::Expected {
                        what: "whitespace between modifiers and nested descriptions",
                        index: modifier.value.span.end.byte as usize,
                    },
                });
            } else {
                return Err(Error {
                    _inner: unused(
                        name.span
                            .shrink_to_end()
                            .error("expected whitespace between `first` and nested descriptions"),
                    ),
                    public: error::InvalidFormatDescription::Expected {
                        what: "whitespace between `first` and nested descriptions",
                        index: name.span.end.byte as usize,
                    },
                });
            }
        }

        return Ok(Item::First {
            opening_bracket,
            _leading_whitespace: unused(leading_whitespace),
            _first_kw: unused(name),
            modifiers: modifiers.modifiers,
            _whitespace_after_modifiers: unused(modifiers.trailing_whitespace),
            nested_format_descriptions: nested_format_descriptions.into_boxed_slice(),
            closing_bracket,
        });
    }

    let Modifiers {
        modifiers,
        trailing_whitespace,
    } = Modifiers::parse(false, tokens)?;

    let Some(closing_bracket) = tokens.next_if_closing_bracket() else {
        return Err(Error {
            _inner: unused(opening_bracket.error("unclosed bracket")),
            public: error::InvalidFormatDescription::UnclosedOpeningBracket {
                index: opening_bracket.byte as usize,
            },
        });
    };

    Ok(Item::Component {
        version,
        _opening_bracket: unused(opening_bracket),
        _leading_whitespace: unused(leading_whitespace),
        name,
        modifiers,
        _trailing_whitespace: unused(trailing_whitespace),
        _closing_bracket: unused(closing_bracket),
    })
}

struct Modifiers<'a> {
    modifiers: Box<[Modifier<'a>]>,
    trailing_whitespace: Option<Spanned<&'a [u8]>>,
}

impl<'a> Modifiers<'a> {
    fn parse<I>(nested_is_allowed: bool, tokens: &mut lexer::Lexed<I>) -> Result<Self, Error>
    where
        I: Iterator<Item = Result<lexer::Token<'a>, Error>>,
    {
        let mut modifiers = Vec::new();
        loop {
            let Some(whitespace) = tokens.next_if_whitespace() else {
                return Ok(Self {
                    modifiers: modifiers.into_boxed_slice(),
                    trailing_whitespace: None,
                });
            };

            // This is not necessary for proper parsing, but provides a much better error when a
            // nested description is used where it's not allowed.
            if !nested_is_allowed && let Some(location) = tokens.next_if_opening_bracket() {
                return Err(Error {
                    _inner: unused(
                        location
                            .to_self()
                            .error("modifier must be of the form `key:value`"),
                    ),
                    public: error::InvalidFormatDescription::InvalidModifier {
                        value: String::from("["),
                        index: location.byte as usize,
                    },
                });
            }

            let Some(token) = tokens.next_if_not_whitespace() else {
                return Ok(Self {
                    modifiers: modifiers.into_boxed_slice(),
                    trailing_whitespace: Some(whitespace),
                });
            };

            let modifier = Modifier::from_leading_whitespace_and_token(whitespace, token)?;
            modifiers.push(modifier);
        }
    }

    fn span(&self) -> Span {
        match &*self.modifiers {
            [] => self
                .trailing_whitespace
                .map(|whitespace| whitespace.span)
                .unwrap_or(Span::DUMMY),
            [modifier] => modifier.key.span.start.to(modifier.value.span.end),
            [first, .., last] => first.key.span.start.to(last.value.span.end),
        }
    }
}

/// Parse a nested format description. The location provided is the most recent one consumed.
#[inline]
fn parse_nested<'a, I>(
    version: FormatDescriptionVersion,
    last_location: Location,
    tokens: &mut lexer::Lexed<I>,
) -> Result<NestedFormatDescription<'a>, Error>
where
    I: Iterator<Item = Result<lexer::Token<'a>, Error>>,
{
    let Some(opening_bracket) = tokens.next_if_opening_bracket() else {
        return Err(Error {
            _inner: unused(last_location.error("expected opening bracket")),
            public: error::InvalidFormatDescription::Expected {
                what: "opening bracket",
                index: last_location.byte as usize,
            },
        });
    };
    let items = parse_inner(version, true, tokens).collect::<Result<_, _>>()?;
    let Some(closing_bracket) = tokens.next_if_closing_bracket() else {
        return Err(Error {
            _inner: unused(opening_bracket.error("unclosed bracket")),
            public: error::InvalidFormatDescription::UnclosedOpeningBracket {
                index: opening_bracket.byte as usize,
            },
        });
    };
    let trailing_whitespace = tokens.next_if_whitespace();

    Ok(NestedFormatDescription {
        _opening_bracket: unused(opening_bracket),
        items,
        _closing_bracket: unused(closing_bracket),
        _trailing_whitespace: unused(trailing_whitespace),
    })
}
