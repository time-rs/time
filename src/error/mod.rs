mod component_range;
mod conversion_range;
#[cfg(feature = "formatting")]
mod format;
#[cfg(feature = "local-offset")]
mod indeterminate_offset;
#[cfg(feature = "parsing")]
mod intermediate_parse;
#[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
mod invalid_format_description;
#[cfg(feature = "parsing")]
mod parse;
#[cfg(feature = "parsing")]
mod try_from_parsed;

pub use component_range::ComponentRange;
pub use conversion_range::ConversionRange;
use core::fmt;
#[cfg(feature = "formatting")]
pub use format::Format;
#[cfg(feature = "local-offset")]
pub use indeterminate_offset::IndeterminateOffset;
#[cfg(feature = "parsing")]
pub use intermediate_parse::IntermediateParse;
#[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
pub use invalid_format_description::InvalidFormatDescription;
#[cfg(feature = "parsing")]
pub use parse::Parse;
#[cfg(feature = "parsing")]
pub use try_from_parsed::TryFromParsed;

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
    #[cfg(feature = "local-offset")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "local-offset")))]
    IndeterminateOffset,
    #[cfg(feature = "formatting")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
    Format(Format),
    #[cfg(feature = "parsing")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
    IntermediateParse(IntermediateParse),
    #[cfg(feature = "parsing")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "parsing")))]
    TryFromParsed(TryFromParsed),
    #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
    #[cfg_attr(
        __time_03_docs,
        doc(cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc")))
    )]
    InvalidFormatDescription(InvalidFormatDescription),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversionRange => ConversionRange.fmt(f),
            Self::ComponentRange(e) => e.fmt(f),
            #[cfg(feature = "local-offset")]
            Self::IndeterminateOffset => IndeterminateOffset.fmt(f),
            #[cfg(feature = "formatting")]
            Self::Format(e) => e.fmt(f),
            #[cfg(feature = "parsing")]
            Self::IntermediateParse(e) => e.fmt(f),
            #[cfg(feature = "parsing")]
            Self::TryFromParsed(e) => e.fmt(f),
            #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
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
            #[cfg(feature = "local-offset")]
            Self::IndeterminateOffset => Some(&IndeterminateOffset),
            #[cfg(feature = "formatting")]
            Self::Format(err) => Some(err),
            #[cfg(feature = "parsing")]
            Self::IntermediateParse(err) => Some(err),
            #[cfg(feature = "parsing")]
            Self::TryFromParsed(err) => Some(err),
            #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
            Self::InvalidFormatDescription(err) => Some(err),
        }
    }
}
