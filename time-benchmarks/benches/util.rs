use bench_util::setup_benchmark;
use criterion::black_box;
use time::util;

setup_benchmark! {
    "Utils",

    fn is_leap_year(ben: &mut Bencher) {
        ben.iter(|| (
            util::is_leap_year(1900),
            util::is_leap_year(2000),
            util::is_leap_year(2004),
            util::is_leap_year(2005),
            util::is_leap_year(2100),
        ));
    }

    fn days_in_year(ben: &mut Bencher) {
        ben.iter(|| (
            util::days_in_year(1900),
            util::days_in_year(2000),
            util::days_in_year(2004),
            util::days_in_year(2005),
            util::days_in_year(2100),
        ));
    }

    fn weeks_in_year(ben: &mut Bencher) {
        ben.iter(|| {
            for year in 0..400 {
                black_box(util::weeks_in_year(year));
            }
        });
    }
}
