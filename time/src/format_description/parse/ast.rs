//! AST for parsing format descriptions.

use alloc::borrow::ToOwned as _;
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
    Literal(Spanned<&'a str>),
    /// Part of a type, along with its modifiers and nested format descriptions.
    Component {
        /// The version of the format description, which may affect how the component is parsed.
        version: FormatDescriptionVersion,
        /// Where the opening bracket was in the format string.
        opening_bracket: Location,
        /// Whitespace between the opening bracket and name.
        _leading_whitespace: Unused<Option<Spanned<&'a str>>>,
        /// The name of the component.
        name: Spanned<&'a str>,
        /// The modifiers for the component.
        modifiers: Box<[Modifier<'a>]>,
        /// The nested format descriptions within the component.
        nested_format_descriptions: Box<[NestedFormatDescription<'a>]>,
        /// Whitespace between the modifiers/nested format descriptions and closing bracket.
        _trailing_whitespace: Unused<Option<Spanned<&'a str>>>,
        /// Where the closing bracket was in the format string.
        closing_bracket: Location,
    },
}

/// A format description that is nested within another format description.
pub(super) struct NestedFormatDescription<'a> {
    /// Whitespace between the end of the previous item and the opening bracket.
    pub(super) leading_whitespace: Option<Spanned<&'a str>>,
    /// Where the opening bracket was in the format string.
    pub(super) opening_bracket: Location,
    /// The items within the nested format description.
    pub(super) items: Box<[Item<'a>]>,
    /// Where the closing bracket was in the format string.
    pub(super) closing_bracket: Location,
}

/// A modifier for a component.
pub(super) struct Modifier<'a> {
    /// Whitespace preceding the modifier.
    pub(super) _leading_whitespace: Unused<Spanned<&'a str>>,
    /// The key of the modifier.
    pub(super) key: Spanned<&'a str>,
    /// Where the colon of the modifier was in the format string.
    pub(super) _colon: Unused<Location>,
    /// The value of the modifier.
    pub(super) value: Spanned<&'a str>,
}

impl<'a> Modifier<'a> {
    fn from_leading_whitespace_and_token(
        leading_whitespace: Spanned<&'a str>,
        token: Spanned<&'a str>,
    ) -> Result<Self, Error> {
        let Some(colon_index) = token.bytes().position(|b| b == b':') else {
            return Err(Error {
                _inner: unused(token.span.error("modifier must be of the form `key:value`")),
                public: error::InvalidFormatDescription::InvalidModifier {
                    value: (*token).to_owned(),
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
                    Ok(Item::Literal("[".spanned(location.to(second_location))))
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

    let modifiers = Modifiers::parse(tokens)?;

    let mut nested_format_descriptions = Vec::new();
    while let Ok(description) = parse_nested(version, modifiers.span().end, tokens) {
        nested_format_descriptions.push(description);
    }

    if modifiers.trailing_whitespace.is_some()
        && let Some(first_nested) = nested_format_descriptions.first_mut()
    {
        first_nested.leading_whitespace = modifiers.trailing_whitespace;
    }

    let nested_fds_trailing_whitespace =
        if modifiers.trailing_whitespace.is_some() && nested_format_descriptions.is_empty() {
            modifiers.trailing_whitespace
        } else {
            tokens.next_if_whitespace()
        };

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
        opening_bracket,
        _leading_whitespace: unused(leading_whitespace),
        name,
        modifiers: modifiers.modifiers,
        nested_format_descriptions: nested_format_descriptions.into_boxed_slice(),
        _trailing_whitespace: unused(nested_fds_trailing_whitespace),
        closing_bracket,
    })
}

struct Modifiers<'a> {
    modifiers: Box<[Modifier<'a>]>,
    trailing_whitespace: Option<Spanned<&'a str>>,
}

impl<'a> Modifiers<'a> {
    /// Parse modifiers until there are none left. Returns the modifiers along with any trailing
    /// whitespace after the last modifier.
    fn parse<I>(tokens: &mut lexer::Lexed<I>) -> Result<Self, Error>
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
            [] => Span::DUMMY,
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
    let leading_whitespace = tokens.next_if_whitespace();
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

    Ok(NestedFormatDescription {
        leading_whitespace,
        opening_bracket,
        items,
        closing_bracket,
    })
}
