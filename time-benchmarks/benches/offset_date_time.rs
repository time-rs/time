use bench_util::setup_benchmark;
use criterion::BatchSize;
use std::time::SystemTime;
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    macros::{datetime, offset},
    OffsetDateTime,
};

setup_benchmark! {
    "OffsetDateTime",

    fn now_utc(ben: &mut Bencher) {
        ben.iter(OffsetDateTime::now_utc);
    }

    fn now_local(ben: &mut Bencher) {
        ben.iter(OffsetDateTime::now_local);
    }

    fn to_offset(ben: &mut Bencher) {
        let sydney = datetime!("2000-01-01 0:00 +11");
        let new_york_offset = offset!("-5");
        let los_angeles_offset = offset!("-8");

        ben.iter(|| (
            sydney.to_offset(new_york_offset),
            sydney.to_offset(los_angeles_offset),
        ));
    }

    fn from_unix_timestamp(ben: &mut Bencher) {
        ben.iter(|| (
            OffsetDateTime::from_unix_timestamp(0),
            OffsetDateTime::from_unix_timestamp(1_546_300_800),
        ));
    }

    fn from_unix_timestamp_nanos(ben: &mut Bencher) {
        ben.iter(|| (
            OffsetDateTime::from_unix_timestamp_nanos(0),
            OffsetDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
        ));
    }

    fn offset(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 0:00 +1");
        let c = datetime!("2019-01-01 0:00 UTC").to_offset(offset!("+1"));

        ben.iter(|| (
            a.offset(),
            b.offset(),
            c.offset(),
        ));
    }

    fn unix_timestamp(ben: &mut Bencher) {
        let a = OffsetDateTime::UNIX_EPOCH;
        let b = OffsetDateTime::UNIX_EPOCH.to_offset(offset!("+1"));
        let c = datetime!("1970-01-01 0:00 -1");
        ben.iter(|| (
            a.unix_timestamp(),
            b.unix_timestamp(),
            c.unix_timestamp(),
        ));
    }

    fn unix_timestamp_nanos(ben: &mut Bencher) {
        let a = datetime!("1970-01-01 0:00 UTC");
        let b = datetime!("1970-01-01 1:00 UTC").to_offset(offset!("-1"));
        ben.iter(|| (
            a.unix_timestamp_nanos(),
            b.unix_timestamp_nanos(),
        ));
    }

    fn date(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 0:00 UTC").to_offset(offset!("-1"));
        ben.iter(|| (a.date(), b.date()));
    }

    fn time(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 0:00 UTC").to_offset(offset!("-1"));
        ben.iter(|| (a.time(), b.time()));
    }

    fn year(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-12-31 23:00 UTC").to_offset(offset!("+1"));
        let c = datetime!("2020-01-01 0:00 UTC");
        ben.iter(|| (
            a.year(),
            b.year(),
            c.year(),
        ));
    }

    fn month(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-12-31 23:00 UTC").to_offset(offset!("+1"));
        ben.iter(|| (a.month(), b.month()));
    }

    fn day(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-12-31 23:00 UTC").to_offset(offset!("+1"));
        ben.iter(|| (a.day(), b.day()));
    }

    fn ordinal(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-12-31 23:00 UTC").to_offset(offset!("+1"));
        ben.iter(|| (a.ordinal(), b.ordinal()));
    }

    fn iso_week(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2020-01-01 0:00 UTC");
        let c = datetime!("2020-12-31 0:00 UTC");
        let d = datetime!("2021-01-01 0:00 UTC");
        ben.iter(|| (
            a.iso_week(),
            b.iso_week(),
            c.iso_week(),
            d.iso_week(),
        ));
    }

    fn weekday(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-02-01 0:00 UTC");
        let c = datetime!("2019-03-01 0:00 UTC");
        ben.iter(|| (
            a.weekday(),
            b.weekday(),
            c.weekday(),
        ));
    }

    fn hour(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 23:59:59 UTC").to_offset(offset!("-2"));
        ben.iter(|| (a.hour(), b.hour()));
    }

    fn minute(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 23:59:59 UTC").to_offset(offset!("+0:30"));
        ben.iter(|| (a.minute(), b.minute()));
    }

    fn second(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 23:59:59 UTC").to_offset(offset!("+0:00:30"));
        ben.iter(|| (a.second(), b.second()));
    }

    fn millisecond(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 23:59:59.999 UTC");
        ben.iter(|| (
            a.millisecond(),
            b.millisecond(),
        ));
    }

    fn microsecond(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 23:59:59.999_999 UTC");
        ben.iter(|| (
            a.microsecond(),
            b.microsecond(),
        ));
    }

    fn nanosecond(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 23:59:59.999_999_999 UTC");
        ben.iter(|| (
            a.nanosecond(),
            b.nanosecond(),
        ));
    }

    fn partial_eq(ben: &mut Bencher) {
        let a = datetime!("2000-01-01 0:00 UTC").to_offset(offset!("-1"));
        let b = datetime!("2000-01-01 0:00 UTC");
        ben.iter(|| a == b);
    }

    fn partial_ord(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 0:00 UTC").to_offset(offset!("-1"));
        ben.iter(|| a.partial_cmp(&b));
    }

    fn ord(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 0:00 UTC").to_offset(offset!("-1"));
        let c = datetime!("2019-01-01 0:00:00.000_000_001 UTC");
        ben.iter(|| (a == b, c > a));
    }

    fn hash(ben: &mut Bencher) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        let a = datetime!("2019-01-01 0:00 UTC");
        let b = datetime!("2019-01-01 0:00 UTC").to_offset(offset!("-1"));
        let c = datetime!("2019-01-01 0:00");

        ben.iter_batched_ref(
            DefaultHasher::new,
            |hasher| {
                a.hash(hasher);
                b.hash(hasher);
                c.hash(hasher);
            },
            BatchSize::SmallInput
        );
    }

    fn add_duration(ben: &mut Bencher) {
        let a = datetime!("2019-01-01 0:00 UTC");
        let dta = 5.days();
        let b = datetime!("2019-12-31 0:00 UTC");
        let dtb = 1.days();
        let c = datetime!("2019-12-31 23:59:59 UTC");
        let dtc = 2.seconds();
        let d = datetime!("2020-01-01 0:00:01 UTC");
        let dtd = (-2).seconds();
        let e = datetime!("1999-12-31 23:00 UTC");
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
        let a = datetime!("2019-01-01 0:00 UTC");
        let dta = 5.std_days();
        let b = datetime!("2019-12-31 0:00 UTC");
        let dtb = 1.std_days();
        let c = datetime!("2019-12-31 23:59:59 UTC");
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
            || datetime!("2019-01-01 0:00 UTC"),
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
            || datetime!("2019-01-01 0:00 UTC"),
            |datetime| {
                *datetime += dta;
                *datetime += dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_duration(ben: &mut Bencher) {
        let a = datetime!("2019-01-06 0:00 UTC");
        let dta = 5.days();
        let b = datetime!("2020-01-01 0:00 UTC");
        let dtb = 1.days();
        let c = datetime!("2020-01-01 0:00:01 UTC");
        let dtc = 2.seconds();

        ben.iter(|| (
            a - dta,
            b - dtb,
            c - dtc,
        ));
    }

    fn sub_std_duration(ben: &mut Bencher) {
        let a = datetime!("2019-01-06 0:00 UTC");
        let dta = 5.std_days();
        let b = datetime!("2020-01-01 0:00 UTC");
        let dtb = 1.std_days();
        let c = datetime!("2020-01-01 0:00:01 UTC");
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
            || datetime!("2019-01-01 0:00 UTC"),
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
            || datetime!("2019-01-01 0:00 UTC"),
            |datetime| {
                *datetime -= dta;
                *datetime -= dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn std_add_duration(ben: &mut Bencher) {
        let a = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let dta = 0.seconds();
        let b = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let dtb = 5.days();
        let c = SystemTime::from(datetime!("2019-12-31 0:00 UTC"));
        let dtc = 1.days();
        let d = SystemTime::from(datetime!("2019-12-31 23:59:59 UTC"));
        let dtd = 2.seconds();
        let e = SystemTime::from(datetime!("2020-01-01 0:00:01 UTC"));
        let dte = (-2).seconds();
        ben.iter(|| (
            a + dta,
            b + dtb,
            c + dtc,
            d + dtd,
            e + dte,
        ));
    }

    fn std_add_assign_duration(ben: &mut Bencher) {
        let dta = 1.days();
        let dtb = 1.seconds();
        ben.iter_batched_ref(
            || SystemTime::from(datetime!("2019-01-01 0:00 UTC")),
            |datetime| {
                *datetime += dta;
                *datetime += dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn std_sub_duration(ben: &mut Bencher) {
        let a = SystemTime::from(datetime!("2019-01-06 0:00 UTC"));
        let dta = 5.days();
        let b = SystemTime::from(datetime!("2020-01-01 0:00 UTC"));
        let dtb = 1.days();
        let c = SystemTime::from(datetime!("2020-01-01 0:00:01 UTC"));
        let dtc = 2.seconds();
        let d = SystemTime::from(datetime!("2019-12-31 23:59:59 UTC"));
        let dtd = (-2).seconds();
        ben.iter(|| (
            a - dta,
            b - dtb,
            c - dtc,
            d - dtd,
        ));
    }

    fn std_sub_assign_duration(ben: &mut Bencher) {
        let dta = 1.days();
        let dtb = 1.seconds();
        ben.iter_batched_ref(
            || SystemTime::from(datetime!("2019-01-01 0:00 UTC")),
            |datetime| {
                *datetime -= dta;
                *datetime -= dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_self(ben: &mut Bencher) {
        let a = datetime!("2019-01-02 0:00 UTC");
        let b = datetime!("2019-01-01 0:00 UTC");
        let c = datetime!("2020-01-01 0:00 UTC");
        let d = datetime!("2019-12-31 0:00 UTC");

        ben.iter(|| (
            a - b,
            b - a,
            c - d,
            d - c,
        ));
    }

    fn std_sub(ben: &mut Bencher) {
        let a_std = SystemTime::from(datetime!("2019-01-02 0:00 UTC"));
        let a = datetime!("2019-01-01 0:00 UTC");
        let b_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let b = datetime!("2019-01-02 0:00 UTC");
        let c_std = SystemTime::from(datetime!("2020-01-01 0:00 UTC"));
        let c = datetime!("2019-12-31 0:00 UTC");
        let d_std = SystemTime::from(datetime!("2019-12-31 0:00 UTC"));
        let d = datetime!("2020-01-01 0:00 UTC");

        ben.iter(|| (
            a_std - a,
            b_std - b,
            c_std - c,
            d_std - d,
        ));
    }

    fn sub_std(ben: &mut Bencher) {
        let a = datetime!("2019-01-02 0:00 UTC");
        let a_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let b = datetime!("2019-01-01 0:00 UTC");
        let b_std = SystemTime::from(datetime!("2019-01-02 0:00 UTC"));
        let c = datetime!("2020-01-01 0:00 UTC");
        let c_std = SystemTime::from(datetime!("2019-12-31 0:00 UTC"));
        let d = datetime!("2019-12-31 0:00 UTC");
        let d_std = SystemTime::from(datetime!("2020-01-01 0:00 UTC"));

        ben.iter(|| (
            a - a_std,
            b - b_std,
            c - c_std,
            d - d_std,
        ));
    }

    fn eq_std(ben: &mut Bencher) {
        let a = OffsetDateTime::now_utc();
        let b_std = SystemTime::from(a);
        ben.iter(|| a == b_std);
    }

    fn std_eq(ben: &mut Bencher) {
        let a = OffsetDateTime::now_utc();
        let b_std = SystemTime::from(a);
        ben.iter(|| b_std == a)
    }

    fn ord_std(ben: &mut Bencher) {
        let d1 = datetime!("2019-01-01 0:00 UTC");
        let d1_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d2 = datetime!("2019-01-01 0:00 UTC");
        let d2_std = SystemTime::from(datetime!("2020-01-01 0:00 UTC"));
        let d3 = datetime!("2019-01-01 0:00 UTC");
        let d3_std = SystemTime::from(datetime!("2019-02-01 0:00 UTC"));
        let d4 = datetime!("2019-01-01 0:00 UTC");
        let d4_std = SystemTime::from(datetime!("2019-01-02 0:00 UTC"));
        let d5 = datetime!("2019-01-01 0:00 UTC");
        let d5_std = SystemTime::from(datetime!("2019-01-01 1:00:00 UTC"));
        let d6 = datetime!("2019-01-01 0:00 UTC");
        let d6_std = SystemTime::from(datetime!("2019-01-01 0:01:00 UTC"));
        let d7 = datetime!("2019-01-01 0:00 UTC");
        let d7_std = SystemTime::from(datetime!("2019-01-01 0:00:01 UTC"));
        let d8 = datetime!("2019-01-01 0:00 UTC");
        let d8_std = SystemTime::from(datetime!("2019-01-01 0:00:00.001 UTC"));
        let d9 = datetime!("2020-01-01 0:00 UTC");
        let d9_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d10 = datetime!("2019-02-01 0:00 UTC");
        let d10_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d11 = datetime!("2019-01-02 0:00 UTC");
        let d11_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d12 = datetime!("2019-01-01 1:00:00 UTC");
        let d12_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d13 = datetime!("2019-01-01 0:01:00 UTC");
        let d13_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d14 = datetime!("2019-01-01 0:00:01 UTC");
        let d14_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d15 = datetime!("2019-01-01 0:00:00.000_000_001 UTC");
        let d15_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));

        ben.iter(|| (
            d1 == d1_std,
            d2 < d2_std,
            d3 < d3_std,
            d4 < d4_std,
            d5 < d5_std,
            d6 < d6_std,
            d7 < d7_std,
            d8 < d8_std,
            d9 > d9_std,
            d10 > d10_std,
            d11 > d11_std,
            d12 > d12_std,
            d13 > d13_std,
            d14 > d14_std,
            d15 > d15_std,
        ));
    }

    fn std_ord(ben: &mut Bencher) {
        let d1_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d1 = datetime!("2019-01-01 0:00 UTC");
        let d2_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d2 = datetime!("2020-01-01 0:00 UTC");
        let d3_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d3 = datetime!("2019-02-01 0:00 UTC");
        let d4_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d4 = datetime!("2019-01-02 0:00 UTC");
        let d5_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d5 = datetime!("2019-01-01 1:00:00 UTC");
        let d6_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d6 = datetime!("2019-01-01 0:01:00 UTC");
        let d7_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d7 = datetime!("2019-01-01 0:00:01 UTC");
        let d8_std = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
        let d8 = datetime!("2019-01-01 0:00:00.000_000_001 UTC");
        let d9_std = SystemTime::from(datetime!("2020-01-01 0:00 UTC"));
        let d9 = datetime!("2019-01-01 0:00 UTC");
        let d10_std = SystemTime::from(datetime!("2019-02-01 0:00 UTC"));
        let d10 = datetime!("2019-01-01 0:00 UTC");
        let d11_std = SystemTime::from(datetime!("2019-01-02 0:00 UTC"));
        let d11 = datetime!("2019-01-01 0:00 UTC");
        let d12_std = SystemTime::from(datetime!("2019-01-01 1:00:00 UTC"));
        let d12 = datetime!("2019-01-01 0:00 UTC");
        let d13_std = SystemTime::from(datetime!("2019-01-01 0:01:00 UTC"));
        let d13 = datetime!("2019-01-01 0:00 UTC");
        let d14_std = SystemTime::from(datetime!("2019-01-01 0:00:01 UTC"));
        let d14 = datetime!("2019-01-01 0:00 UTC");
        let d15_std = SystemTime::from(datetime!("2019-01-01 0:00:00.001 UTC"));
        let d15 = datetime!("2019-01-01 0:00 UTC");

        ben.iter(|| (
            d1_std == d1,
            d2_std < d2,
            d3_std < d3,
            d4_std < d4,
            d5_std < d5,
            d6_std < d6,
            d7_std < d7,
            d8_std < d8,
            d9_std > d9,
            d10_std > d10,
            d11_std > d11,
            d12_std > d12,
            d13_std > d13,
            d14_std > d14,
            d15_std > d15,
        ));
    }

    fn from_std(ben: &mut Bencher) {
        let a = SystemTime::UNIX_EPOCH;
        let b = SystemTime::UNIX_EPOCH - 1.std_days();
        let c = SystemTime::UNIX_EPOCH + 1.std_days();
        ben.iter(|| (
            OffsetDateTime::from(a),
            OffsetDateTime::from(b),
            OffsetDateTime::from(c),
        ));
    }

    fn to_std(ben: &mut Bencher) {
        let a = OffsetDateTime::UNIX_EPOCH;
        let b = OffsetDateTime::UNIX_EPOCH + 1.days();
        let c = OffsetDateTime::UNIX_EPOCH - 1.days();
        ben.iter(|| (
            SystemTime::from(a),
            SystemTime::from(b),
            SystemTime::from(c),
        ));
    }

    fn display(ben: &mut Bencher) {
        let a = datetime!("1970-01-01 0:00 UTC");
        ben.iter(|| a.to_string());
    }
}
