use bench_util::setup_benchmark;
use criterion::{black_box, BatchSize};
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    macros::{date, time},
    util, Date, Weekday,
};

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

    fn parse_monday_based_week(ben: &mut Bencher) {
        ben.iter(|| Date::parse("Sun 00 2023", "%a %W %Y"));
    }

    fn parse_sunday_based_week(ben: &mut Bencher) {
        ben.iter(|| Date::parse("Sun 01 2018", "%a %U %Y"));
    }

    fn from_iso_ywd(ben: &mut Bencher) {
        use Weekday::*;
        ben.iter(|| (
            Date::from_iso_ywd(2019, 1, Monday),
            Date::from_iso_ywd(2019, 1, Tuesday),
            Date::from_iso_ywd(2020, 53, Friday),
            Date::from_iso_ywd(2019, 53, Monday),
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

    fn iso_year_week(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        ben.iter(|| d.iso_year_week());
    }

    fn week(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        ben.iter(|| d.week());
    }

    fn as_ymd(ben: &mut Bencher) {
        let d = date!("2019-01-02");
        ben.iter(|| d.as_ymd());
    }

    fn as_yo(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        ben.iter(|| d.as_yo());
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
        let d = date!("2019-01-01");
        ben.iter(|| d.next_day());
    }

    fn previous_day(ben: &mut Bencher) {
        let d = date!("2019-01-02");
        ben.iter(|| d.previous_day());
    }

    fn julian_day(ben: &mut Bencher) {
        let d = date!("-100_000-01-01");
        ben.iter(|| d.julian_day());
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

    fn format(ben: &mut Bencher) {
        // Check all specifiers for date objects.
        let date = date!("2019-01-02");
        ben.iter(|| (
            date.format("%a"),
            date.format("%A"),
            date.format("%b"),
            date.format("%B"),
            date.format("%C"),
            date.format("%d"),
            date.format("%D"),
            date.format("%F"),
            date.format("%g"),
            date.format("%G"),
            date.format("%j"),
            date.format("%m"),
            date.format("%u"),
            date.format("%U"),
            date.format("%V"),
            date.format("%w"),
            date.format("%W"),
            date.format("%y"),
            date.format("%Y"),
        ));
    }

    fn parse(ben: &mut Bencher) {
        ben.iter(|| (
            Date::parse("2019-01-02 Wed", "%F %a"),
            Date::parse("2019-01-02 Wednesday", "%F %A"),
            Date::parse("2019-01-02 Jan", "%F %b"),
            Date::parse("2019-01-02 January", "%F %B"),
            Date::parse("2019-01-02 20", "%F %C"),
            Date::parse("2019-01-02 02", "%F %d"),
            Date::parse("2019-01-02 1/02/19", "%F %D"),
            Date::parse("2019-01-02", "%F"),
            Date::parse("2019-01-02 19", "%F %g"),
            Date::parse("2019-01-02 2019", "%F %G"),
            Date::parse("2019-01-02 002", "%F %j"),
            Date::parse("2019-01-02 01", "%F %m"),
            Date::parse("2019-01-02 3", "%F %u"),
            Date::parse("2019-01-02 00", "%F %U"),
            Date::parse("2019-01-02 01", "%F %V"),
            Date::parse("2019-01-02 3", "%F %w"),
            Date::parse("2019-01-02 00", "%F %W"),
            Date::parse("2019-01-02 19", "%F %y"),
            Date::parse("2019-01-02 2019", "%F %Y"),
            Date::parse("", ""),
        ));
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
