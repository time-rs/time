//! Description of how types should be formatted and parsed.

mod component;
pub mod modifier;
#[cfg(feature = "alloc")]
pub(crate) mod parse;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
pub use component::Component;

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
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatDescription<'a> {
    /// A string that is formatted as-is.
    Literal(&'a str),
    /// A minimal representation of a single non-literal item.
    Component(Component),
    /// A series of literals or components that collectively form a partial or complete
    /// description.
    ///
    /// Note that this is a reference to a slice, such that a statically known list can be
    /// provided.
    BorrowedCompound(&'a [Self]),
    /// A series of literals or components that collectively form a partial or complete
    /// description.
    // It's necessary to have a separate variant rather than use `Cow`, as features should be
    // strictly additive; a `Cow` cannot be used in non-alloc environments.
    #[cfg(feature = "alloc")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
    OwnedCompound(Vec<Self>),
}
