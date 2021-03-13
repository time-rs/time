use criterion::{black_box, Bencher};
use time::util;

setup_benchmark! {
    "Utils",

    fn is_leap_year(ben: &mut Bencher) {
        ben.iter(|| {
            for year in 0..400 {
                black_box(util::is_leap_year(year));
            }
        });
    }

    fn days_in_year(ben: &mut Bencher) {
        ben.iter(|| {
            for year in 0..400 {
                black_box(util::days_in_year(year));
            }
        });
    }

    fn weeks_in_year(ben: &mut Bencher) {
        ben.iter(|| {
            for year in 0..400 {
                black_box(util::weeks_in_year(year));
            }
        });
    }
}
