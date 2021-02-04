#![allow(dead_code)] // TODO remove this

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
pub enum Error {}
