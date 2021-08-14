use criterion::{black_box, Bencher};
use criterion_cycles_per_byte::CyclesPerByte;
use time::{util, Month};

setup_benchmark! {
    "Utils",

    fn days_in_year_month(ben: &mut Bencher<'_, CyclesPerByte>) {
        // Common year
        ben.iter(|| util::days_in_year_month(2019, Month::January));
        ben.iter(|| util::days_in_year_month(2019, Month::February));
        ben.iter(|| util::days_in_year_month(2019, Month::March));
        ben.iter(|| util::days_in_year_month(2019, Month::April));
        ben.iter(|| util::days_in_year_month(2019, Month::May));
        ben.iter(|| util::days_in_year_month(2019, Month::June));
        ben.iter(|| util::days_in_year_month(2019, Month::July));
        ben.iter(|| util::days_in_year_month(2019, Month::August));
        ben.iter(|| util::days_in_year_month(2019, Month::September));
        ben.iter(|| util::days_in_year_month(2019, Month::October));
        ben.iter(|| util::days_in_year_month(2019, Month::November));
        ben.iter(|| util::days_in_year_month(2019, Month::December));

        // Leap year
        ben.iter(|| util::days_in_year_month(2020, Month::January));
        ben.iter(|| util::days_in_year_month(2020, Month::February));
        ben.iter(|| util::days_in_year_month(2020, Month::March));
        ben.iter(|| util::days_in_year_month(2020, Month::April));
        ben.iter(|| util::days_in_year_month(2020, Month::May));
        ben.iter(|| util::days_in_year_month(2020, Month::June));
        ben.iter(|| util::days_in_year_month(2020, Month::July));
        ben.iter(|| util::days_in_year_month(2020, Month::August));
        ben.iter(|| util::days_in_year_month(2020, Month::September));
        ben.iter(|| util::days_in_year_month(2020, Month::October));
        ben.iter(|| util::days_in_year_month(2020, Month::November));
        ben.iter(|| util::days_in_year_month(2020, Month::December));
    }

    fn is_leap_year(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| {
            for year in 0..400 {
                black_box(util::is_leap_year(year));
            }
        });
    }

    fn days_in_year(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| {
            for year in 0..400 {
                black_box(util::days_in_year(year));
            }
        });
    }

    fn weeks_in_year(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| {
            for year in 0..400 {
                black_box(util::weeks_in_year(year));
            }
        });
    }
}
