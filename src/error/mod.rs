mod component_range;
mod conversion_range;
mod format;
mod from_parsed;
mod indeterminate_offset;
#[cfg(feature = "alloc")]
mod invalid_format_description;
mod parse;

pub use component_range::ComponentRange;
pub use conversion_range::ConversionRange;
pub use format::Format;
pub use from_parsed::FromParsed;
pub use indeterminate_offset::IndeterminateOffset;
#[cfg(feature = "alloc")]
pub use invalid_format_description::InvalidFormatDescription;
pub use parse::Parse;

use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact error returned.
/// `Result<_, time::Error>` will work in these situations.
#[allow(missing_copy_implementations, variant_size_differences)]
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConversionRange,
    ComponentRange(ComponentRange),
    IndeterminateOffset,
    Format(Format),
    Parse(Parse),
    FromParsed(FromParsed),
    #[cfg(feature = "alloc")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
    InvalidFormatDescription(InvalidFormatDescription),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversionRange => ConversionRange.fmt(f),
            Self::ComponentRange(e) => e.fmt(f),
            Self::IndeterminateOffset => IndeterminateOffset.fmt(f),
            Self::Format(e) => e.fmt(f),
            Self::Parse(e) => e.fmt(f),
            Self::FromParsed(e) => e.fmt(f),
            #[cfg(feature = "alloc")]
            Self::InvalidFormatDescription(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConversionRange => Some(&ConversionRange),
            Self::ComponentRange(err) => Some(err),
            Self::IndeterminateOffset => Some(&IndeterminateOffset),
            Self::Format(err) => Some(err),
            Self::Parse(err) => Some(err),
            Self::FromParsed(err) => Some(err),
            Self::InvalidFormatDescription(err) => Some(err),
        }
    }
}
