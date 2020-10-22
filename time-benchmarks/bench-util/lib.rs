/// Helper macro that wraps code blocks in benchmark functions, and makes sure
/// to add all those functions to the Criterion benchmark group.
#[macro_export]
macro_rules! setup_benchmark {
    (
        $group_prefix:literal,
        $(
            $(#[$fn_attr:meta])*
            fn $fn_name:ident ($bencher:ident : &mut Bencher)
            $code:block
        )*
    ) => {
        $(
            $(#[$fn_attr])*
            pub fn $fn_name(c: &mut criterion::Criterion) {
                c.bench_function(&format!("{}: {}", $group_prefix, stringify!($fn_name)), |$bencher| {
                    $code
                });
            }
        )*

        criterion::criterion_group! {
            name = benches;
            config = criterion::Criterion::default()
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
        criterion::criterion_main!(benches);
    };
}
