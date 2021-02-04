//! Parsing for various types.

#[macro_use]
mod combinator;

mod date;
mod offset;
mod parsed;
mod time;

pub use parsed::Parsed;

/// An error that occurred during parsing.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// A string literal was not what was expected.
    InvalidLiteral,
    /// A dynamic component was not valid.
    InvalidComponent(&'static str),
}
