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
    clippy::nursery,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::use_debug,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::enum_glob_use,
    clippy::inline_always,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::suspicious_arithmetic_impl,
    clippy::use_self,
    clippy::wildcard_imports,
    clippy::zero_prefixed_literal,
    unstable_name_collisions
)]

// This is required on rustc < 1.42.0.
#[allow(unused_extern_crates)]
extern crate proc_macro;

macro_rules! error {
    ($message:literal) => {
        error!(::proc_macro2::Span::call_site(), $message)
    };

    ($span:expr, $message:literal) => {
        Err(::syn::Error::new($span, $message))
    };

    ($span:expr, $($args:expr),+) => {
        Err(::syn::Error::new($span, format!($($args),+)))
    };
}

mod kw {
    use syn::custom_keyword;
    custom_keyword!(am);
    custom_keyword!(pm);
    custom_keyword!(AM);
    custom_keyword!(PM);
    custom_keyword!(utc);
    custom_keyword!(UTC);
}

mod date;
mod ext;
mod offset;
mod time;
mod time_crate;

use date::Date;
use offset::Offset;
use proc_macro_hack::proc_macro_hack;
use quote::ToTokens;
use syn::parse_macro_input;
use time::Time;

#[proc_macro_hack]
#[allow(clippy::unimplemented)]
pub fn time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Time).to_token_stream().into()
}

#[proc_macro_hack]
#[allow(clippy::unimplemented)]
pub fn offset(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Offset).to_token_stream().into()
}

#[proc_macro_hack]
#[allow(clippy::unimplemented)]
pub fn date(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Date).to_token_stream().into()
}
