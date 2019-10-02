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
    unused_results,
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

/// The `Duration` struct and its associated `impl`s.
mod duration;
/// The `Instant` struct and its associated `impl`s.
#[cfg(feature = "std")]
mod instant;
/// Ensure certain methods are present on all types.
mod shim;
/// The `Sign` struct and its associated `impl`s.
mod sign;
/// Days of the week.
mod weekday;

pub use duration::Duration;
#[cfg(feature = "std")]
pub use instant::Instant;
pub(crate) use shim::NumberExt;
pub use sign::Sign;
pub use weekday::Weekday;

#[allow(clippy::missing_docs_in_private_items, deprecated)]
#[deprecated(
    since = "0.2.0",
    note = "This error will never be produced by non-deprecated methods."
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRangeError;

#[allow(deprecated)]
impl core::fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Source duration value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
#[allow(deprecated)]
impl std::error::Error for OutOfRangeError {}
