//! The `Format` struct and its implementations.

#[cfg(not(std))]
use crate::internal_prelude::*;

/// Various well-known formats, along with the possibility for a custom format
/// (provided either at compile-time or runtime).
#[allow(clippy::missing_docs_in_private_items)] // variants
#[cfg_attr(supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    Rfc3339,
    Custom(String),
    #[cfg(not(supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

// TODO We're only using `AsRef` for back-compatibility. In 0.3, switch this to
// `Into<Cow<'a, str>>`, which is both broader and avoids unnecessary clones.
// This will require the addition of a lifetime to the `Format` struct.

impl<T: AsRef<str>> From<T> for Format {
    #[inline]
    fn from(s: T) -> Self {
        Format::Custom(s.as_ref().to_owned())
    }
}
