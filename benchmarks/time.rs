use criterion::Bencher;
use criterion_cycles_per_byte::CyclesPerByte;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::time;
use time::Time;

setup_benchmark! {
    "Time",

    // region: constructors
    fn from_hms(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::from_hms(1, 2, 3));
    }

    fn from_hms_milli(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::from_hms_milli(1, 2, 3, 4));
    }

    fn from_hms_micro(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::from_hms_micro(1, 2, 3, 4));
    }

    fn from_hms_nano(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::from_hms_nano(1, 2, 3, 4));
    }
    // endregion constructors

    // region: getters
    fn as_hms(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.as_hms());
    }

    fn as_hms_milli(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.as_hms_milli());
    }

    fn as_hms_micro(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.as_hms_micro());
    }

    fn as_hms_nano(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.as_hms_nano());
    }

    fn hour(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.hour());
    }

    fn minute(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.minute());
    }

    fn second(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.second());
    }

    fn millisecond(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.millisecond());
    }

    fn microsecond(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.microsecond());
    }

    fn nanosecond(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT.nanosecond());
    }
    // endregion getters

    // region: trait impls
    fn add_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        ben.iter(|| Time::MIDNIGHT + a);
        ben.iter(|| Time::MIDNIGHT + b);
        ben.iter(|| Time::MIDNIGHT + c);
        ben.iter(|| Time::MIDNIGHT + d);
        ben.iter(|| Time::MIDNIGHT + e);
    }

    fn add_assign_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time += a,
                |time| *time += b,
                |time| *time += c,
                |time| *time += d,
                |time| *time += e,
            ]
        );
    }

    fn sub_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        ben.iter(|| Time::MIDNIGHT - a);
        ben.iter(|| Time::MIDNIGHT - b);
        ben.iter(|| Time::MIDNIGHT - c);
        ben.iter(|| Time::MIDNIGHT - d);
        ben.iter(|| Time::MIDNIGHT - e);
    }

    fn sub_assign_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time -= a,
                |time| *time -= b,
                |time| *time -= c,
                |time| *time -= d,
                |time| *time -= e,
            ]
        );
    }

    fn add_std_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        ben.iter(|| Time::MIDNIGHT + a);
        ben.iter(|| Time::MIDNIGHT + b);
        ben.iter(|| Time::MIDNIGHT + c);
        ben.iter(|| Time::MIDNIGHT + d);
        ben.iter(|| Time::MIDNIGHT + e);
    }

    fn add_assign_std_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time += a,
                |time| *time += b,
                |time| *time += c,
                |time| *time += d,
                |time| *time += e,
            ]
        );
    }

    fn sub_std_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        ben.iter(|| Time::MIDNIGHT - a);
        ben.iter(|| Time::MIDNIGHT - b);
        ben.iter(|| Time::MIDNIGHT - c);
        ben.iter(|| Time::MIDNIGHT - d);
        ben.iter(|| Time::MIDNIGHT - e);
    }

    fn sub_assign_std_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time -= a,
                |time| *time -= b,
                |time| *time -= c,
                |time| *time -= d,
                |time| *time -= e,
            ]
        );
    }

    fn sub_time(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT - time!(0:00:01));
        ben.iter(|| time!(1:00) - Time::MIDNIGHT);
        ben.iter(|| time!(1:00) - time!(0:00:01));
    }

    fn ordering(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Time::MIDNIGHT < time!(0:00:00.000_000_001));
        ben.iter(|| Time::MIDNIGHT < time!(0:00:01));
        ben.iter(|| time!(12:00) > time!(11:00));
        ben.iter(|| Time::MIDNIGHT == time!(0:00:00.000_000_001));
    }
    // endregion trait impls
}
