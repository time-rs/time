use std::num::NonZeroU8;

use bench_util::setup_benchmark;
use time::ext::NumericalStdDuration;

setup_benchmark! {
    "Numerical durations (std)",

    fn unsigned(ben: &mut Bencher) {
        ben.iter(|| (
            5.std_nanoseconds(),
            5.std_microseconds(),
            5.std_milliseconds(),
            5.std_seconds(),
            5.std_minutes(),
            5.std_hours(),
            5.std_days(),
            5.std_weeks(),
        ));
    }

    fn unsigned_typed(ben: &mut Bencher) {
        ben.iter(|| (
            5_u64.std_nanoseconds(),
            5_u64.std_microseconds(),
            5_u64.std_milliseconds(),
            5_u64.std_seconds(),
            5_u64.std_minutes(),
            5_u64.std_hours(),
            5_u64.std_days(),
            5_u64.std_weeks(),
        ));
    }

    fn nonzero(ben: &mut Bencher) {
        let nz = NonZeroU8::new(5).unwrap();
        ben.iter(|| (
            nz.std_nanoseconds(),
            nz.std_microseconds(),
            nz.std_milliseconds(),
            nz.std_seconds(),
            nz.std_minutes(),
            nz.std_hours(),
            nz.std_days(),
            nz.std_weeks(),
        ));
    }

    fn float(ben: &mut Bencher) {
        ben.iter(|| (
            1.9.std_nanoseconds(),
            1.0.std_nanoseconds(),
            1.0.std_microseconds(),
            1.0.std_milliseconds(),
            1.0.std_seconds(),
            1.0.std_minutes(),
            1.0.std_hours(),
            1.0.std_days(),
            1.0.std_weeks(),
            1.5.std_nanoseconds(),
            1.5.std_microseconds(),
            1.5.std_milliseconds(),
            1.5.std_seconds(),
            1.5.std_minutes(),
            1.5.std_hours(),
            1.5.std_days(),
            1.5.std_weeks(),
        ));
    }
}
