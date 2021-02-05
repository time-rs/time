//! Error formatting a struct

use core::fmt;

/// An error occurred when formatting.
#[non_exhaustive]
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    /// The type being formatted does not contain sufficient information to format a component.
    #[non_exhaustive]
    InsufficientTypeInformation,
    /// A value of `core::fmt::Error` was returned internally.
    StdFmt,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientTypeInformation { .. } => f.write_str(
                "The type being formatted does not contain sufficient information to format a \
                 component.",
            ),
            Self::StdFmt => core::fmt::Error.fmt(f),
        }
    }
}

impl From<fmt::Error> for Format {
    fn from(_: fmt::Error) -> Self {
        Self::StdFmt
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for Format {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InsufficientTypeInformation { .. } => None,
            Self::StdFmt => Some(&core::fmt::Error),
        }
    }
}

impl From<Format> for crate::Error {
    fn from(original: Format) -> Self {
        Self::Format(original)
    }
}
