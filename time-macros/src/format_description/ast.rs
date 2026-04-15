use std::iter;

use super::{Error, Location, Spanned, SpannedValue, Unused, lexer, unused};
use crate::FormatDescriptionVersion;
use crate::format_description::Span;

pub(super) enum Item<'a> {
    Literal {
        version: FormatDescriptionVersion,
        value: Spanned<&'a [u8]>,
    },
    Component {
        _opening_bracket: Unused<Location>,
        _leading_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
        name: Spanned<&'a [u8]>,
        modifiers: Box<[Modifier<'a>]>,
        _trailing_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
        _closing_bracket: Unused<Location>,
    },
    Optional {
        version: FormatDescriptionVersion,
        opening_bracket: Location,
        _leading_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
        _optional_kw: Unused<Spanned<&'a [u8]>>,
        modifiers: Box<[Modifier<'a>]>,
        _whitespace_after_modifiers: Unused<Option<Spanned<&'a [u8]>>>,
        nested_format_description: NestedFormatDescription<'a>,
        closing_bracket: Location,
    },
    First {
        opening_bracket: Location,
        _leading_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
        _first_kw: Unused<Spanned<&'a [u8]>>,
        modifiers: Box<[Modifier<'a>]>,
        _whitespace_after_modifiers: Unused<Option<Spanned<&'a [u8]>>>,
        nested_format_descriptions: Box<[NestedFormatDescription<'a>]>,
        closing_bracket: Location,
    },
}

pub(super) struct NestedFormatDescription<'a> {
    pub(super) _opening_bracket: Unused<Location>,
    pub(super) items: Box<[Item<'a>]>,
    pub(super) _closing_bracket: Unused<Location>,
    pub(super) _trailing_whitespace: Unused<Option<Spanned<&'a [u8]>>>,
}

#[derive(Debug)]
pub(super) struct Modifier<'a> {
    pub(super) _leading_whitespace: Unused<Spanned<&'a [u8]>>,
    pub(super) key: Spanned<&'a [u8]>,
    pub(super) _colon: Unused<Location>,
    pub(super) value: Spanned<&'a [u8]>,
}

impl<'a> Modifier<'a> {
    fn from_leading_whitespace_and_token(
        leading_whitespace: Spanned<&'a [u8]>,
        token: Spanned<&'a [u8]>,
    ) -> Result<Self, Error> {
        let Some(colon_index) = token.iter().position(|&b| b == b':') else {
            return Err(token.span.error("modifier must be of the form `key:value`"));
        };
        let key = &token[..colon_index];
        let value = &token[colon_index + 1..];

        if key.is_empty() {
            return Err(token.span.shrink_to_start().error("expected modifier key"));
        }
        if value.is_empty() {
            return Err(token.span.shrink_to_end().error("expected modifier value"));
        }

        Ok(Self {
            _leading_whitespace: unused(leading_whitespace),
            key: key.spanned(token.span),
            _colon: unused(token.span.start.offset(colon_index as u32)),
            value: value.spanned(token.span),
        })
    }

    pub(super) fn key_value_span(&self) -> Span {
        self.key.span.start.to(self.value.span.end)
    }
}

pub(super) fn parse<'item: 'iter, 'iter, I: Iterator<Item = Result<lexer::Token<'item>, Error>>>(
    version: FormatDescriptionVersion,
    tokens: &'iter mut lexer::Lexed<I>,
) -> impl Iterator<Item = Result<Item<'item>, Error>> + use<'item, 'iter, I> {
    parse_inner(version, false, tokens)
}

fn parse_inner<'item, I: Iterator<Item = Result<lexer::Token<'item>, Error>>>(
    version: FormatDescriptionVersion,
    nested: bool,
    tokens: &mut lexer::Lexed<I>,
) -> impl Iterator<Item = Result<Item<'item>, Error>> + use<'_, 'item, I> {
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
            lexer::Token::Literal(value) => Ok(Item::Literal { version, value }),
            lexer::Token::Bracket {
                kind: lexer::BracketKind::Opening,
                location,
            } => {
                if version.is_v1()
                    && let Some(second_location) = tokens.next_if_opening_bracket()
                {
                    Ok(Item::Literal {
                        version,
                        value: b"[".as_slice().spanned(location.to(second_location)),
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
            lexer::Token::ComponentPart { kind: _, value } if nested => {
                Ok(Item::Literal { version, value })
            }
            lexer::Token::ComponentPart { kind: _, value: _ } => {
                bug!("component part should have been consumed by `parse_component`")
            }
        })
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
                return Err(location.error("modifier must be of the form `key:value`"));
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
                .unwrap_or_else(Span::dummy),
            [modifier] => modifier.key.span.start.to(modifier.value.span.end),
            [first, .., last] => first.key.span.start.to(last.value.span.end),
        }
    }
}

fn parse_component<'a, I: Iterator<Item = Result<lexer::Token<'a>, Error>>>(
    version: FormatDescriptionVersion,
    opening_bracket: Location,
    tokens: &mut lexer::Lexed<I>,
) -> Result<Item<'a>, Error> {
    let leading_whitespace = tokens.next_if_whitespace();

    let Some(name) = tokens.next_if_not_whitespace() else {
        let span = match leading_whitespace {
            Some(Spanned { value: _, span }) => span,
            None => opening_bracket.to(opening_bracket),
        };
        return Err(span.error("expected component name"));
    };

    if *name == b"optional" {
        let modifiers = Modifiers::parse(true, tokens)?;
        let nested = parse_nested(version, modifiers.span().end, tokens)?;

        let Some(closing_bracket) = tokens.next_if_closing_bracket() else {
            return Err(opening_bracket.error("unclosed bracket"));
        };

        if modifiers.trailing_whitespace.is_none() {
            if let Some(modifier) = modifiers.modifiers.last() {
                return Err(modifier
                    .value
                    .span
                    .shrink_to_end()
                    .error("expected whitespace between modifiers and nested description"));
            } else {
                return Err(name
                    .span
                    .shrink_to_end()
                    .error("expected whitespace between `optional` and nested description"));
            }
        }

        return Ok(Item::Optional {
            version,
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
            return Err(modifiers
                .span()
                .shrink_to_end()
                .error("expected at least one nested description"));
        }

        let Some(closing_bracket) = tokens.next_if_closing_bracket() else {
            return Err(opening_bracket.error("unclosed bracket"));
        };

        if modifiers.trailing_whitespace.is_none() {
            if let Some(modifier) = modifiers.modifiers.last() {
                return Err(modifier
                    .value
                    .span
                    .shrink_to_end()
                    .error("expected whitespace between modifiers and nested descriptions"));
            } else {
                return Err(name
                    .span
                    .shrink_to_end()
                    .error("expected whitespace between `first` and nested descriptions"));
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
        return Err(opening_bracket.error("unclosed bracket"));
    };

    Ok(Item::Component {
        _opening_bracket: unused(opening_bracket),
        _leading_whitespace: unused(leading_whitespace),
        name,
        modifiers,
        _trailing_whitespace: unused(trailing_whitespace),
        _closing_bracket: unused(closing_bracket),
    })
}

fn parse_nested<'a, I: Iterator<Item = Result<lexer::Token<'a>, Error>>>(
    version: FormatDescriptionVersion,
    last_location: Location,
    tokens: &mut lexer::Lexed<I>,
) -> Result<NestedFormatDescription<'a>, Error> {
    let Some(opening_bracket) = tokens.next_if_opening_bracket() else {
        return Err(last_location.error("expected opening bracket"));
    };
    let items = parse_inner(version, true, tokens).collect::<Result<_, _>>()?;
    let Some(closing_bracket) = tokens.next_if_closing_bracket() else {
        return Err(opening_bracket.error("unclosed bracket"));
    };
    let trailing_whitespace = tokens.next_if_whitespace();

    Ok(NestedFormatDescription {
        _opening_bracket: unused(opening_bracket),
        items,
        _closing_bracket: unused(closing_bracket),
        _trailing_whitespace: unused(trailing_whitespace),
    })
}
