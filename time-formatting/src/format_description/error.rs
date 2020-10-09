//! Errors that can be returned when parsing a format description.

use core::fmt;

/// The format description provided was not valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidFormatDescription<'a> {
    /// There was a bracket pair that was opened but not closed.
    UnclosedOpeningBracket {
        /// The zero-based index of the opening bracket.
        index: usize,
    },
    /// A component name is not valid.
    InvalidComponentName {
        /// The name of the invalid component name.
        name: &'a str,
        /// The zero-based index the component name starts at.
        index: usize,
    },
    /// A modifier is not valid.
    InvalidModifier {
        /// The value of the invalid modifier.
        value: &'a str,
        /// The zero-based index the modifier starts at.
        index: usize,
    },
    /// A component name is missing.
    MissingComponentName {
        /// The zero-based index where the component name should start.
        index: usize,
    },
}

impl fmt::Display for InvalidFormatDescription<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InvalidFormatDescription::*;
        match self {
            UnclosedOpeningBracket { index } => {
                write!(f, "unclosed opening bracket at byte index {}", index)
            }
            InvalidComponentName { name, index } => write!(
                f,
                "invalid component name `{}` at byte index {}",
                name, index
            ),
            InvalidModifier { value, index } => {
                write!(f, "invalid modifier `{}` at byte index {}", value, index)
            }
            MissingComponentName { index } => {
                write!(f, "missing component name at byte index {}", index)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidFormatDescription<'_> {}
