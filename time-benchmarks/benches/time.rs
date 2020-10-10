use bench_util::setup_benchmark;
use criterion::BatchSize;
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    time, Time,
};

setup_benchmark! {
    "Time",

    fn midnight(ben: &mut Bencher) {
        ben.iter(Time::midnight);
    }

    fn from_hms(ben: &mut Bencher) {
        ben.iter(|| (
            Time::from_hms(1, 2, 3),
            Time::from_hms(24, 0, 0),
            Time::from_hms(0, 60, 0),
            Time::from_hms(0, 0, 60),
        ));
    }

    fn from_hms_milli(ben: &mut Bencher) {
        ben.iter(|| (
            Time::from_hms_milli(1, 2, 3, 4),
            Time::from_hms_milli(24, 0, 0, 0),
            Time::from_hms_milli(0, 60, 0, 0),
            Time::from_hms_milli(0, 0, 60, 0),
            Time::from_hms_milli(0, 0, 0, 1_000),
        ));
    }

    fn from_hms_micro(ben: &mut Bencher) {
        ben.iter(|| (
            Time::from_hms_micro(1, 2, 3, 4),
            Time::from_hms_micro(24, 0, 0, 0),
            Time::from_hms_micro(0, 60, 0, 0),
            Time::from_hms_micro(0, 0, 60, 0),
            Time::from_hms_micro(0, 0, 0, 1_000_000),
        ));
    }

    fn from_hms_nano(ben: &mut Bencher) {
        ben.iter(|| (
            Time::from_hms_nano(1, 2, 3, 4),
            Time::from_hms_nano(24, 0, 0, 0),
            Time::from_hms_nano(0, 60, 0, 0),
            Time::from_hms_nano(0, 0, 60, 0),
            Time::from_hms_nano(0, 0, 0, 1_000_000_000),
        ))
    }

    fn hour(ben: &mut Bencher) {
        let a = Time::from_hms(0, 0, 0).unwrap();
        let b = Time::from_hms(0, 59, 59).unwrap();
        let c = Time::from_hms(23, 0, 0).unwrap();
        let d = Time::from_hms(23, 59, 59).unwrap();
        ben.iter(|| (
            a.hour(),
            b.hour(),
            c.hour(),
            d.hour(),
        ));
    }

    fn minute(ben: &mut Bencher) {
        let a = Time::from_hms(0, 0, 0).unwrap();
        let b = Time::from_hms(23, 0, 59).unwrap();
        let c = Time::from_hms(0, 23, 0).unwrap();
        let d = Time::from_hms(23, 23, 59).unwrap();
        ben.iter(|| (
            a.minute(),
            b.minute(),
            c.minute(),
            d.minute(),
        ));
    }

    fn second(ben: &mut Bencher) {
        let a = Time::from_hms(0, 0, 0).unwrap();
        let b = Time::from_hms(23, 59, 0).unwrap();
        let c = Time::from_hms(0, 0, 23).unwrap();
        let d = Time::from_hms(23, 59, 23).unwrap();
        ben.iter(|| (
            a.second(),
            b.second(),
            c.second(),
            d.second(),
        ));
    }

    fn millisecond(ben: &mut Bencher) {
        let a = Time::from_hms_milli(0, 0, 0, 0).unwrap();
        let b = Time::from_hms_milli(23, 59, 59, 0).unwrap();
        let c = Time::from_hms_milli(0, 0, 0, 999).unwrap();
        let d = Time::from_hms_milli(23, 59, 59, 999).unwrap();
        ben.iter(|| (
            a.millisecond(),
            b.millisecond(),
            c.millisecond(),
            d.millisecond(),
        ));
    }

    fn microsecond(ben: &mut Bencher) {
        let a = Time::from_hms_micro(0, 0, 0, 0).unwrap();
        let b = Time::from_hms_micro(23, 59, 59, 0).unwrap();
        let c = Time::from_hms_micro(0, 0, 0, 999_999).unwrap();
        let d = Time::from_hms_micro(23, 59, 59, 999_999).unwrap();
        ben.iter(|| (
            a.microsecond(),
            b.microsecond(),
            c.microsecond(),
            d.microsecond(),
        ));
    }

    fn nanosecond(ben: &mut Bencher) {
        let a = Time::from_hms_nano(0, 0, 0, 0).unwrap();
        let b = Time::from_hms_nano(23, 59, 59, 0).unwrap();
        let c = Time::from_hms_nano(0, 0, 0, 999_999_999).unwrap();
        let d = Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap();
        ben.iter(|| (
            a.nanosecond(),
            b.nanosecond(),
            c.nanosecond(),
            d.nanosecond(),
        ));
    }

    fn format(ben: &mut Bencher) {
        let time = time!("0:01:02.345_678_901");
        let time_12h = time!("12:01:02");
        ben.iter(|| (
            time.format("%H"),
            time.format("%I"),
            time.format("%M"),
            time.format("%N"),
            time.format("%p"),
            time.format("%P"),
            time.format("%r"),
            time.format("%R"),
            time.format("%S"),
            time.format("%T"),
            time_12h.format("%p"),
            time_12h.format("%P"),
        ));
    }

    fn parse(ben: &mut Bencher) {
        ben.iter(|| (
            Time::parse("0:01:02.345678901 00", "%T.%N %H"),
            Time::parse("0:01:02.345678901 12", "%T.%N %I"),
            Time::parse("0:01:02.345678901 01", "%T.%N %M"),
            Time::parse("0:01:02.345678901 345678901", "%T.%N %N"),
            Time::parse("0:01:02.345678901 am", "%T.%N %p"),
            Time::parse("0:01:02.345678901 AM", "%T.%N %P"),
            Time::parse("0:01:02.345678901 12:01:02 am", "%T.%N %r"),
            Time::parse("0:01:02.345678901 0:01", "%T.%N %R"),
            Time::parse("0:01:02.345678901 02", "%T.%N %S"),
            Time::parse("0:01:02.345678901 0:01:02", "%T.%N %T"),
            Time::parse("1:00 am", "%-I:%M %p"),
            Time::parse("1:00 pm", "%-I:%M %p"),
            Time::parse(" 1:00 am", "%_I:%M %p"),
            Time::parse(" 1:00 pm", "%_I:%M %p"),
            Time::parse("01:00 am", "%0I:%M %p"),
            Time::parse("01:00 pm", "%0I:%M %p"),
            Time::parse("1:02:03.456789012 pm", "%-I:%M:%S.%N %p"),
            Time::parse("", ""),
        ));
    }

    fn parse_missing_seconds(ben: &mut Bencher) {
        ben.iter(|| (
            Time::parse("0:00", "%-H:%M"),
            Time::parse("23:59", "%H:%M"),
            Time::parse("12:00 am", "%I:%M %p"),
            Time::parse("12:00 pm", "%I:%M %p"),
        ));
    }

    fn parse_missing_minutes(ben: &mut Bencher) {
        ben.iter(|| (
            Time::parse("0", "%-H"),
            Time::parse("23", "%H"),
            Time::parse("12am", "%I%p"),
            Time::parse("12pm", "%I%p"),
        ));
    }

    fn display(ben: &mut Bencher) {
        let a = time!("0:00");
        let b = time!("23:59");
        let c = time!("23:59:59");
        let d = time!("0:00:01");
        let e = time!("0:00:00.001");
        let f = time!("0:00:00.000_001");
        let g = time!("0:00:00.000_000_001");

        ben.iter(|| (
            a.to_string(),
            b.to_string(),
            c.to_string(),
            d.to_string(),
            e.to_string(),
            f.to_string(),
            g.to_string(),
        ));
    }

    fn add_duration(ben: &mut Bencher) {
        let t = time!("0:00");
        let dta = 1.milliseconds();
        let dtb = 1.seconds();
        let dtc = 1.minutes();
        let dtd = 1.hours();
        let dte = 1.days();
        ben.iter(|| t + dta + dtb + dtc + dtd + dte);
    }

    fn add_assign_duration(ben: &mut Bencher) {
        let dta = 1.milliseconds();
        let dtb = 1.seconds();
        let dtc = 1.minutes();
        let dtd = 1.hours();
        let dte = 1.days();
        ben.iter_batched_ref(
            || time!("0:00"),
            |time| {
                *time += dta;
                *time += dtb;
                *time += dtc;
                *time += dtd;
                *time += dte;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_duration(ben: &mut Bencher) {
        let t = time!("12:00");

        let dta = 1.milliseconds();
        let dtb = 1.seconds();
        let dtc = 1.minutes();
        let dtd = 1.hours();
        let dte = 1.days();

        ben.iter(|| t - dta - dtb - dtc - dtd - dte);
    }

    fn sub_assign_duration(ben: &mut Bencher) {
        let dta = 1.milliseconds();
        let dtb = 1.seconds();
        let dtc = 1.minutes();
        let dtd = 1.hours();
        let dte = 1.days();

        ben.iter_batched_ref(
            || time!("0:00"),
            |time| {
                *time -= dta;
                *time -= dtb;
                *time -= dtc;
                *time -= dtd;
                *time -= dte;
            },
            BatchSize::SmallInput
        );
    }

    fn add_std_duration(ben: &mut Bencher) {
        let t = time!("0:00");
        let dta = 1.std_milliseconds();
        let dtb = 1.std_seconds();
        let dtc = 1.std_minutes();
        let dtd = 1.std_hours();
        let dte = 1.std_days();
        ben.iter(|| t + dta + dtb + dtc + dtd + dte);
    }

    fn add_assign_std_duration(ben: &mut Bencher) {
        let dta = 1.std_milliseconds();
        let dtb = 1.std_seconds();
        let dtc = 1.std_minutes();
        let dtd = 1.std_hours();
        let dte = 1.std_days();
        ben.iter_batched_ref(
            || time!("0:00"),
            |time| {
                *time += dta;
                *time += dtb;
                *time += dtc;
                *time += dtd;
                *time += dte;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_std_duration(ben: &mut Bencher) {
        let t = time!("12:00");

        let dta = 1.std_milliseconds();
        let dtb = 1.std_seconds();
        let dtc = 1.std_minutes();
        let dtd = 1.std_hours();
        let dte = 1.std_days();

        ben.iter(|| t - dta - dtb - dtc - dtd - dte);
    }

    fn sub_assign_std_duration(ben: &mut Bencher) {
        let dta = 1.std_milliseconds();
        let dtb = 1.std_seconds();
        let dtc = 1.std_minutes();
        let dtd = 1.std_hours();
        let dte = 1.std_days();

        ben.iter_batched_ref(
            || time!("0:00"),
            |time| {
                *time -= dta;
                *time -= dtb;
                *time -= dtc;
                *time -= dtd;
                *time -= dte;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_time(ben: &mut Bencher) {
        let a = time!("0:00");
        let b = time!("1:00");
        let c = time!("0:00:01");
        ben.iter(|| (
            a - c,
            b - a,
            b - c,
        ));
    }

    fn ordering(ben: &mut Bencher) {
        let a = time!("0:00");
        let b = time!("0:00:00.000_000_001");
        let c = time!("0:00:01");
        let d = time!("12:00");
        let e = time!("11:00");
        ben.iter(|| (
            a < b,
            a < c,
            d > e,
            a == b,
        ));
    }
}
