#![no_std]

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use time_macros_impl::{date, datetime, offset, time};
