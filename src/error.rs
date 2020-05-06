use crate::internal_prelude::*;
use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact
/// error returned. `Result<_, time::Error>` will work in these situations.
// Boxing the `ComponentRangeError` reduces the size of `Error` from 72 bytes to
// 16.
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[cfg_attr(supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConversionRange,
    #[cfg(not(no_alloc))]
    ComponentRange(Box<ComponentRangeError>),
    #[cfg(no_alloc)]
    ComponentRange(ComponentRangeError),
    Parse(ParseError),
    Format(FormatError),
    IndeterminateOffset,
    #[cfg(not(supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Error {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            e @ Error::ConversionRange | e @ Error::IndeterminateOffset => e.fmt(f),
            Error::ComponentRange(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
            Error::Format(e) => e.fmt(f),
            #[cfg(not(supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(std)]
impl std::error::Error for Error {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            err @ Error::ConversionRange | err @ Error::IndeterminateOffset => Some(err),
            Error::ComponentRange(box_err) => Some(box_err.as_ref()),
            Error::Parse(err) => Some(err),
            Error::Format(err) => Some(err),
            #[cfg(not(supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConversionRangeError;

impl fmt::Display for ConversionRangeError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(std)]
impl std::error::Error for ConversionRangeError {}

impl From<ConversionRangeError> for Error {
    #[inline(always)]
    fn from(_: ConversionRangeError) -> Self {
        Error::ConversionRange
    }
}

#[cfg(not(no_alloc))]
pub(crate) type RangeErrorGivenVec = Vec<(&'static str, i64)>;

// todo(heapless): figure out max heapless range error given vec size
#[cfg(no_alloc)]
pub(crate) type RangeErrorGivenVec = Vec<(&'static str, i64), heapless::consts::U5>;

/// An error type indicating that a component provided to a method was out of
/// range, causing a failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need
// for a type parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentRangeError {
    /// Name of the component.
    pub component_name: &'static str,
    /// Minimum allowed value, inclusive.
    pub minimum: i64,
    /// Maximum allowed value, inclusive.
    pub maximum: i64,
    /// Value that was provided.
    pub value: i64,
    /// The minimum and/or maximum is only valid with the following values.
    pub(crate) given: RangeErrorGivenVec,
}

impl fmt::Display for ComponentRangeError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be in the range {}..={}",
            self.component_name, self.minimum, self.maximum
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
    #[cfg(not(no_alloc))]
    fn from(original: ComponentRangeError) -> Self {
        Error::ComponentRange(Box::new(original))
    }
    #[inline(always)]
    #[cfg(no_alloc)]
    fn from(original: ComponentRangeError) -> Self {
        Error::ComponentRange(original)
    }
}

#[cfg(std)]
impl std::error::Error for ComponentRangeError {}

impl From<ParseError> for Error {
    #[inline(always)]
    fn from(original: ParseError) -> Self {
        Error::Parse(original)
    }
}

/// The system's UTC offset could not be determined at the given datetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IndeterminateOffsetError;

impl fmt::Display for IndeterminateOffsetError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The system's UTC offset could not be determined")
    }
}

#[cfg(std)]
impl std::error::Error for IndeterminateOffsetError {}

impl From<IndeterminateOffsetError> for Error {
    #[inline(always)]
    fn from(_: IndeterminateOffsetError) -> Self {
        Error::IndeterminateOffset
    }
}

/// An error occurred while formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(supports_non_exhaustive, non_exhaustive)]
pub enum FormatError {
    /// The format provided requires more information than the type provides.
    InsufficientTypeInformation,
    /// An error occurred while formatting into the provided stream.
    StdFmtError,
    #[cfg(not(supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for FormatError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FormatError::*;
        match self {
            InsufficientTypeInformation => {
                f.write_str("The format provided requires more information than the type provides.")
            }
            StdFmtError => fmt::Error.fmt(f),
            #[cfg(not(supports_non_exhaustive))]
            __NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(std)]
impl std::error::Error for FormatError {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FormatError::StdFmtError => Some(&fmt::Error),
            _ => None,
        }
    }
}

// This is strictly necessary to be able to use `?` with various formatters.
impl From<fmt::Error> for FormatError {
    #[inline(always)]
    fn from(_: fmt::Error) -> Self {
        FormatError::StdFmtError
    }
}

impl From<FormatError> for Error {
    #[inline(always)]
    fn from(error: FormatError) -> Self {
        Error::Format(error)
    }
}
