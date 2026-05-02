//! Lexer for parsing format descriptions.

use core::iter::FusedIterator;

use super::{Error, Location, Spanned, SpannedValue, unused};
use crate::error::InvalidFormatDescription;
use crate::hint;
use crate::internal_macros::const_try_opt;

/// An iterator over the lexed tokens.
pub(super) struct Lexer<'input, const VERSION: u8> {
    input: &'input [u8],
    depth: u8,
    byte_pos: u32,
    nested_component_name_seen: bool,
}

pub(super) enum PeekKind {
    Literal,
    OpeningBracket,
    ClosingBracket,
    ComponentWhitespace,
    ComponentNotWhitespace,
}

impl<'input, const VERSION: u8> Lexer<'input, VERSION> {
    #[inline]
    fn advance(&mut self, bytes: u32) {
        self.input = &self.input[bytes as usize..];
        self.byte_pos += bytes;
    }

    #[inline]
    fn peek_kind(&self) -> Option<PeekKind> {
        Some(match *self.input.first()? {
            b'\\' if version!(2..) && self.depth == 0 => PeekKind::Literal,
            b'\\' if version!(2..) => PeekKind::ComponentNotWhitespace,
            b'[' if version!(1)
                && !self.nested_component_name_seen
                && self.input.get(1) == Some(&b'[') =>
            {
                PeekKind::Literal
            }
            b'[' => PeekKind::OpeningBracket,
            b']' if self.depth > 0 => PeekKind::ClosingBracket,
            _ if self.depth == 0 => PeekKind::Literal,
            byte if byte.is_ascii_whitespace() => PeekKind::ComponentWhitespace,
            _ => PeekKind::ComponentNotWhitespace,
        })
    }

    /// Consume the next token if it is whitespace.
    #[inline]
    pub(super) fn next_if_whitespace(&mut self) -> Option<Spanned<&'input str>> {
        if matches!(self.peek_kind(), Some(PeekKind::ComponentWhitespace))
            && let Some(Ok(Token::ComponentPart {
                kind: ComponentKind::Whitespace,
                value,
            })) = self.next()
        {
            Some(value)
        } else {
            None
        }
    }

    /// Consume the next token if it is a component item that is not whitespace.
    #[inline]
    pub(super) fn next_if_not_whitespace(&mut self) -> Option<Spanned<&'input str>> {
        if matches!(self.peek_kind(), Some(PeekKind::ComponentNotWhitespace))
            && let Some(Ok(Token::ComponentPart {
                kind: ComponentKind::NotWhitespace,
                value,
            })) = self.next()
        {
            Some(value)
        } else {
            None
        }
    }

    /// Consume the next token if it is an opening bracket.
    #[inline]
    pub(super) fn next_if_opening_bracket(&mut self) -> Option<Location> {
        if matches!(self.peek_kind(), Some(PeekKind::OpeningBracket))
            && let Some(Ok(Token::Bracket {
                kind: BracketKind::Opening,
                location,
            })) = self.next()
        {
            Some(location)
        } else {
            None
        }
    }

    /// Peek at the next token if it is a closing bracket.
    #[inline]
    pub(super) fn peek_closing_bracket(&self) -> Option<Location> {
        match self.peek_kind() {
            Some(PeekKind::ClosingBracket) => Some(Location {
                byte: self.byte_pos,
            }),
            _ => None,
        }
    }

    /// Consume the next token if it is a closing bracket.
    #[inline]
    pub(super) fn next_if_closing_bracket(&mut self) -> Option<Location> {
        if matches!(self.peek_kind(), Some(PeekKind::ClosingBracket))
            && let Some(Ok(Token::Bracket {
                kind: BracketKind::Closing,
                location,
            })) = self.next()
        {
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

impl<'input, const VERSION: u8> Iterator for Lexer<'input, VERSION> {
    type Item = Result<Token<'input>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        assert_version!();

        let byte = const_try_opt!(self.input.first());
        let location = Location {
            byte: self.byte_pos,
        };

        Some(Ok(match byte {
            // possible escape sequence
            b'\\' if version!(2..) => {
                let backslash_loc = location;
                match self.input.get(1) {
                    Some(b'\\' | b'[' | b']') => {
                        let char_loc = Location {
                            byte: self.byte_pos + 1,
                        };
                        // The escaped character is emitted as-is.
                        // Safety: We know that this is either a left bracket, right bracket, or
                        // backslash.
                        let char = unsafe { str::from_utf8_unchecked(&self.input[1..2]) };
                        self.advance(2);
                        if self.depth == 0 {
                            Token::Literal(char.spanned(backslash_loc.to(char_loc)))
                        } else {
                            Token::ComponentPart {
                                kind: ComponentKind::NotWhitespace,
                                value: char.spanned(backslash_loc.to(char_loc)),
                            }
                        }
                    }
                    Some(_) => {
                        let loc = Location {
                            byte: self.byte_pos + 1,
                        };
                        return Some(Err(Error {
                            _inner: unused(loc.error("invalid escape sequence")),
                            public: InvalidFormatDescription::Expected {
                                what: "valid escape sequence",
                                index: loc.byte as usize,
                            },
                        }));
                    }
                    None => {
                        return Some(Err(Error {
                            _inner: unused(backslash_loc.error("unexpected end of input")),
                            public: InvalidFormatDescription::Expected {
                                what: "valid escape sequence",
                                index: backslash_loc.byte as usize,
                            },
                        }));
                    }
                }
            }
            // If we have no seen a nested component name and the following character is `[`, then
            // we know that this is an escaped bracket. If either is not the case, then it's an
            // opening bracket handled by the following branch.
            b'[' if version!(1)
                && !self.nested_component_name_seen
                && self.input.get(1) == Some(&b'[') =>
            {
                let second_location = Location {
                    byte: self.byte_pos + 1,
                };
                self.advance(2);
                Token::Literal("[".spanned(location.to(second_location)))
            }
            // opening bracket
            b'[' => {
                match self.depth.checked_add(1) {
                    Some(depth) => self.depth = depth,
                    None => {
                        hint::cold_path();
                        return Some(Err(Error {
                            _inner: unused(location.error("too much nesting")),
                            public: InvalidFormatDescription::NotSupported {
                                what: "highly-nested format description",
                                context: "",
                                index: location.byte as usize,
                            },
                        }));
                    }
                }
                self.advance(1);

                Token::Bracket {
                    kind: BracketKind::Opening,
                    location,
                }
            }
            // closing bracket
            b']' if self.depth > 0 => {
                self.depth -= 1;
                if version!(1) {
                    // If the depth is zero, then we are no longer in a nested component. As such we
                    // have not seen the component name. If the depth is not zero, then we have just
                    // completed a nested format description or nested component. In either case,
                    // the nested component name comes before this, so we have seen it.
                    self.nested_component_name_seen = self.depth != 0;
                }
                self.advance(1);

                Token::Bracket {
                    kind: BracketKind::Closing,
                    location,
                }
            }
            // literal
            _ if self.depth == 0 => {
                let mut bytes: u32 = 1;
                let mut end_location = location;
                while let Some(&next_byte) = self.input.get(bytes as usize) {
                    if (version!(2..) && next_byte == b'\\') || next_byte == b'[' {
                        break;
                    }
                    end_location = Location {
                        byte: self.byte_pos + bytes,
                    };
                    bytes += 1;
                }

                // Safety: A string was passed to this function, and only UTF-8 has been consumed,
                // leaving behind a string known to begin at a character boundary.
                let value = unsafe { str::from_utf8_unchecked(&self.input[..bytes as usize]) };
                self.advance(bytes);

                Token::Literal(value.spanned(location.to(end_location)))
            }
            // component part
            byte => {
                let mut bytes: u32 = 1;
                let mut end_location = location;
                let is_whitespace = byte.is_ascii_whitespace();

                while let Some(&next_byte) = self.input.get(bytes as usize) {
                    if matches!(next_byte, b'\\' | b'[' | b']')
                        || is_whitespace != next_byte.is_ascii_whitespace()
                    {
                        break;
                    }
                    end_location = Location {
                        byte: self.byte_pos + bytes,
                    };
                    bytes += 1;
                }

                // Safety: A string was passed to this function, and only UTF-8 has been consumed,
                // leaving behind a string known to begin at a character boundary.
                let value = unsafe { str::from_utf8_unchecked(&self.input[..bytes as usize]) };
                self.advance(bytes);

                // If what we just consumed is not whitespace, then it is either the component name
                // or a modifier (which comes after the component name). In either situation, we
                // have seen the component name, so we set the flag. This is only relevant for v1
                // format descriptions.
                if version!(1) && !is_whitespace {
                    self.nested_component_name_seen = true;
                }

                Token::ComponentPart {
                    kind: if is_whitespace {
                        ComponentKind::Whitespace
                    } else {
                        ComponentKind::NotWhitespace
                    },
                    value: value.spanned(location.to(end_location)),
                }
            }
        }))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            // We're guaranteed at least one token if there is any input.
            if self.input.is_empty() { 0 } else { 1 },
            // The maximum number of tokens occurs when everything is an escape sequence, which is
            // two bytes per token.
            Some(self.input.len() / 2),
        )
    }
}

impl<const VERSION: u8> FusedIterator for Lexer<'_, VERSION> {}

/// Parse the string into a series of [`Token`]s.
///
/// `VERSION` controls the version of the format description that is being parsed.
///
/// - When `VERSION` is 1, `[[` is the only escape sequence, resulting in a literal `[`. For the
///   start of a nested format description, a single `[` is used and is _never_ part of the escape
///   sequence. For example, `[optional [[day]]]` will lex successfully, ultimately resulting in a
///   component named `optional` with the nested component `day`.
/// - When `VERSION` is 2 or 3, all escape sequences begin with `\`. The only characters that may
///   currently follow are `\`, `[`, and `]`, all of which result in the literal character. All
///   other characters result in a lex error.
#[inline]
pub(super) const fn lex<const VERSION: u8>(input: &str) -> Lexer<'_, VERSION> {
    assert_version!();

    Lexer {
        input: input.as_bytes(),
        depth: 0,
        byte_pos: 0,
        nested_component_name_seen: false,
    }
}
