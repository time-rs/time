//! Simple time handling.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    anonymous_parameters,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    clippy::all
)]
#![warn(
    unused_extern_crates,
    box_pointers,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_qualifications,
    variant_size_differences,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items
)]
#![allow(
    clippy::suspicious_arithmetic_impl,
    clippy::inline_always,
    // TODO Change to `warn` once rust-lang/rust-clippy#4605 is resolved.
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
)]

// Include the `format!` macro in `#![no_std]` environments.
#[macro_use]
extern crate alloc;

macro_rules! format_conditional {
    ($conditional:ident) => {
        format!(concat!(stringify!($conditional), "={}"), $conditional)
    };

    ($first_conditional:ident, $($conditional:ident),*) => {{
        let mut s = alloc::string::String::new();
        s.push_str(&format_conditional!($first_conditional));
        $(s.push_str(&format!(concat!(", ", stringify!($conditional), "={}"), $conditional));)*
        s
    }}
}

macro_rules! assert_value_in_range {
    ($value:ident in $start:expr => $end:expr) => {
        if !($start..=$end).contains(&$value) {
            panic!(
                concat!(stringify!($value), " must be in the range {}..={}"),
                $start,
                $end,
            );
        }
    };

    ($value:ident in $start:expr => exclusive $end:expr) => {
        if !($start..$end).contains(&$value) {
            panic!(
                concat!(stringify!($value), " must be in the range {}..{}"),
                $start,
                $end,
            );
        }
    };

    ($value:ident in $start:expr => $end:expr, given $($conditional:ident),+ $(,)?) => {
        if !($start..=$end).contains(&$value) {
            panic!(
                concat!(stringify!($value), " must be in the range {}..={} given{}"),
                $start,
                $end,
                &format_conditional!($($conditional),+)
            );
        };
    };
}

/// The `Date` struct and its associated `impl`s.
mod date;
/// The `DateTime` struct and its associated `impl`s.
mod datetime;
/// The `Duration` struct and its associated `impl`s.
mod duration;
/// The `Instant` struct and its associated `impl`s.
#[cfg(feature = "std")]
mod instant;
/// Ensure certain methods are present on all types.
mod shim;
/// The `Sign` struct and its associated `impl`s.
mod sign;
/// The `Time` struct and its associated `impl`s.
mod time;
/// Days of the week.
mod weekday;

pub use self::time::Time;
pub use date::{days_in_year, is_leap_year, weeks_in_year, Date};
pub use datetime::DateTime;
pub use duration::Duration;
#[cfg(feature = "std")]
pub use instant::Instant;
pub(crate) use shim::NumberExt;
pub use sign::Sign;
pub use weekday::Weekday;

// For some back-compatibility, we're also implementing some deprecated methods.

#[cfg(feature = "std")]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(since = "0.2.0", note = "Use `Instant`")]
pub type PreciseTime = Instant;

#[cfg(feature = "std")]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(since = "0.2.0", note = "Use `Instant`")]
pub type SteadyTime = Instant;

// Include zero-sized field so users can't construct this explicitly.
#[allow(clippy::missing_docs_in_private_items, deprecated)]
#[deprecated(
    since = "0.2.0",
    note = "This error is only produced by deprecated methods."
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRangeError(());

#[allow(deprecated)]
impl core::fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Source duration value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
#[allow(deprecated)]
impl std::error::Error for OutOfRangeError {}
