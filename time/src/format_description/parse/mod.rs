//! Parser for format descriptions.

use alloc::boxed::Box;
use alloc::vec::Vec;

mod ast;
mod format_item;
mod lexer;

/// Parse a sequence of items from the format description.
///
/// The syntax for the format description can be found in [the
/// book](https://time-rs.github.io/book/api/format-description.html).
pub fn parse(
    s: &str,
) -> Result<Vec<crate::format_description::FormatItem<'_>>, crate::error::InvalidFormatDescription>
{
    let mut lexed = lexer::lex(s.as_bytes());
    let ast = ast::parse(&mut lexed);
    let format_items = format_item::parse(ast);
    Ok(format_items
        .map(|res| res.and_then(TryInto::try_into))
        .collect::<Result<_, _>>()?)
}

/// Parse a sequence of items from the format description.
///
/// The syntax for the format description can be found in [the
/// book](https://time-rs.github.io/book/api/format-description.html).
///
/// Unlike [`parse`], this function returns [`OwnedFormatItem`], which owns its contents. This means
/// that there is no lifetime that needs to be handled.
///
/// [`OwnedFormatItem`]: crate::format_description::OwnedFormatItem
pub fn parse_owned(
    s: &str,
) -> Result<crate::format_description::OwnedFormatItem, crate::error::InvalidFormatDescription> {
    let mut lexed = lexer::lex(s.as_bytes());
    let ast = ast::parse(&mut lexed);
    let format_items = format_item::parse(ast);
    let items = format_items
        .map(|res| res.map(Into::into))
        .collect::<Result<Box<_>, _>>()?;
    Ok(items.into())
}

/// A location within a string.
#[derive(Clone, Copy)]
struct Location {
    /// The zero-indexed byte of the string.
    byte: u32,
}

impl Location {
    /// Create a new [`Span`] from `self` to `other`.
    const fn to(self, end: Self) -> Span {
        Span { start: self, end }
    }

    /// Offset the location by the provided amount.
    ///
    /// Note that this assumes the resulting location is on the same line as the original location.
    #[must_use = "this does not modify the original value"]
    const fn offset(&self, offset: u32) -> Self {
        Self {
            byte: self.byte + offset,
        }
    }

    /// Create an error with the provided message at this location.
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
    #[allow(clippy::missing_docs_in_private_items)]
    start: Location,
    #[allow(clippy::missing_docs_in_private_items)]
    end: Location,
}

impl Span {
    /// Obtain a `Span` pointing at the start of the pre-existing span.
    #[must_use = "this does not modify the original value"]
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
    const fn shrink_to_after(&self, pos: u32) -> Self {
        Self {
            start: Location {
                byte: self.start.byte + pos + 1,
            },
            end: self.end,
        }
    }

    /// Create an error with the provided message at this span.
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

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Helper trait to attach a [`Span`] to a value.
trait SpannedValue: Sized {
    /// Attach a [`Span`] to a value.
    fn spanned(self, span: Span) -> Spanned<Self>;
}

impl<T> SpannedValue for T {
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
    _inner: ErrorInner,
    /// The error needed for interoperability with the rest of `time`.
    public: crate::error::InvalidFormatDescription,
}

impl From<Error> for crate::error::InvalidFormatDescription {
    fn from(error: Error) -> Self {
        error.public
    }
}
