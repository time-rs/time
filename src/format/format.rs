//! The `Format` struct and its implementations.

use crate::internal_prelude::*;

/// Various well-known formats, along with the possibility for a custom format
/// (provided either at compile-time or runtime).
#[allow(clippy::missing_docs_in_private_items)] // variants
#[cfg_attr(supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Format<'a> {
    Rfc3339,
    Custom(Cow<'a, str>),
    #[cfg(not(supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl<'a, T> From<T> for Format<'a>
where
    T: Into<Cow<'a, str>>,
{
    #[inline]
    fn from(s: T) -> Self {
        Format::Custom(s.into())
    }
}
