//! Error converting a [`Parsed`](crate::parsing::Parsed) struct to another type

use crate::error;
use core::fmt;

/// An error that occurred when converting a [`Parsed`](crate::parsing::Parsed) to another type.
#[non_exhaustive]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromParsed {
    /// The [`Parsed`](crate::parsing::Parsed) did not include enough information to construct the
    /// type.
    InsufficientInformation,
    /// Some component contained an invalid value for the type.
    ComponentRange(error::ComponentRange),
}

impl fmt::Display for FromParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientInformation => f.write_str(
                "the `Parsed` struct did not include enough information to construct the type",
            ),
            Self::ComponentRange(err) => err.fmt(f),
        }
    }
}

impl From<error::ComponentRange> for FromParsed {
    fn from(v: error::ComponentRange) -> Self {
        Self::ComponentRange(v)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for FromParsed {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InsufficientInformation => None,
            Self::ComponentRange(err) => Some(err),
        }
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
impl From<FromParsed> for crate::Error {
    fn from(original: FromParsed) -> Self {
        Self::FromParsed(original)
    }
}
