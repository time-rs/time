//! Parsing for various types.

mod combinator;
mod component;
mod parsed;

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
