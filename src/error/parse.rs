//! Error that occurred at some stage of parsing

use crate::error::{IntermediateParse, TryFromParsed};
use core::fmt;

/// An error that occurred at some stage of parsing.
#[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parse {
    #[allow(clippy::missing_docs_in_private_items)]
    TryFromParsed(TryFromParsed),
    #[allow(clippy::missing_docs_in_private_items)]
    IntermediateParse(IntermediateParse),
}

impl fmt::Display for Parse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TryFromParsed(err) => err.fmt(f),
            Self::IntermediateParse(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for Parse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TryFromParsed(err) => Some(err),
            Self::IntermediateParse(err) => Some(err),
        }
    }
}

impl From<TryFromParsed> for Parse {
    fn from(err: TryFromParsed) -> Self {
        Self::TryFromParsed(err)
    }
}

impl From<IntermediateParse> for Parse {
    fn from(err: IntermediateParse) -> Self {
        Self::IntermediateParse(err)
    }
}

impl From<Parse> for crate::Error {
    fn from(err: Parse) -> Self {
        match err {
            Parse::TryFromParsed(err) => Self::TryFromParsed(err),
            Parse::IntermediateParse(err) => Self::IntermediateParse(err),
        }
    }
}
