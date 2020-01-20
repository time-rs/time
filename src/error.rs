#[cfg(not(feature = "std"))]
use crate::alloc_prelude::*;
use crate::format::ParseError;
use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact
/// error returned. `Result<_, time::Error>` will work in these situations.
// Boxing the `ComponentRangeError` reduces the size of `Error` from 72 bytes to
// 16.
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[rustversion::attr(since(1.40), non_exhaustive)]
#[rustversion::attr(
    before(1.40),
    doc("This enum is non-exhaustive. Additional variants may be added at any time.")
)]
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
            Error::ConversionRange(e) => e.fmt(f),
            Error::ComponentRange(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
#[allow(clippy::missing_docs_in_private_items)]
#[rustversion::attr(since(1.40), non_exhaustive)]
#[rustversion::attr(
    before(1.40),
    doc("This struct is non-exhaustive. Additional variants may be added at any time.")
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRangeError {
    #[allow(clippy::missing_docs_in_private_items)]
    nonexhaustive: (),
}

impl ConversionRangeError {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) const fn new() -> Self {
        Self { nonexhaustive: () }
    }
}

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
        Error::ConversionRange(original)
    }
}

/// An error type indicating that a component provided to a method was out of
/// range, causing a failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need
// for a type parameter.
#[rustversion::attr(since(1.40), non_exhaustive)]
#[rustversion::attr(
    before(1.40),
    doc("This struct is non-exhaustive. Additional fields may be added at any time.")
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        Error::ComponentRange(Box::new(original))
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ComponentRangeError {}

impl From<ParseError> for Error {
    #[inline(always)]
    fn from(original: ParseError) -> Self {
        Error::Parse(original)
    }
}
