//! The [`Format`] struct and its implementations.

/// Various well-known formats, along with the possibility for a custom format
/// (provided either at compile-time or runtime).
#[allow(clippy::missing_docs_in_private_items)] // variants
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Format<'a> {
    Rfc3339,
    Custom(&'a str),
}

impl<'a> From<&'a str> for Format<'a> {
    fn from(s: &'a str) -> Self {
        Format::Custom(s)
    }
}
