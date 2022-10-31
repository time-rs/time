#![allow(clippy::missing_docs_in_private_items)]

use alloc::vec::Vec;
use core::ops::{RangeFrom, RangeTo};

mod ast;
mod format_item;
mod lexer;

pub fn parse(
    s: &str,
) -> Result<Vec<crate::format_description::FormatItem<'_>>, crate::error::InvalidFormatDescription>
{
    let lexed = lexer::lex(s.as_bytes());
    let ast = ast::parse(lexed);
    let format_items = format_item::parse(ast);
    let items = format_items.collect::<Result<Vec<_>, _>>()?;

    Ok(items.into_iter().map(Into::into).collect())
}

#[derive(Clone, Copy)]
struct Location {
    line: usize,
    column: usize,
    byte: usize,
}

impl Location {
    #[must_use = "this does not modify the original value"]
    const fn offset(&self, offset: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + offset,
            byte: self.byte + offset,
        }
    }

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

#[derive(Clone, Copy)]
struct Span {
    start: Location,
    end: Location,
}

impl Span {
    const fn start_end(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    #[must_use = "this does not modify the original value"]
    fn subspan(&self, range: impl Subspan) -> Self {
        range.subspan(self)
    }

    #[must_use = "this does not modify the original value"]
    const fn shrink_to_start(&self) -> Self {
        Self {
            start: self.start,
            end: self.start,
        }
    }

    #[must_use = "this does not modify the original value"]
    const fn shrink_to_end(&self) -> Self {
        Self {
            start: self.end,
            end: self.end,
        }
    }

    const fn error(self, message: &'static str) -> ErrorInner {
        ErrorInner {
            _message: message,
            _span: self,
        }
    }

    const fn start_byte(&self) -> usize {
        self.start.byte
    }
}

trait Subspan {
    fn subspan(self, span: &Span) -> Span;
}

impl Subspan for RangeFrom<usize> {
    fn subspan(self, span: &Span) -> Span {
        assert_eq!(span.start.line, span.end.line);

        Span {
            start: Location {
                line: span.start.line,
                column: span.start.column + self.start,
                byte: span.start.byte + self.start,
            },
            end: span.end,
        }
    }
}

impl Subspan for RangeTo<usize> {
    fn subspan(self, span: &Span) -> Span {
        assert_eq!(span.start.line, span.end.line);

        Span {
            start: span.start,
            end: Location {
                line: span.start.line,
                column: span.start.column + self.end - 1,
                byte: span.start.byte + self.end - 1,
            },
        }
    }
}

struct ErrorInner {
    _message: &'static str,
    _span: Span,
}

struct Error {
    _inner: ErrorInner,
    public: crate::error::InvalidFormatDescription,
}

impl From<Error> for crate::error::InvalidFormatDescription {
    fn from(error: Error) -> Self {
        error.public
    }
}
