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
#![allow(clippy::many_single_char_names)]

macro_rules! setup_benchmark {
    (
        $group_prefix:literal,
        $(
            $(#[$fn_attr:meta])*
            fn $fn_name:ident ($bencher:ident : $bencher_type:ty)
            $code:block
        )*
    ) => {
        $(
            $(#[$fn_attr])*
            fn $fn_name(
                c: &mut ::criterion::Criterion<::criterion_cycles_per_byte::CyclesPerByte>
            ) {
                c.bench_function(
                    concat!($group_prefix, ": ", stringify!($fn_name)),
                    |$bencher: $bencher_type| $code
                );
            }
        )*

        ::criterion::criterion_group! {
            name = benches;
            config = ::criterion::Criterion::default()
                .with_measurement(::criterion_cycles_per_byte::CyclesPerByte)
                // Set a stricter statistical significance threshold ("p-value")
                // for deciding what's an actual performance change vs. noise.
                // The more benchmarks, the lower this needs to be in order to
                // not get lots of false positives.
                .significance_level(0.0001)
                // Ignore any performance change less than this (0.05 = 5%) as
                // noise, regardless of statistical significance.
                .noise_threshold(0.05)
                // Reduce the time taken to run each benchmark
                .warm_up_time(::std::time::Duration::from_millis(100))
                .measurement_time(::std::time::Duration::from_millis(400));
            targets = $($fn_name,)*
        }
    };
}

macro_rules! iter_batched_ref {
    ($ben:ident, $initializer:expr,[$($routine:expr),+ $(,)?]) => {$(
        $ben.iter_batched_ref(
            $initializer,
            $routine,
            ::criterion::BatchSize::SmallInput,
        );
    )+};
}

macro_rules! mods {
    ($(mod $mod:ident;)+) => {
        $(mod $mod;)+
        ::criterion::criterion_main!($($mod::benches),+);
    }
}

mods![
    mod date;
    mod duration;
    mod formatting;
    mod instant;
    mod month;
    mod offset_date_time;
    mod parsing;
    mod primitive_date_time;
    mod rand;
    mod time;
    mod utc_offset;
    mod util;
    mod weekday;
];
