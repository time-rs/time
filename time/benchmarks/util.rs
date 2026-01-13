use std::hint::black_box as bb;

use criterion::Bencher;
use time::util;

setup_benchmark! {
    "Utils",

    fn noop(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for i in 0..400 {
                let _ = bb(i);
            }
        });
    }

    fn is_leap_year(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for year in 0..400 {
                let _ = bb(util::is_leap_year(bb(year)));
            }
        });
    }

    fn days_in_year(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for year in 0..400 {
                let _ = bb(util::days_in_year(bb(year)));
            }
        });
    }

    fn weeks_in_year(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for year in 0..400 {
                let _ = bb(util::weeks_in_year(bb(year)));
            }
        });
    }
}
