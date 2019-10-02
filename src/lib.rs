#![no_std]
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
    missing_docs,
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
/// Ensure certain methods are present on all types.
mod shim;
/// The `Sign` struct and its associated `impl`s.
mod sign;

pub use duration::Duration;
pub(crate) use shim::NumberExt;
pub use sign::Sign;

#[allow(missing_docs)]
#[deprecated(
    since = "0.2.0",
    note = "This error will never be produced by non-deprecated methods."
)]
pub struct OutOfRangeError;
