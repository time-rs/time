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
            fn $fn_name(c: &mut ::criterion::Criterion) {
                c.bench_function(
                    concat!($group_prefix, ": ", stringify!($fn_name)),
                    |$bencher: $bencher_type| $code
                );
            }
        )*

        ::criterion::criterion_group! {
            name = benches;
            config = ::criterion::Criterion::default()
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

macro_rules! mods {
    ($($mod:ident)+) => {
        $(mod $mod;)+
        criterion::criterion_main!($($mod::benches),+);
    }
}

mods![
    date
    duration
    instant
    offset_date_time
    primitive_date_time
    rand
    time
    utc_offset
    util
    weekday
];
