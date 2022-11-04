//! Lexer for parsing format descriptions.

use core::iter;

use super::{Location, Spanned, SpannedValue};

/// A token emitted by the lexer. There is no semantic meaning at this stage.
pub(super) enum Token<'a> {
    /// A literal string, formatted and parsed as-is.
    Literal(Spanned<&'a [u8]>),
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
        value: Spanned<&'a [u8]>,
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
    #[allow(clippy::missing_docs_in_private_items)]
    Whitespace,
    #[allow(clippy::missing_docs_in_private_items)]
    NotWhitespace,
}

/// The state of the lexer as it relates to lexing the start of a nested description.
enum NestedState {
    /// The most recent tokens do not indicate the start of a nested description.
    Inconsequential,
    /// We are after an opening bracket, but have not yet seen a component name.
    AfterOpeningBracket,
    /// We are after an opening bracket and have seen a component name, but have not yet seen the
    /// whitespace that follows the component name.
    AfterNestedKeyword,
    /// We have seen an opening bracket, the component name, and the subsequent whitespace. The
    /// next token should be a bracket.
    AfterWhitespaceAfterNestedKeyword,
    /// We have seen an opening bracket, the component name, the subsequent whitespace, and the
    /// bracket that starts the nested description.
    AfterStartOfNestedDescriptionBracket,
    /// We have just seen the closing bracket of a nested description.
    AfterEndOfNestedDescription,
    /// We have just seen the closing bracket of a nested description, and the whitespace that
    /// follows it.
    AfterWhiteSpaceAfterEndOfNestedDescription,
}

impl NestedState {
    /// Whether the current state indicates that the depth should be incremented.
    const fn should_increment_depth(&self) -> bool {
        matches!(
            self,
            Self::AfterWhitespaceAfterNestedKeyword
                | Self::AfterStartOfNestedDescriptionBracket
                | Self::AfterEndOfNestedDescription
                | Self::AfterWhiteSpaceAfterEndOfNestedDescription
        )
    }
}

/// Attach [`Location`] information to each byte in the iterator.
fn attach_location(iter: impl Iterator<Item = u8>) -> impl Iterator<Item = (u8, Location)> {
    let mut byte_pos = 0;

    iter.map(move |byte| {
        let location = Location { byte: byte_pos };
        byte_pos += 1;
        (byte, location)
    })
}

/// Parse the string into a series of [`Token`]s.
pub(super) fn lex(mut input: &[u8]) -> impl Iterator<Item = Token<'_>> {
    let mut depth: u8 = 0;
    let mut iter = attach_location(input.iter().copied()).peekable();
    let mut second_bracket_location = None;
    // Used to keep track of whether we might be starting a nested description. Affects the behavior
    // of `depth`.
    let mut nested_state = NestedState::Inconsequential;

    iter::from_fn(move || {
        // There is a flag set to emit the second half of an escaped bracket pair.
        if let Some(location) = second_bracket_location.take() {
            if nested_state.should_increment_depth() {
                depth += 1;
            }
            nested_state = NestedState::AfterOpeningBracket;

            return Some(Token::Bracket {
                kind: BracketKind::Opening,
                location,
            });
        }

        Some(match iter.next()? {
            (b'[', location) => {
                if let Some((_, second_location)) = iter.next_if(|&(byte, _)| byte == b'[') {
                    // Escaped bracket. This only increments the depth if we are starting a nested
                    // description. Otherwise it will eventually be interpreted as a literal.
                    second_bracket_location = Some(second_location);
                    if nested_state.should_increment_depth() {
                        depth += 1;
                        nested_state = NestedState::AfterStartOfNestedDescriptionBracket;
                    } else {
                        nested_state = NestedState::AfterOpeningBracket;
                    }
                    input = &input[2..];
                } else {
                    // opening bracket
                    depth += 1;
                    input = &input[1..];
                    nested_state = NestedState::AfterOpeningBracket;
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
                nested_state = NestedState::AfterEndOfNestedDescription;

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
                nested_state = NestedState::Inconsequential;

                Token::Literal(value.spanned(start_location.to(end_location)))
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

                nested_state = match (&nested_state, is_whitespace) {
                    (NestedState::AfterOpeningBracket, _) => NestedState::AfterNestedKeyword,
                    (NestedState::AfterNestedKeyword, true) => {
                        NestedState::AfterWhitespaceAfterNestedKeyword
                    }
                    (NestedState::AfterEndOfNestedDescription, true) => {
                        NestedState::AfterWhiteSpaceAfterEndOfNestedDescription
                    }
                    _ => NestedState::Inconsequential,
                };

                Token::ComponentPart {
                    kind: if is_whitespace {
                        ComponentKind::Whitespace
                    } else {
                        ComponentKind::NotWhitespace
                    },
                    value: value.spanned(start_location.to(end_location)),
                }
            }
        })
    })
}
