//! Lexer for parsing format descriptions.

use core::iter;

use super::{Error, Location, Spanned, SpannedValue, attach_location, unused};
use crate::format_description::FormatDescriptionVersion;

/// An iterator over the lexed tokens.
pub(super) struct Lexed<I>
where
    I: Iterator,
{
    /// The internal iterator.
    iter: iter::Peekable<I>,
}

impl<I> Iterator for Lexed<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'iter, 'token, I> Lexed<I>
where
    'token: 'iter,
    I: Iterator<Item = Result<Token<'token>, Error>> + 'iter,
{
    /// Peek at the next item in the iterator.
    #[inline]
    pub(super) fn peek(&mut self) -> Option<&I::Item> {
        self.iter.peek()
    }

    /// Consume the next token if it is whitespace.
    #[inline]
    pub(super) fn next_if_whitespace(&mut self) -> Option<Spanned<&'token str>> {
        if let Some(&Ok(Token::ComponentPart {
            kind: ComponentKind::Whitespace,
            value,
        })) = self.peek()
        {
            self.next(); // consume
            Some(value)
        } else {
            None
        }
    }

    /// Consume the next token if it is a component item that is not whitespace.
    #[inline]
    pub(super) fn next_if_not_whitespace(&mut self) -> Option<Spanned<&'token str>> {
        if let Some(&Ok(Token::ComponentPart {
            kind: ComponentKind::NotWhitespace,
            value,
        })) = self.peek()
        {
            self.next(); // consume
            Some(value)
        } else {
            None
        }
    }

    /// Consume the next token if it is an opening bracket.
    #[inline]
    pub(super) fn next_if_opening_bracket(&mut self) -> Option<Location> {
        if let Some(&Ok(Token::Bracket {
            kind: BracketKind::Opening,
            location,
        })) = self.peek()
        {
            self.next(); // consume
            Some(location)
        } else {
            None
        }
    }

    /// Peek at the next token if it is a closing bracket.
    #[inline]
    pub(super) fn peek_closing_bracket(&'iter mut self) -> Option<&'iter Location> {
        if let Some(Ok(Token::Bracket {
            kind: BracketKind::Closing,
            location,
        })) = self.peek()
        {
            Some(location)
        } else {
            None
        }
    }

    /// Consume the next token if it is a closing bracket.
    #[inline]
    pub(super) fn next_if_closing_bracket(&mut self) -> Option<Location> {
        if let Some(&Ok(Token::Bracket {
            kind: BracketKind::Closing,
            location,
        })) = self.peek()
        {
            self.next(); // consume
            Some(location)
        } else {
            None
        }
    }
}

/// A token emitted by the lexer. There is no semantic meaning at this stage.
pub(super) enum Token<'a> {
    /// A literal string, formatted and parsed as-is.
    Literal(Spanned<&'a str>),
    /// An opening or closing bracket. May or may not be the start or end of a component.
    Bracket {
        /// Whether the bracket is opening or closing.
        kind: BracketKind,
        /// Where the bracket was in the format string.
        location: Location,
    },
    /// One part of a component. This could be its name, a modifier, or whitespace.
    ComponentPart {
        /// Whether the part is whitespace or not.
        kind: ComponentKind,
        /// The part itself.
        value: Spanned<&'a str>,
    },
}

/// What type of bracket is present.
pub(super) enum BracketKind {
    /// An opening bracket: `[`
    Opening,
    /// A closing bracket: `]`
    Closing,
}

/// Indicates whether the component is whitespace or not.
pub(super) enum ComponentKind {
    Whitespace,
    NotWhitespace,
}

/// Parse the string into a series of [`Token`]s.
///
/// `version` controls the version of the format description that is being parsed.
///
/// - When `version` is 1, `[[` is the only escape sequence, resulting in a literal `[`. For the
///   start of a nested format description, a single `[` is used and is _never_ part of the escape
///   sequence. For example, `[optional [[day]]]` will lex successfully, ultimately resulting in a
///   component named `optional` with the nested component `day`.
/// - When `version` is 2 or 3, all escape sequences begin with `\`. The only characters that may
///   currently follow are `\`, `[`, and `]`, all of which result in the literal character. All
///   other characters result in a lex error.
#[inline]
pub(super) fn lex(
    version: FormatDescriptionVersion,
    input: &str,
) -> Lexed<impl Iterator<Item = Result<Token<'_>, Error>>> {
    // Avoid checking for character boundaries on every indexing operation. Everything still results
    // in valid UTF-8.
    let mut input = input.as_bytes();
    let mut depth: u32 = 0;
    // Whether, within a nested format description, we have seen the component name. This is used to
    // distinguish between `[[` as an escaped literal and `[[` as the start of a nested format
    // description (and the start of a component). This is only relevant for v1 format descriptions.
    let mut nested_component_name_seen = false;
    let mut iter = attach_location(input.iter()).peekable();
    let mut second_bracket_location = None;

    let iter = iter::from_fn(move || {
        // The flag is only set when version is zero.
        if version.is_v1() {
            // There is a flag set to emit the second half of an escaped bracket pair.
            if let Some(location) = second_bracket_location.take() {
                return Some(Ok(Token::Bracket {
                    kind: BracketKind::Opening,
                    location,
                }));
            }
        }

        Some(Ok(match iter.next()? {
            // possible escape sequence
            (b'\\', backslash_loc) if version.is_at_least_v2() => {
                match iter.next() {
                    Some((b'\\' | b'[' | b']', char_loc)) => {
                        // The escaped character is emitted as-is.
                        // Safety: We know that this is either a left bracket, right bracket, or
                        // backslash.
                        let char = unsafe { str::from_utf8_unchecked(&input[1..2]) };
                        input = &input[2..];
                        if depth == 0 {
                            Token::Literal(char.spanned(backslash_loc.to(char_loc)))
                        } else {
                            Token::ComponentPart {
                                kind: ComponentKind::NotWhitespace,
                                value: char.spanned(backslash_loc.to(char_loc)),
                            }
                        }
                    }
                    Some((_, loc)) => {
                        return Some(Err(Error {
                            _inner: unused(loc.error("invalid escape sequence")),
                            public: crate::error::InvalidFormatDescription::Expected {
                                what: "valid escape sequence",
                                index: loc.byte as usize,
                            },
                        }));
                    }
                    None => {
                        return Some(Err(Error {
                            _inner: unused(backslash_loc.error("unexpected end of input")),
                            public: crate::error::InvalidFormatDescription::Expected {
                                what: "valid escape sequence",
                                index: backslash_loc.byte as usize,
                            },
                        }));
                    }
                }
            }
            // potentially escaped opening bracket
            // If we have seen a nested component name, then we know for sure that this is not
            // an escaped bracket. If we have not, then we check for the escape sequence.
            (b'[', location) if version.is_v1() && !nested_component_name_seen => {
                if let Some((_, second_location)) = iter.next_if(|&(&byte, _)| byte == b'[') {
                    // Escaped bracket. Store the location of the second so we can emit it later.
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
            // opening bracket
            (b'[', location) => {
                depth += 1;
                input = &input[1..];

                Token::Bracket {
                    kind: BracketKind::Opening,
                    location,
                }
            }
            // closing bracket
            (b']', location) if depth > 0 => {
                depth -= 1;
                if version.is_v1() {
                    // If the depth is zero, then we are no longer in a nested component. As such we
                    // have not seen the component name. If the depth is not zero, then we have just
                    // completed a nested format description or nested component. In either case,
                    // the nested component name comes before this, so we have seen it.
                    nested_component_name_seen = depth != 0;
                }
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

                while let Some((_, location)) = iter.next_if(|&(&byte, _)| {
                    !((version.is_at_least_v2() && byte == b'\\') || byte == b'[')
                }) {
                    end_location = location;
                    bytes += 1;
                }

                // Safety: A string was passed to this function, and only UTF-8 has been consumed,
                // leaving behind a string known to begin at a character boundary.
                let value = unsafe { str::from_utf8_unchecked(&input[..bytes]) };
                input = &input[bytes..];

                Token::Literal(value.spanned(start_location.to(end_location)))
            }
            // component part
            (byte, start_location) => {
                let mut bytes = 1;
                let mut end_location = start_location;
                let is_whitespace = byte.is_ascii_whitespace();

                while let Some((_, location)) = iter.next_if(|&(byte, _)| {
                    !matches!(byte, b'\\' | b'[' | b']')
                        && is_whitespace == byte.is_ascii_whitespace()
                }) {
                    end_location = location;
                    bytes += 1;
                }

                // Safety: A string was passed to this function, and only UTF-8 has been consumed,
                // leaving behind a string known to begin at a character boundary.
                let value = unsafe { str::from_utf8_unchecked(&input[..bytes]) };
                input = &input[bytes..];

                // If what we just consumed is not whitespace, then it is either the component name
                // or a modifier (which comes after the component name). In either situation, we
                // have seen the component name, so we set the flag. This is only relevant for v1
                // format descriptions.
                if version.is_v1() && !is_whitespace {
                    nested_component_name_seen = true;
                }

                Token::ComponentPart {
                    kind: if is_whitespace {
                        ComponentKind::Whitespace
                    } else {
                        ComponentKind::NotWhitespace
                    },
                    value: value.spanned(start_location.to(end_location)),
                }
            }
        }))
    });

    Lexed {
        iter: iter.peekable(),
    }
}
