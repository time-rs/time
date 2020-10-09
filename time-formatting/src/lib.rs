//! Formatting for the time crate.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(__time_formatting_01_docs, feature(doc_cfg))]
#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::use_debug,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::enum_glob_use,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::redundant_pub_crate,
    clippy::use_self,
    unstable_name_collisions
)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod format_description;

#[cfg(feature = "alloc")]
pub use format_description::parse::parse_format_description;
