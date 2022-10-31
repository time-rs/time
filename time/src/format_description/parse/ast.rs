use alloc::string::String;
use alloc::vec::Vec;
use core::iter;
use core::iter::Peekable;

use super::{lexer, Error, Location, Span};

#[allow(variant_size_differences)]
pub(super) enum Item<'a> {
    Literal {
        value: &'a [u8],
        _span: Span,
    },
    EscapedBracket {
        _first: Location,
        _second: Location,
    },
    Component {
        _opening_bracket: Location,
        _leading_whitespace: Option<Whitespace<'a>>,
        name: Name<'a>,
        modifiers: Vec<Modifier<'a>>,
        _trailing_whitespace: Option<Whitespace<'a>>,
        _closing_bracket: Location,
    },
}

pub(super) struct Whitespace<'a> {
    pub(super) _value: &'a [u8],
    pub(super) span: Span,
}

pub(super) struct Name<'a> {
    pub(super) value: &'a [u8],
    pub(super) span: Span,
}

pub(super) struct Modifier<'a> {
    pub(super) _leading_whitespace: Whitespace<'a>,
    pub(super) key: Key<'a>,
    pub(super) _colon: Location,
    pub(super) value: Value<'a>,
}

pub(super) struct Key<'a> {
    pub(super) value: &'a [u8],
    pub(super) span: Span,
}

pub(super) struct Value<'a> {
    pub(super) value: &'a [u8],
    pub(super) span: Span,
}

pub(super) fn parse<'a>(
    tokens: impl Iterator<Item = lexer::Token<'a>>,
) -> impl Iterator<Item = Result<Item<'a>, Error>> {
    let mut tokens = tokens.peekable();
    iter::from_fn(move || {
        Some(match tokens.next()? {
            lexer::Token::Literal { value, span } => Ok(Item::Literal { value, _span: span }),
            lexer::Token::Bracket {
                kind: lexer::BracketKind::Opening,
                location,
            } => {
                // escaped bracket
                if let Some(&lexer::Token::Bracket {
                    kind: lexer::BracketKind::Opening,
                    location: second_location,
                }) = tokens.peek()
                {
                    tokens.next(); // consume
                    Ok(Item::EscapedBracket {
                        _first: location,
                        _second: second_location,
                    })
                }
                // component
                else {
                    parse_component(location, &mut tokens)
                }
            }
            lexer::Token::Bracket {
                kind: lexer::BracketKind::Closing,
                location: _,
            } => unreachable!(
                "internal error: closing bracket should have been consumed by `parse_component`",
            ),
            lexer::Token::ComponentPart {
                kind: _,
                value: _,
                span: _,
            } => unreachable!(
                "internal error: component part should have been consumed by `parse_component`",
            ),
        })
    })
}

fn parse_component<'a>(
    opening_bracket: Location,
    tokens: &mut Peekable<impl Iterator<Item = lexer::Token<'a>>>,
) -> Result<Item<'a>, Error> {
    let leading_whitespace = if let Some(&lexer::Token::ComponentPart {
        kind: lexer::ComponentKind::Whitespace,
        value,
        span,
    }) = tokens.peek()
    {
        tokens.next(); // consume
        Some(Whitespace {
            _value: value,
            span,
        })
    } else {
        None
    };

    let name = if let Some(&lexer::Token::ComponentPart {
        kind: lexer::ComponentKind::NotWhitespace,
        value,
        span,
    }) = tokens.peek()
    {
        tokens.next(); // consume
        Name { value, span }
    } else {
        let span = leading_whitespace.map_or_else(
            || Span {
                start: opening_bracket,
                end: opening_bracket,
            },
            |whitespace| whitespace.span.shrink_to_end(),
        );
        return Err(Error {
            _inner: span.error("expected component name"),
            public: crate::error::InvalidFormatDescription::MissingComponentName {
                index: span.start_byte(),
            },
        });
    };

    let mut modifiers = Vec::new();
    let trailing_whitespace = loop {
        let whitespace = if let Some(&lexer::Token::ComponentPart {
            kind: lexer::ComponentKind::Whitespace,
            value,
            span,
        }) = tokens.peek()
        {
            tokens.next(); // consume
            Whitespace {
                _value: value,
                span,
            }
        } else {
            break None;
        };

        if let Some(&lexer::Token::ComponentPart {
            kind: lexer::ComponentKind::NotWhitespace,
            value,
            span,
        }) = tokens.peek()
        {
            tokens.next(); // consume

            let colon_index = match value.iter().position(|&b| b == b':') {
                Some(index) => index,
                None => {
                    return Err(Error {
                        _inner: span.error("modifier must be of the form `key:value`"),
                        public: crate::error::InvalidFormatDescription::InvalidModifier {
                            value: String::from_utf8_lossy(value).into_owned(),
                            index: span.start_byte(),
                        },
                    });
                }
            };
            let key = &value[..colon_index];
            let value = &value[colon_index + 1..];

            if key.is_empty() {
                return Err(Error {
                    _inner: span.shrink_to_start().error("expected modifier key"),
                    public: crate::error::InvalidFormatDescription::InvalidModifier {
                        value: String::new(),
                        index: span.start_byte(),
                    },
                });
            }
            if value.is_empty() {
                return Err(Error {
                    _inner: span.shrink_to_end().error("expected modifier value"),
                    public: crate::error::InvalidFormatDescription::InvalidModifier {
                        value: String::new(),
                        index: span.shrink_to_end().start_byte(),
                    },
                });
            }

            modifiers.push(Modifier {
                _leading_whitespace: whitespace,
                key: Key {
                    value: key,
                    span: span.subspan(..colon_index),
                },
                _colon: span.start.offset(colon_index),
                value: Value {
                    value,
                    span: span.subspan(colon_index + 1..),
                },
            });
        } else {
            break Some(whitespace);
        }
    };

    let closing_bracket = if let Some(&lexer::Token::Bracket {
        kind: lexer::BracketKind::Closing,
        location,
    }) = tokens.peek()
    {
        tokens.next(); // consume
        location
    } else {
        return Err(Error {
            _inner: opening_bracket.error("unclosed bracket"),
            public: crate::error::InvalidFormatDescription::UnclosedOpeningBracket {
                index: opening_bracket.byte,
            },
        });
    };

    Ok(Item::Component {
        _opening_bracket: opening_bracket,
        _leading_whitespace: leading_whitespace,
        name,
        modifiers,
        _trailing_whitespace: trailing_whitespace,
        _closing_bracket: closing_bracket,
    })
}
