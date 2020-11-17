#[cfg(feature = "alloc")]
use crate::formatting::error::InvalidFormatDescription;
use crate::formatting::format;
use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact
/// error returned. `Result<_, time::Error>` will work in these situations.
#[allow(missing_copy_implementations, variant_size_differences)]
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConversionRange,
    ComponentRange(ComponentRange),
    IndeterminateOffset,
    Format(format::Error),
    #[cfg(feature = "alloc")]
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
    InvalidFormatDescription(InvalidFormatDescription),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConversionRange => ConversionRange.fmt(f),
            Error::ComponentRange(e) => e.fmt(f),
            Error::IndeterminateOffset => IndeterminateOffset.fmt(f),
            Error::Format(e) => e.fmt(f),
            #[cfg(feature = "alloc")]
            Error::InvalidFormatDescription(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ConversionRange => Some(&ConversionRange),
            Error::ComponentRange(err) => Some(err),
            Error::IndeterminateOffset => Some(&IndeterminateOffset),
            Error::Format(err) => Some(err),
            Error::InvalidFormatDescription(err) => Some(err),
        }
    }
}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRange;

impl fmt::Display for ConversionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for ConversionRange {}

impl From<ConversionRange> for Error {
    fn from(_: ConversionRange) -> Self {
        Self::ConversionRange
    }
}

/// An error type indicating that a component provided to a method was out of
/// range, causing a failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need
// for a type parameter.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentRange {
    /// Name of the component.
    pub name: &'static str,
    /// Minimum allowed value, inclusive.
    pub minimum: i64,
    /// Maximum allowed value, inclusive.
    pub maximum: i64,
    /// Value that was provided.
    pub value: i64,
    /// The minimum and/or maximum value is conditional on the value of other
    /// parameters.
    pub conditional_range: bool,
}

impl fmt::Display for ComponentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be in the range {}..={}",
            self.name, self.minimum, self.maximum
        )?;

        if self.conditional_range {
            write!(f, ", given values of other parameters")?;
        }

        Ok(())
    }
}

impl From<ComponentRange> for Error {
    fn from(original: ComponentRange) -> Self {
        Self::ComponentRange(original)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for ComponentRange {}

/// The system's UTC offset could not be determined at the given datetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateOffset;

impl fmt::Display for IndeterminateOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The system's UTC offset could not be determined")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl std::error::Error for IndeterminateOffset {}

impl From<IndeterminateOffset> for Error {
    fn from(_: IndeterminateOffset) -> Self {
        Self::IndeterminateOffset
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl From<InvalidFormatDescription> for Error {
    fn from(original: InvalidFormatDescription) -> Self {
        Error::InvalidFormatDescription(original)
    }
}

impl From<format::Error> for Error {
    fn from(original: format::Error) -> Self {
        Error::Format(original)
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use super::*;
    use std::error::Error as ParseError;

    #[test]
    fn indeterminate_offset() {
        assert_eq!(
            IndeterminateOffset.to_string(),
            Error::IndeterminateOffset.to_string()
        );
        assert!(match Error::from(IndeterminateOffset).source() {
            Some(error) => error.is::<IndeterminateOffset>(),
            None => false,
        });
    }
}
