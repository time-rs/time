#![allow(dead_code)] // TODO remove this

//! Parsing for the time crate.

macro_rules! try_parse_all {
    ($input:ident, $($e:expr),+ $(,)?) => {{
        let mut __input = *$input;
        let __ret_val = ($($e(&mut __input)?,)+);
        *$input = __input;
        __ret_val
    }};
}

mod combinator;
mod offset;
mod parsed;

pub use parsed::Parsed;

/// An error that occurred during parsing.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Error {}
