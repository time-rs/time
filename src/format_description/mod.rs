//! Description of how types should be formatted and parsed.

mod component;
pub mod modifier;
#[cfg(feature = "alloc")]
pub(crate) mod parse;

pub use self::component::Component;
#[cfg(feature = "alloc")]
pub use self::parse::parse;

/// Helper methods.
#[cfg(feature = "alloc")]
mod helper {
    /// Consume all leading whitespace, advancing `index` as appropriate.
    #[must_use = "This does not modify the original string."]
    pub(crate) fn consume_whitespace<'a>(s: &'a str, index: &mut usize) -> &'a str {
        *index += s.len();
        let s = s.trim_start();
        *index -= s.len();
        s
    }
}

/// Well-known formats, typically RFCs.
pub mod well_known {
    /// The format described in [RFC 3339](https://tools.ietf.org/html/rfc3339#section-5.6).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Rfc3339;
}

/// A complete description of how to format and parse a type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatItem<'a> {
    /// A string that is formatted as-is.
    Literal(&'a str),
    /// A minimal representation of a single non-literal item.
    Component(Component),
    /// A series of literals or components that collectively form a partial or complete
    /// description.
    Compound(&'a [Self]),
}
