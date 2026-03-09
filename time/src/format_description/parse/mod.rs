//! Parser for format descriptions.

use alloc::boxed::Box;
use alloc::vec::Vec;

use self::sealed::{Version, VersionedParser};
pub use self::strftime::{parse_strftime_borrowed, parse_strftime_owned};
use crate::{error, format_description};

/// A helper macro to make version restrictions simpler to read and write.
macro_rules! version {
    ($range:expr) => {
        $range.contains(&VERSION)
    };
}

/// A helper macro to statically validate the version (when used as a const parameter).
macro_rules! validate_version {
    ($version:ident) => {
        const {
            assert!($version >= 1 && $version <= 3);
        }
    };
}

mod ast;
mod format_item;
mod lexer;
mod strftime;

mod sealed {
    use super::*;

    /// The version of the parser, represented in the type system.
    #[expect(
        missing_debug_implementations,
        reason = "only used at the type level; not public API"
    )]
    pub struct Version<const N: usize>;

    /// A trait for parsing format descriptions, with different output types depending on the
    /// version.
    pub trait VersionedParser {
        /// The output type of the borrowed parser. This type avoids allocating where possible.
        type BorrowedOutput<'input>;

        /// The output type of the owned parser. This type may allocate but is valid for `'static`.
        type OwnedOutput;

        /// Parse a format description into a type that avoids allocating where possible.
        fn parse_borrowed(
            s: &str,
        ) -> Result<Self::BorrowedOutput<'_>, error::InvalidFormatDescription>;

        /// Parse a format description into an owned type, which may allocate but is valid for
        /// `'static`.
        fn parse_owned(s: &str) -> Result<Self::OwnedOutput, error::InvalidFormatDescription>;
    }
}

impl VersionedParser for Version<1> {
    type BorrowedOutput<'input> = Vec<format_description::BorrowedFormatItem<'input>>;
    type OwnedOutput = format_description::OwnedFormatItem;

    #[inline]
    fn parse_borrowed(
        s: &str,
    ) -> Result<Self::BorrowedOutput<'_>, error::InvalidFormatDescription> {
        let mut lexed = lexer::lex::<1>(s.as_bytes());
        let ast = ast::parse::<_, 1>(&mut lexed);
        let format_items = format_item::parse(ast);
        Ok(format_items
            .map(|res| res.and_then(TryInto::try_into))
            .collect::<Result<_, _>>()?)
    }

    #[inline]
    fn parse_owned(s: &str) -> Result<Self::OwnedOutput, error::InvalidFormatDescription> {
        let mut lexed = lexer::lex::<1>(s.as_bytes());
        let ast = ast::parse::<_, 1>(&mut lexed);
        let format_items = format_item::parse(ast);
        let items = format_items.collect::<Result<Box<_>, _>>()?;
        Ok(items.try_into()?)
    }
}

impl VersionedParser for Version<2> {
    type BorrowedOutput<'input> = Vec<format_description::BorrowedFormatItem<'input>>;
    type OwnedOutput = format_description::OwnedFormatItem;

    #[inline]
    fn parse_borrowed(
        s: &str,
    ) -> Result<Self::BorrowedOutput<'_>, error::InvalidFormatDescription> {
        let mut lexed = lexer::lex::<2>(s.as_bytes());
        let ast = ast::parse::<_, 2>(&mut lexed);
        let format_items = format_item::parse(ast);
        Ok(format_items
            .map(|res| res.and_then(TryInto::try_into))
            .collect::<Result<_, _>>()?)
    }

    #[inline]
    fn parse_owned(s: &str) -> Result<Self::OwnedOutput, error::InvalidFormatDescription> {
        let mut lexed = lexer::lex::<2>(s.as_bytes());
        let ast = ast::parse::<_, 2>(&mut lexed);
        let format_items = format_item::parse(ast);
        let items = format_items.collect::<Result<Box<_>, _>>()?;
        Ok(items.try_into()?)
    }
}

impl VersionedParser for Version<3> {
    type BorrowedOutput<'input> = format_description::FormatDescriptionV3<'input>;
    type OwnedOutput = format_description::FormatDescriptionV3<'static>;

    #[inline]
    fn parse_borrowed(
        s: &str,
    ) -> Result<Self::BorrowedOutput<'_>, error::InvalidFormatDescription> {
        let mut lexed = lexer::lex::<3>(s.as_bytes());
        let ast = ast::parse::<_, 3>(&mut lexed);
        let format_items = format_item::parse(ast);
        let items = format_items.collect::<Result<Box<_>, _>>()?;
        let inner = format_description::__private::FormatDescriptionV3Inner::try_from(items)?;
        Ok(inner.into_opaque())
    }

    #[inline]
    fn parse_owned(s: &str) -> Result<Self::OwnedOutput, error::InvalidFormatDescription> {
        let mut lexed = lexer::lex::<3>(s.as_bytes());
        let ast = ast::parse::<_, 3>(&mut lexed);
        let format_items = format_item::parse(ast);
        let items = format_items.collect::<Result<Box<_>, _>>()?;
        let inner = format_description::__private::FormatDescriptionV3Inner::try_from(items)?;
        Ok(inner.into_opaque().to_owned())
    }
}

/// Parse a sequence of items from the format description.
///
/// The syntax for the format description can be found in [the
/// book](https://time-rs.github.io/book/api/format-description.html).
///
/// This function exists for backward compatibility reasons. It is equivalent to calling
/// `parse_borrowed::<1>(s)`. In the future, this function will be deprecated in favor of
/// `parse_borrowed`.
#[inline]
pub fn parse(
    s: &str,
) -> Result<Vec<format_description::BorrowedFormatItem<'_>>, error::InvalidFormatDescription> {
    parse_borrowed::<1>(s)
}

/// Parse a sequence of items from the format description.
///
/// The syntax for the format description can be found in [the
/// book](https://time-rs.github.io/book/api/format-description.html). The version of the format
/// description is provided as the const parameter. **It is recommended to use version 3.**
///
/// # Return type
///
/// The return type of this function depends on the version provided.
///
/// - For versions 1 and 2, the function returns `Result<Vec<BorrowedFormatItem<'_>>,
///   InvalidFormatDescription>`.
/// - For version 3, the function returns `Result<FormatDescriptionV3<'_>,
///   InvalidFormatDescription>`.
#[inline]
pub fn parse_borrowed<const VERSION: usize>(
    s: &str,
) -> Result<
    <Version<VERSION> as VersionedParser>::BorrowedOutput<'_>,
    error::InvalidFormatDescription,
>
where
    Version<VERSION>: VersionedParser,
{
    Version::<VERSION>::parse_borrowed(s)
}

/// Parse a sequence of items from the format description.
///
/// The syntax for the format description can be found in [the
/// book](https://time-rs.github.io/book/api/format-description.html). The version of the format
/// description is provided as the const parameter.
///
/// Unlike [`parse`], this function returns [`OwnedFormatItem`], which owns its contents. This means
/// that there is no lifetime that needs to be handled. **It is recommended to use version 2.**
///
/// # Return type
///
/// The return type of this function depends on the version provided.
///
/// - For versions 1 and 2, the function returns `Result<OwnedFormatItem,
///   InvalidFormatDescription>`.
/// - For version 3, the function returns `Result<FormatDescriptionV3<'static>,
///   InvalidFormatDescription>`.
///
/// [`OwnedFormatItem`]: crate::format_description::OwnedFormatItem
#[inline]
pub fn parse_owned<const VERSION: usize>(
    s: &str,
) -> Result<<Version<VERSION> as VersionedParser>::OwnedOutput, error::InvalidFormatDescription>
where
    Version<VERSION>: VersionedParser,
{
    Version::<VERSION>::parse_owned(s)
}

/// Attach [`Location`] information to each byte in the iterator.
#[inline]
fn attach_location<'item>(
    iter: impl Iterator<Item = &'item u8>,
) -> impl Iterator<Item = (&'item u8, Location)> {
    let mut byte_pos = 0;

    iter.map(move |byte| {
        let location = Location { byte: byte_pos };
        byte_pos += 1;
        (byte, location)
    })
}

/// A location within a string.
#[derive(Clone, Copy)]
struct Location {
    /// The zero-indexed byte of the string.
    byte: u32,
}

impl Location {
    /// Create a new [`Span`] from `self` to `other`.
    #[inline]
    const fn to(self, end: Self) -> Span {
        Span { start: self, end }
    }

    /// Create a new [`Span`] consisting entirely of `self`.
    #[inline]
    const fn to_self(self) -> Span {
        Span {
            start: self,
            end: self,
        }
    }

    /// Offset the location by the provided amount.
    ///
    /// Note that this assumes the resulting location is on the same line as the original location.
    #[must_use = "this does not modify the original value"]
    #[inline]
    const fn offset(&self, offset: u32) -> Self {
        Self {
            byte: self.byte + offset,
        }
    }

    /// Create an error with the provided message at this location.
    #[inline]
    const fn error(self, message: &'static str) -> ErrorInner {
        ErrorInner {
            _message: message,
            _span: Span {
                start: self,
                end: self,
            },
        }
    }
}

/// A start and end point within a string.
#[derive(Clone, Copy)]
struct Span {
    start: Location,
    end: Location,
}

impl Span {
    const DUMMY: Self = Self {
        start: Location { byte: u32::MAX },
        end: Location { byte: u32::MAX },
    };

    /// Obtain a `Span` pointing at the start of the pre-existing span.
    #[must_use = "this does not modify the original value"]
    #[inline]
    const fn shrink_to_start(&self) -> Self {
        Self {
            start: self.start,
            end: self.start,
        }
    }

    /// Obtain a `Span` pointing at the end of the pre-existing span.
    #[must_use = "this does not modify the original value"]
    const fn shrink_to_end(&self) -> Self {
        Self {
            start: self.end,
            end: self.end,
        }
    }

    /// Obtain a `Span` that ends before the provided position of the pre-existing span.
    #[must_use = "this does not modify the original value"]
    #[inline]
    const fn shrink_to_before(&self, pos: u32) -> Self {
        Self {
            start: self.start,
            end: Location {
                byte: self.start.byte + pos - 1,
            },
        }
    }

    /// Obtain a `Span` that starts after provided position to the end of the pre-existing span.
    #[must_use = "this does not modify the original value"]
    #[inline]
    const fn shrink_to_after(&self, pos: u32) -> Self {
        Self {
            start: Location {
                byte: self.start.byte + pos + 1,
            },
            end: self.end,
        }
    }

    /// Create an error with the provided message at this span.
    #[inline]
    const fn error(self, message: &'static str) -> ErrorInner {
        ErrorInner {
            _message: message,
            _span: self,
        }
    }
}

/// A value with an associated [`Span`].
#[derive(Clone, Copy)]
struct Spanned<T> {
    /// The value.
    value: T,
    /// Where the value was in the format string.
    span: Span,
}

impl<T> core::ops::Deref for Spanned<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Spanned<T> {
    #[inline]
    fn map<F, U>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned {
            value: f(self.value),
            span: self.span,
        }
    }
}

trait OptionExt<T> {
    fn transpose(self) -> Spanned<Option<T>>;
}

impl<T> OptionExt<T> for Option<Spanned<T>> {
    #[inline]
    fn transpose(self) -> Spanned<Option<T>> {
        match self {
            Some(spanned) => Spanned {
                value: Some(spanned.value),
                span: spanned.span,
            },
            None => Spanned {
                value: None,
                span: Span::DUMMY,
            },
        }
    }
}

/// Helper trait to attach a [`Span`] to a value.
trait SpannedValue: Sized {
    /// Attach a [`Span`] to a value.
    fn spanned(self, span: Span) -> Spanned<Self>;
}

impl<T> SpannedValue for T {
    #[inline]
    fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned { value: self, span }
    }
}

/// The internal error type.
struct ErrorInner {
    /// The message displayed to the user.
    _message: &'static str,
    /// Where the error originated.
    _span: Span,
}

/// A complete error description.
struct Error {
    /// The internal error.
    _inner: Unused<ErrorInner>,
    /// The error needed for interoperability with the rest of `time`.
    public: error::InvalidFormatDescription,
}

impl From<Error> for error::InvalidFormatDescription {
    #[inline]
    fn from(error: Error) -> Self {
        error.public
    }
}

/// A value that may be used in the future, but currently is not.
///
/// This struct exists so that data can semantically be passed around without _actually_ passing it
/// around. This way the data still exists if it is needed in the future.
// `PhantomData` is not used directly because we don't want to introduce any trait implementations.
struct Unused<T>(core::marker::PhantomData<T>);

/// Indicate that a value is currently unused.
#[inline]
fn unused<T>(_: T) -> Unused<T> {
    Unused(core::marker::PhantomData)
}
