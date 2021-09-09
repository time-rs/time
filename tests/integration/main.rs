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
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::enum_glob_use,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::clone_on_copy,
    clippy::default_trait_access,
    clippy::let_underscore_drop,
    clippy::cmp_owned
)]

/// Construct a non-exhaustive modifier.
macro_rules! modifier {
    ($name:ident {
        $($field:ident $(: $value:expr)?),+ $(,)?
    }) => {{
        let mut value = ::time::format_description::modifier::$name::default();
        $(value.$field = modifier!(@value $field $($value)?);)+
        value
    }};

    (@value $field:ident) => ($field);
    (@value $field:ident $value:expr) => ($value);
}

mod date;
mod derives;
mod duration;
mod error;
mod ext;
mod formatting;
mod instant;
mod macros;
mod month;
mod offset_date_time;
mod parse_format_description;
mod parsed;
mod parsing;
mod primitive_date_time;
mod quickcheck;
mod rand;
mod serde;
mod time;
mod utc_offset;
mod util;
mod weekday;

#[cfg(__ui_tests)]
#[test]
fn compile_fail() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/integration/compile-fail/*.rs");
}
