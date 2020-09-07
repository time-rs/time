pub use crate::format::parse::Error as Parse;
#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact
/// error returned. `Result<_, time::Error>` will work in these situations.
// Boxing the `ComponentRange` reduces the size of `Error` from 72 bytes to 16.
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[cfg_attr(__time_02_supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConversionRange(ConversionRange),
    ComponentRange(Box<ComponentRange>),
    Parse(Parse),
    IndeterminateOffset(IndeterminateOffset),
    #[cfg(not(__time_02_supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConversionRange(e) => e.fmt(f),
            Error::ComponentRange(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
            Error::IndeterminateOffset(e) => e.fmt(f),
            #[cfg(not(__time_02_supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ConversionRange(err) => Some(err),
            Error::ComponentRange(box_err) => Some(box_err.as_ref()),
            Error::Parse(err) => Some(err),
            Error::IndeterminateOffset(err) => Some(err),
            #[cfg(not(__time_02_supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRange {
    #[allow(clippy::missing_docs_in_private_items)]
    __non_exhaustive: (),
}

impl ConversionRange {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) const fn new() -> Self {
        Self {
            __non_exhaustive: (),
        }
    }
}

impl fmt::Display for ConversionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionRange {}

impl From<ConversionRange> for Error {
    fn from(original: ConversionRange) -> Self {
        Error::ConversionRange(original)
    }
}

/// An error type indicating that a component provided to a method was out of
/// range, causing a failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need
// for a type parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentRange {
    /// Name of the component.
    pub(crate) name: &'static str,
    /// Minimum allowed value, inclusive.
    pub(crate) minimum: i64,
    /// Maximum allowed value, inclusive.
    pub(crate) maximum: i64,
    /// Value that was provided.
    pub(crate) value: i64,
    /// The minimum and/or maximum is only valid with the following values.
    pub(crate) given: Vec<(&'static str, i64)>,
}

impl fmt::Display for ComponentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be in the range {}..={}",
            self.name, self.minimum, self.maximum
        )?;

        let mut iter = self.given.iter();
        if let Some((name, value)) = iter.next() {
            write!(f, " given {}={}", name, value)?;
            for (name, value) in iter {
                write!(f, ", {}={}", name, value)?;
            }
        }

        write!(f, " (was {})", self.value)
    }
}

impl From<ComponentRange> for Error {
    fn from(original: ComponentRange) -> Self {
        Error::ComponentRange(Box::new(original))
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ComponentRange {}

impl From<Parse> for Error {
    fn from(original: Parse) -> Self {
        Error::Parse(original)
    }
}

/// The system's UTC offset could not be determined at the given datetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateOffset {
    #[allow(clippy::missing_docs_in_private_items)]
    __non_exhaustive: (),
}

impl IndeterminateOffset {
    #[allow(clippy::missing_docs_in_private_items, dead_code)]
    pub(crate) const fn new() -> Self {
        Self {
            __non_exhaustive: (),
        }
    }
}

impl fmt::Display for IndeterminateOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The system's UTC offset could not be determined")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IndeterminateOffset {}

impl From<IndeterminateOffset> for Error {
    fn from(original: IndeterminateOffset) -> Self {
        Error::IndeterminateOffset(original)
    }
}
