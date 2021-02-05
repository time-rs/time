//! Error parsing a format description

use core::fmt;

/// An error that occurred during parsing.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parse {
    /// A string literal was not what was expected.
    #[non_exhaustive]
    InvalidLiteral,
    /// A dynamic component was not valid.
    InvalidComponent(&'static str),
}

impl fmt::Display for Parse {
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
impl std::error::Error for Parse {}

impl From<Parse> for crate::Error {
    fn from(original: Parse) -> Self {
        Self::Parse(original)
    }
}
