//! Description of how types should be formatted and parsed.
//!
//! The formatted value will be output to the provided writer. Format descriptions can be
//! [well-known](crate::format_description::well_known) or obtained by using the
//! [`format_description!`](crate::macros::format_description) macro, the
//! [`format_description::parse`](crate::format_description::parse()) function.

mod component;
pub mod modifier;
#[cfg(feature = "alloc")]
pub(crate) mod parse;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use core::fmt;

pub use self::component::Component;
#[cfg(feature = "alloc")]
pub use self::parse::parse;
use crate::error;

/// Helper methods.
#[cfg(feature = "alloc")]
mod helper {
    /// Consume all leading whitespace, advancing `index` as appropriate.
    #[must_use = "This does not modify the original slice."]
    pub(crate) fn consume_whitespace<'a>(bytes: &'a [u8], index: &mut usize) -> &'a [u8] {
        let first_non_whitespace = bytes
            .iter()
            .position(|c| !c.is_ascii_whitespace())
            .unwrap_or(bytes.len());
        *index += first_non_whitespace;
        &bytes[first_non_whitespace..]
    }
}

/// Well-known formats, typically standards.
pub mod well_known {
    pub mod iso8601;
    mod rfc2822;
    mod rfc3339;

    #[doc(inline)]
    pub use iso8601::Iso8601;
    pub use rfc2822::Rfc2822;
    pub use rfc3339::Rfc3339;
}

/// A complete description of how to format and parse a type.
#[non_exhaustive]
#[cfg_attr(not(feature = "alloc"), derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub enum FormatItem<'a> {
    /// Bytes that are formatted as-is.
    ///
    /// **Note**: If you call the `format` method that returns a `String`, these bytes will be
    /// passed through `String::from_utf8_lossy`.
    Literal(&'a [u8]),
    /// A minimal representation of a single non-literal item.
    Component(Component),
    /// A series of literals or components that collectively form a partial or complete
    /// description.
    Compound(&'a [Self]),
    /// A `FormatItem` that may or may not be present when parsing. If parsing fails, there will be
    /// no effect on the resulting `struct`.
    ///
    /// This variant has no effect on formatting, as the value is guaranteed to be present.
    Optional(&'a Self),
    /// A series of `FormatItem`s where, when parsing, the first successful parse is used. When
    /// formatting, the first element of the slice is used.  An empty slice is a no-op when
    /// formatting or parsing.
    First(&'a [Self]),
}

#[cfg(feature = "alloc")]
impl fmt::Debug for FormatItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatItem::Literal(literal) => f.write_str(&String::from_utf8_lossy(literal)),
            FormatItem::Component(component) => component.fmt(f),
            FormatItem::Compound(compound) => compound.fmt(f),
            FormatItem::Optional(item) => f.debug_tuple("Optional").field(item).finish(),
            FormatItem::First(items) => f.debug_tuple("First").field(items).finish(),
        }
    }
}

impl From<Component> for FormatItem<'_> {
    fn from(component: Component) -> Self {
        Self::Component(component)
    }
}

impl TryFrom<FormatItem<'_>> for Component {
    type Error = error::DifferentVariant;

    fn try_from(value: FormatItem<'_>) -> Result<Self, Self::Error> {
        match value {
            FormatItem::Component(component) => Ok(component),
            _ => Err(error::DifferentVariant),
        }
    }
}

impl<'a> From<&'a [FormatItem<'_>]> for FormatItem<'a> {
    fn from(items: &'a [FormatItem<'_>]) -> FormatItem<'a> {
        FormatItem::Compound(items)
    }
}

impl<'a> TryFrom<FormatItem<'a>> for &[FormatItem<'a>] {
    type Error = error::DifferentVariant;

    fn try_from(value: FormatItem<'a>) -> Result<Self, Self::Error> {
        match value {
            FormatItem::Compound(items) => Ok(items),
            _ => Err(error::DifferentVariant),
        }
    }
}

impl PartialEq<Component> for FormatItem<'_> {
    fn eq(&self, rhs: &Component) -> bool {
        matches!(self, FormatItem::Component(component) if component == rhs)
    }
}

impl PartialEq<FormatItem<'_>> for Component {
    fn eq(&self, rhs: &FormatItem<'_>) -> bool {
        rhs == self
    }
}

impl PartialEq<&[FormatItem<'_>]> for FormatItem<'_> {
    fn eq(&self, rhs: &&[FormatItem<'_>]) -> bool {
        matches!(self, FormatItem::Compound(compound) if compound == rhs)
    }
}

impl PartialEq<FormatItem<'_>> for &[FormatItem<'_>] {
    fn eq(&self, rhs: &FormatItem<'_>) -> bool {
        rhs == self
    }
}
