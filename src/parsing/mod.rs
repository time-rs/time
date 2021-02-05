//! Parsing for various types.

mod combinator;
mod component;
mod parsed;

use core::fmt;
pub use parsed::Parsed;

/// An error that occurred during parsing.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// A string literal was not what was expected.
    InvalidLiteral,
    /// A dynamic component was not valid.
    InvalidComponent(&'static str),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLiteral => f.write_str("a character literal was not valid"),
            Self::InvalidComponent(name) => {
                write!(f, "the '{}' component could not be parsed", name)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

/// An error that occurred when converting a [`Parsed`] to another type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromParsedError {
    /// The [`Parsed`] did not include enough information to construct the type.
    InsufficientInformation,
    /// Some component contained an invalid value for the type.
    ComponentRange(crate::error::ComponentRange),
}

impl fmt::Display for FromParsedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientInformation => f.write_str(
                "the `Parsed` struct did not include enough information to construct the type",
            ),
            Self::ComponentRange(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromParsedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InsufficientInformation => None,
            Self::ComponentRange(err) => Some(err),
        }
    }
}
