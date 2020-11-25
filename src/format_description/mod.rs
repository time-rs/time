//! Description of how types should be formatted and parsed.

mod component;
pub(crate) mod error;
pub mod modifier;
#[cfg(feature = "alloc")]
pub(crate) mod parse;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
pub use component::Component;
#[cfg(feature = "alloc")]
pub use error::InvalidFormatDescription;
#[cfg(feature = "alloc")]
pub use parse::parse;

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

/// A complete description of how to format and parse a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatDescription<'a> {
    /// A string that is formatted as-is.
    Literal(&'a str),
    /// A minimal representation of a single non-literal item.
    Component(Component),
    /// A series of literals or components that collectively form a partial or
    /// complete description.
    ///
    /// Note that this is a reference to a slice, such that either a [`Vec`] or
    /// statically known list can be provided.
    Compound(&'a [Self]),
}

impl From<Component> for FormatDescription<'_> {
    fn from(component: Component) -> Self {
        FormatDescription::Component(component)
    }
}

impl<'a> From<&'a [FormatDescription<'_>]> for FormatDescription<'a> {
    fn from(x: &'a [FormatDescription<'_>]) -> Self {
        FormatDescription::Compound(x)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl<'a> From<&'a Vec<FormatDescription<'_>>> for FormatDescription<'a> {
    fn from(x: &'a Vec<FormatDescription<'_>>) -> Self {
        FormatDescription::Compound(x)
    }
}
