use crate::format::ParseError;
#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact
/// error returned. `Result<_, time::Error>` will work in these situations.
// Boxing the `ComponentRangeError` reduces the size of `Error` from 72 bytes to
// 16.
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConversionRange(ConversionRangeError),
    ComponentRange(Box<ComponentRangeError>),
    Parse(ParseError),
}

impl fmt::Display for Error {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversionRange(e) => e.fmt(f),
            Self::ComponentRange(e) => e.fmt(f),
            Self::Parse(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRangeError;

impl fmt::Display for ConversionRangeError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionRangeError {}

impl From<ConversionRangeError> for Error {
    #[inline(always)]
    fn from(original: ConversionRangeError) -> Self {
        Self::ConversionRange(original)
    }
}

/// An error type indicating that a component provided to a method was out of
/// range, causing a failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need
// for a type parameter.
#[allow(missing_copy_implementations)] // Non-copy fields may be added.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentRangeError {
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

impl fmt::Display for ComponentRangeError {
    #[inline(always)]
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

impl From<ComponentRangeError> for Error {
    #[inline(always)]
    fn from(original: ComponentRangeError) -> Self {
        Self::ComponentRange(Box::new(original))
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ComponentRangeError {}

impl From<ParseError> for Error {
    #[inline(always)]
    fn from(original: ParseError) -> Self {
        Self::Parse(original)
    }
}
