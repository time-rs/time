#![allow(dead_code)] // TODO remove this

//! Parsing for various types.

macro_rules! try_parse_all {
    ($input:ident, $($e:expr),+ $(,)?) => {{
        let mut __input = *$input;
        let __ret_val = ($($e(&mut __input)?,)+);
        *$input = __input;
        __ret_val
    }};
}

#[macro_use]
mod combinator;

mod offset;
mod parsed;
mod time;

pub use parsed::Parsed;

/// An error that occurred during parsing.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Error {}
