use bench_util::setup_benchmark;
use criterion::BatchSize;
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    macros::{date, datetime, offset, time},
    PrimitiveDateTime,
};

setup_benchmark! {
    "PrimitiveDateTime",

    fn new(ben: &mut Bencher) {
        let d = date!("2019-01-01");
        let t = time!("0:00");
        ben.iter(|| PrimitiveDateTime::new(d, t));
    }

    fn date(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        ben.iter(|| a.date());
    }

    fn time(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        ben.iter(|| a.time());
    }

    fn year(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-12-31 0:00");
        let c = datetime!("2020-01-01 0:00");
        ben.iter(|| (
            a.year(),
            b.year(),
            c.year(),
        ));
    }

    fn month(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-12-31 0:00");
        ben.iter(|| (a.month(), b.month()));
    }

    fn day(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-12-31 0:00");
        ben.iter(|| (a.day(), b.day()));
    }

    fn month_day(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-12-31 0:00");
        ben.iter(|| (
            a.month_day(),
            b.month_day(),
        ));
    }

    fn ordinal(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-12-31 0:00");
        ben.iter(|| (
            a.ordinal(),
            b.ordinal(),
        ));
    }

    fn iso_year_week(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-10-04 0:00");
        let c = datetime!("2020-01-01 0:00");
        let d = datetime!("2020-12-31 0:00");
        let e = datetime!("2021-01-01 0:00");
        ben.iter(|| (
            a.iso_year_week(),
            b.iso_year_week(),
            c.iso_year_week(),
            d.iso_year_week(),
            e.iso_year_week(),
        ));
    }

    fn week(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-10-04 0:00");
        let c = datetime!("2020-01-01 0:00");
        let d = datetime!("2020-12-31 0:00");
        let e = datetime!("2021-01-01 0:00");
        ben.iter(|| (
            a.week(),
            b.week(),
            c.week(),
            d.week(),
            e.week(),
        ));
    }

    fn sunday_based_week(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2020-01-01 0:00");
        let c = datetime!("2020-12-31 0:00");
        let d = datetime!("2021-01-01 0:00");
        ben.iter(|| (
            a.sunday_based_week(),
            b.sunday_based_week(),
            c.sunday_based_week(),
            d.sunday_based_week(),
        ));
    }

    fn monday_based_week(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2020-01-01 0:00");
        let c = datetime!("2020-12-31 0:00");
        let d = datetime!("2021-01-01 0:00");
        ben.iter(|| (
            a.monday_based_week(),
            b.monday_based_week(),
            c.monday_based_week(),
            d.monday_based_week(),
        ));
    }

    fn weekday(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-02-01 0:00");
        let c = datetime!("2019-03-01 0:00");
        let d = datetime!("2019-04-01 0:00");
        let e = datetime!("2019-05-01 0:00");
        let f = datetime!("2019-06-01 0:00");
        let g = datetime!("2019-07-01 0:00");
        let h = datetime!("2019-08-01 0:00");
        let i = datetime!("2019-09-01 0:00");
        let j = datetime!("2019-10-01 0:00");
        let k = datetime!("2019-11-01 0:00");
        let l = datetime!("2019-12-01 0:00");

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

    fn hour(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-01-01 23:59:59");
        ben.iter(|| (
            a.hour(),
            b.hour(),
        ));
    }

    fn minute(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-01-01 23:59:59");
        ben.iter(|| (
            a.minute(),
            b.minute(),
        ));
    }

    fn second(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-01-01 23:59:59");
        ben.iter(|| (
            a.second(),
            b.second(),
        ));
    }

    fn millisecond(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-01-01 23:59:59.999");
        ben.iter(|| (
            a.millisecond(),
            b.millisecond(),
        ));
    }

    fn microsecond(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-01-01 23:59:59.999_999");
        ben.iter(|| (
            a.microsecond(),
            b.microsecond(),
        ));
    }

    fn nanosecond(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-01-01 23:59:59.999_999_999");
        ben.iter(|| (
            a.nanosecond(),
            b.nanosecond(),
        ));
    }

    fn assume_offset(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let b = datetime!("2019-01-01 0:00");
        ben.iter(|| (
            a.assume_offset(offset!("UTC")),
            b.assume_offset(offset!("-1")),
        ));
    }

    fn assume_utc(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        ben.iter(|| a.assume_utc());
    }

    fn format(ben: &mut Bencher) {
        let a = datetime!("2019-01-02 3:04:05");
        ben.iter(|| a.format("%c"));
    }

    fn parse(ben: &mut Bencher) {
        ben.iter(|| (
            PrimitiveDateTime::parse("Wed Jan 2 3:04:05 2019", "%c"),
            PrimitiveDateTime::parse("2019-002 23:59:59", "%Y-%j %T"),
            PrimitiveDateTime::parse("2019-W01-3 12:00:00 pm", "%G-W%V-%u %r"),
        ));
    }

    fn add_duration(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let dta = 5.days();
        let b = datetime!("2019-12-31 0:00");
        let dtb = 1.days();
        let c = datetime!("2019-12-31 23:59:59");
        let dtc = 2.seconds();
        let d = datetime!("2020-01-01 0:00:01");
        let dtd = (-2).seconds();
        let e = datetime!("1999-12-31 23:00");
        let dte = 1.hours();

        ben.iter(|| (
            a + dta,
            b + dtb,
            c + dtc,
            d + dtd,
            e + dte,
        ));
    }

    fn add_std_duration(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00");
        let dta = 5.std_days();
        let b = datetime!("2019-12-31 0:00");
        let dtb = 1.std_days();
        let c = datetime!("2019-12-31 23:59:59");
        let dtc = 2.std_seconds();

        ben.iter(|| (
            a + dta,
            b + dtb,
            c + dtc,
        ));
    }

    fn add_assign_duration(ben: &mut Bencher) {
        let dta = 1.days();
        let dtb = 1.seconds();
        ben.iter_batched_ref(
            || datetime!("2019-01-01 0:00"),
            |datetime| {
                *datetime += dta;
                *datetime += dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn add_assign_std_duration(ben: &mut Bencher) {
        let dta = 1.std_days();
        let dtb = 1.std_seconds();
        ben.iter_batched_ref(
            || datetime!("2019-01-01 0:00"),
            |datetime| {
                *datetime += dta;
                *datetime += dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_duration(ben: &mut Bencher) {
        let a = datetime!("2019-01-06 0:00");
        let dta = 5.days();
        let b = datetime!("2020-01-01 0:00");
        let dtb = 1.days();
        let c = datetime!("2020-01-01 0:00:01");
        let dtc = 2.seconds();
        let d = datetime!("2019-12-31 23:59:59");
        let dtd = (-2).seconds();
        let e = datetime!("1999-12-31 23:00");
        let dte = (-1).hours();

        ben.iter(|| (
            a - dta,
            b - dtb,
            c - dtc,
            d - dtd,
            e - dte,
        ));
    }

    fn sub_std_duration(ben: &mut Bencher) {
        let a = datetime!("2019-01-06 0:00");
        let dta = 5.std_days();
        let b = datetime!("2020-01-01 0:00");
        let dtb = 1.std_days();
        let c = datetime!("2020-01-01 0:00:01");
        let dtc = 2.std_seconds();

        ben.iter(|| (
            a - dta,
            b - dtb,
            c - dtc,
        ));
    }

    fn sub_assign_duration(ben: &mut Bencher) {
        let dta = 1.days();
        let dtb = 1.seconds();
        ben.iter_batched_ref(
            || datetime!("2019-01-01 0:00"),
            |datetime| {
                *datetime -= dta;
                *datetime -= dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_assign_std_duration(ben: &mut Bencher) {
        let dta = 1.std_days();
        let dtb = 1.std_seconds();
        ben.iter_batched_ref(
            || datetime!("2019-01-01 0:00"),
            |datetime| {
                *datetime -= dta;
                *datetime -= dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_datetime(ben: &mut Bencher) {
        let a = datetime!("2019-01-02 0:00");
        let b = datetime!("2019-01-01 0:00");
        let c = datetime!("2020-01-01 0:00");
        let d = datetime!("2019-12-31 0:00");
        ben.iter(|| (
            a - b,
            b - a,
            c - d,
            d - c,
        ));
    }

    fn ord(ben: &mut Bencher) {
        let d1a = datetime!("2019-01-01 0:00");
        let d1b = datetime!("2019-01-01 0:00");
        let d2a = datetime!("2019-01-01 0:00");
        let d2b = datetime!("2020-01-01 0:00");
        let d3a = datetime!("2019-01-01 0:00");
        let d3b = datetime!("2019-02-01 0:00");
        let d4a = datetime!("2019-01-01 0:00");
        let d4b = datetime!("2019-01-02 0:00");
        let d5a = datetime!("2019-01-01 0:00");
        let d5b = datetime!("2019-01-01 1:00");
        let d6a = datetime!("2019-01-01 0:00");
        let d6b = datetime!("2019-01-01 0:01");
        let d7a = datetime!("2019-01-01 0:00");
        let d7b = datetime!("2019-01-01 0:00:01");
        let d8a = datetime!("2019-01-01 0:00");
        let d8b = datetime!("2019-01-01 0:00:00.000_000_001");
        let d9a = datetime!("2020-01-01 0:00");
        let d9b = datetime!("2019-01-01 0:00");
        let d10a = datetime!("2019-02-01 0:00");
        let d10b = datetime!("2019-01-01 0:00");
        let d11a = datetime!("2019-01-02 0:00");
        let d11b = datetime!("2019-01-01 0:00");
        let d12a = datetime!("2019-01-01 1:00");
        let d12b = datetime!("2019-01-01 0:00");
        let d13a = datetime!("2019-01-01 0:01");
        let d13b = datetime!("2019-01-01 0:00");
        let d14a = datetime!("2019-01-01 0:00:01");
        let d14b = datetime!("2019-01-01 0:00");
        let d15a = datetime!("2019-01-01 0:00:00.000_000_001");
        let d15b = datetime!("2019-01-01 0:00");

        ben.iter(|| (
            d1a.partial_cmp(&d1b),
            d2a.partial_cmp(&d2b),
            d3a.partial_cmp(&d3b),
            d4a.partial_cmp(&d4b),
            d5a.partial_cmp(&d5b),
            d6a.partial_cmp(&d6b),
            d7a.partial_cmp(&d7b),
            d8a.partial_cmp(&d8b),
            d9a.partial_cmp(&d9b),
            d10a.partial_cmp(&d10b),
            d11a.partial_cmp(&d11b),
            d12a.partial_cmp(&d12b),
            d13a.partial_cmp(&d13b),
            d14a.partial_cmp(&d14b),
            d15a.partial_cmp(&d15b),
        ));
    }

    fn display(ben: &mut Bencher) {
        let a = datetime!("1970-01-01 0:00");
        ben.iter(|| a.to_string());
    }
}
