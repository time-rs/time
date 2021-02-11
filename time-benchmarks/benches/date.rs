use bench_util::setup_benchmark;
use criterion::{black_box, BatchSize};
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{date, time};
use time::{util, Date, Weekday};

setup_benchmark! {
    "Date",

    fn debug(ben: &mut Bencher) {
        let d = date!("2020-02-03");
        ben.iter(|| format!("{:?}", d));
    }

    fn weeks_in_year_exhaustive(ben: &mut Bencher) {
        ben.iter(|| {
            for year in 0..400 {
                black_box(util::weeks_in_year(year));
            }
        });
    }

    fn monday_based_week(ben: &mut Bencher) {
        let d = date!("2023-01-01");
        ben.iter(|| d.monday_based_week());
    }

    fn sunday_based_week(ben: &mut Bencher) {
        let d = date!("2023-01-01");
        ben.iter(|| d.sunday_based_week());
    }

    fn from_iso_ywd(ben: &mut Bencher) {
        use Weekday::*;
        ben.iter(|| (
            Date::from_iso_week_date(2019, 1, Monday),
            Date::from_iso_week_date(2019, 1, Tuesday),
            Date::from_iso_week_date(2020, 53, Friday),
            Date::from_iso_week_date(2019, 53, Monday),
        ));
    }

    fn year(ben: &mut Bencher) {
        let d = date!("2019-002");
        ben.iter(|| d.year());
    }

    fn month(ben: &mut Bencher) {
        let d = date!("2019-002");
        ben.iter(|| d.month());
    }

    fn day(ben: &mut Bencher) {
        let d = date!("2019-002");
        ben.iter(|| d.day());
    }

    fn iso_week(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        ben.iter(|| d.iso_week());
    }

    fn to_calendar_date(ben: &mut Bencher) {
        let d = date!("2019-01-02");
        ben.iter(|| d.to_calendar_date());
    }

    fn to_ordinal_date(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        ben.iter(|| d.to_ordinal_date());
    }

    fn to_iso_week_date(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        ben.iter(|| d.to_iso_week_date());
    }

    fn weekday(ben: &mut Bencher) {
        let a = date!("2019-01-01");
        let b = date!("2019-02-01");
        let c = date!("2019-03-01");
        let d = date!("2019-04-01");
        let e = date!("2019-05-01");
        let f = date!("2019-06-01");
        let g = date!("2019-07-01");
        let h = date!("2019-08-01");
        let i = date!("2019-09-01");
        let j = date!("2019-10-01");
        let k = date!("2019-11-01");
        let l = date!("2019-12-01");

        ben.iter(|| (
            a.weekday(),
            b.weekday(),
            c.weekday(),
            d.weekday(),
            e.weekday(),
            f.weekday(),
            g.weekday(),
            h.weekday(),
            i.weekday(),
            j.weekday(),
            k.weekday(),
            l.weekday(),
        ));
    }

    fn next_day(ben: &mut Bencher) {
        let a = date!("2019-01-01");
        let b = date!("2019-02-01");
        let c = date!("2019-12-31");
        let d = date!("2020-12-31");
        let e = Date::MAX;

        ben.iter(|| (
            a.next_day(),
            b.next_day(),
            c.next_day(),
            d.next_day(),
            e.next_day(),
        ));
    }

    fn previous_day(ben: &mut Bencher) {
        let a = date!("2019-01-02");
        let b = date!("2019-02-01");
        let c = date!("2020-01-01");
        let d = date!("2021-01-01");
        let e = Date::MIN;

        ben.iter(|| (
            a.previous_day(),
            b.previous_day(),
            c.previous_day(),
            d.previous_day(),
            e.previous_day(),
        ));
    }

    fn to_julian_day(ben: &mut Bencher) {
        let d = date!("2000-01-01");
        ben.iter(|| d.to_julian_day());
    }

    fn from_julian_day(ben: &mut Bencher) {
        ben.iter(|| Date::from_julian_day(-34_803_190));
    }

    fn midnight(ben: &mut Bencher) {
        let d = date!("1970-01-01");
        ben.iter(|| d.midnight());
    }

    fn with_time(ben: &mut Bencher) {
        let d = date!("1970-01-01");
        let t = time!("0:00");
        ben.iter(|| d.with_time(t));
    }

    fn with_hms(ben: &mut Bencher) {
        let d = date!("1970-01-01");
        ben.iter(|| d.with_hms(0, 0, 0));
    }

    fn with_hms_milli(ben: &mut Bencher) {
        let d = date!("1970-01-01");
        ben.iter(|| d.with_hms_milli(0, 0, 0, 0));
    }

    fn with_hms_micro(ben: &mut Bencher) {
        let d = date!("1970-01-01");
        ben.iter(|| d.with_hms_micro(0, 0, 0, 0));
    }

    fn with_hms_nano(ben: &mut Bencher) {
        let d = date!("1970-01-01");
        ben.iter(|| d.with_hms_nano(0, 0, 0, 0));
    }

    fn display(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        ben.iter(|| d.to_string());
    }

    fn add(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        let dt = 5.days();
        ben.iter(|| d + dt);
    }

    fn add_std(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        let dt = 5.std_days();
        ben.iter(|| d + dt);
    }

    fn add_assign(ben: &mut Bencher) {
        let dt = 1.days();
        ben.iter_batched_ref(
            || date!("2019-12-31"),
            |date| {
                *date += dt;
            },
            BatchSize::SmallInput
        );
    }

    fn add_assign_std(ben: &mut Bencher) {
        let dt = 1.std_days();
        ben.iter_batched_ref(
            || date!("2019-12-31"),
            |date| {
                *date += dt;
            },
            BatchSize::SmallInput
        );
    }

    fn sub(ben: &mut Bencher) {
        let d = date!("2019-01-06");
        let dt = 5.days();
        ben.iter(|| d - dt);
    }

    fn sub_std(ben: &mut Bencher) {
        let d = date!("2019-01-06");
        let dt = 5.std_days();
        ben.iter(|| d - dt);
    }

    fn sub_assign(ben: &mut Bencher) {
        let dt = 1.days();
        ben.iter_batched_ref(
            || date!("2020-01-01"),
            |date| {
                *date -= dt;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_assign_std(ben: &mut Bencher) {
        let dt = 1.std_days();
        ben.iter_batched_ref(
            || date!("2020-01-01"),
            |date| {
                *date -= dt;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_self(ben: &mut Bencher) {
        let first = date!("2019-01-01");
        let second = date!("2019-01-02");
        ben.iter(|| second - first);
    }

    fn partial_ord(ben: &mut Bencher) {
        let first = date!("2019-01-01");
        let second = date!("2019-01-02");
        ben.iter(|| (
            first.partial_cmp(&first),
            first.partial_cmp(&second),
            second.partial_cmp(&first),
        ));
    }

    fn ord(ben: &mut Bencher) {
        let first = date!("2019-01-01");
        let second = date!("2019-01-02");
        ben.iter(|| (
            first.cmp(&first),
            first.cmp(&second),
            second.cmp(&first),
        ));
    }

}
