use criterion::{BatchSize, Bencher};
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::time;
use time::Time;

setup_benchmark! {
    "Time",

    // region: constructors
    fn from_hms(ben: &mut Bencher) {
        ben.iter(|| Time::from_hms(1, 2, 3));
    }

    fn from_hms_milli(ben: &mut Bencher) {
        ben.iter(|| Time::from_hms_milli(1, 2, 3, 4));
    }

    fn from_hms_micro(ben: &mut Bencher) {
        ben.iter(|| Time::from_hms_micro(1, 2, 3, 4));
    }

    fn from_hms_nano(ben: &mut Bencher) {
        ben.iter(|| Time::from_hms_nano(1, 2, 3, 4))
    }
    // endregion constructors

    // region: getters
    fn as_hms(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.as_hms());
    }

    fn as_hms_milli(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.as_hms_milli());
    }

    fn as_hms_micro(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.as_hms_micro());
    }

    fn as_hms_nano(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.as_hms_nano());
    }

    fn hour(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.hour());
    }

    fn minute(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.minute());
    }

    fn second(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.second());
    }

    fn millisecond(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.millisecond());
    }

    fn microsecond(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.microsecond());
    }

    fn nanosecond(ben: &mut Bencher) {
        ben.iter(|| Time::MIDNIGHT.nanosecond());
    }
    // endregion getters

    // region: trait impls
    fn add_duration(ben: &mut Bencher) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        ben.iter(|| Time::MIDNIGHT + a + b + c + d + e);
    }

    fn add_assign_duration(ben: &mut Bencher) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        ben.iter_batched_ref(
            || Time::MIDNIGHT,
            |time| {
                *time += a;
                *time += b;
                *time += c;
                *time += d;
                *time += e;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_duration(ben: &mut Bencher) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();

        ben.iter(|| Time::MIDNIGHT - a - b - c - d - e);
    }

    fn sub_assign_duration(ben: &mut Bencher) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();

        ben.iter_batched_ref(
            || Time::MIDNIGHT,
            |time| {
                *time -= a;
                *time -= b;
                *time -= c;
                *time -= d;
                *time -= e;
            },
            BatchSize::SmallInput
        );
    }

    fn add_std_duration(ben: &mut Bencher) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        ben.iter(|| Time::MIDNIGHT + a + b + c + d + e);
    }

    fn add_assign_std_duration(ben: &mut Bencher) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        ben.iter_batched_ref(
            || Time::MIDNIGHT,
            |time| {
                *time += a;
                *time += b;
                *time += c;
                *time += d;
                *time += e;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_std_duration(ben: &mut Bencher) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();

        ben.iter(|| Time::MIDNIGHT - a - b - c - d - e);
    }

    fn sub_assign_std_duration(ben: &mut Bencher) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();

        ben.iter_batched_ref(
            || Time::MIDNIGHT,
            |time| {
                *time -= a;
                *time -= b;
                *time -= c;
                *time -= d;
                *time -= e;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_time(ben: &mut Bencher) {
        ben.iter(|| (
            Time::MIDNIGHT - time!("0:00:01"),
            time!("1:00") - Time::MIDNIGHT,
            time!("1:00") - time!("0:00:01"),
        ));
    }

    fn ordering(ben: &mut Bencher) {
        ben.iter(|| (
            Time::MIDNIGHT < time!("0:00:00.000_000_001"),
            Time::MIDNIGHT < time!("0:00:01"),
            time!("12:00") > time!("11:00"),
            Time::MIDNIGHT == time!("0:00:00.000_000_001"),
        ));
    }
    // endregion trait impls
}
