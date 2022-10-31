use core::iter;

use super::{Location, Span};

pub(super) enum Token<'a> {
    Literal {
        value: &'a [u8],
        span: Span,
    },
    Bracket {
        kind: BracketKind,
        location: Location,
    },
    ComponentPart {
        kind: ComponentKind,
        value: &'a [u8],
        span: Span,
    },
}

pub(super) enum BracketKind {
    Opening,
    Closing,
}

pub(super) enum ComponentKind {
    Whitespace,
    NotWhitespace,
}

fn attach_location(iter: impl Iterator<Item = u8>) -> impl Iterator<Item = (u8, Location)> {
    let mut line = 1;
    let mut column = 1;
    let mut byte_pos = 0;

    iter.map(move |byte| {
        let location = Location {
            line,
            column,
            byte: byte_pos,
        };
        column += 1;
        byte_pos += 1;

        if byte == b'\n' {
            line += 1;
            column = 1;
        }

        (byte, location)
    })
}

pub(super) fn lex(mut input: &[u8]) -> impl Iterator<Item = Token<'_>> {
    let mut depth: u8 = 0;
    let mut iter = attach_location(input.iter().copied()).peekable();
    let mut second_bracket_location = None;

    iter::from_fn(move || {
        // There is a flag set to emit the second half of an escaped bracket pair.
        if let Some(location) = second_bracket_location.take() {
            return Some(Token::Bracket {
                kind: BracketKind::Opening,
                location,
            });
        }

        Some(match iter.next()? {
            (b'[', location) => {
                if let Some((_, second_location)) = iter.next_if(|&(byte, _)| byte == b'[') {
                    // escaped bracket
                    second_bracket_location = Some(second_location);
                    input = &input[2..];
                } else {
                    // opening bracket
                    depth += 1;
                    input = &input[1..];
                }

                Token::Bracket {
                    kind: BracketKind::Opening,
                    location,
                }
            }
            // closing bracket
            (b']', location) if depth > 0 => {
                depth -= 1;
                input = &input[1..];
                Token::Bracket {
                    kind: BracketKind::Closing,
                    location,
                }
            }
            // literal
            (_, start_location) if depth == 0 => {
                let mut bytes = 1;
                let mut end_location = start_location;

                while let Some((_, location)) = iter.next_if(|&(byte, _)| byte != b'[') {
                    end_location = location;
                    bytes += 1;
                }

                let value = &input[..bytes];
                input = &input[bytes..];
                Token::Literal {
                    value,
                    span: Span::start_end(start_location, end_location),
                }
            }
            // component part
            (byte, start_location) => {
                let mut bytes = 1;
                let mut end_location = start_location;
                let is_whitespace = byte.is_ascii_whitespace();

                while let Some((_, location)) = iter.next_if(|&(byte, _)| {
                    byte != b'[' && byte != b']' && is_whitespace == byte.is_ascii_whitespace()
                }) {
                    end_location = location;
                    bytes += 1;
                }

                let value = &input[..bytes];
                input = &input[bytes..];
                Token::ComponentPart {
                    kind: if is_whitespace {
                        ComponentKind::Whitespace
                    } else {
                        ComponentKind::NotWhitespace
                    },
                    value,
                    span: Span::start_end(start_location, end_location),
                }
            }
        })
    })
}
