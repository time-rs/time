use criterion::Bencher;
use criterion_cycles_per_byte::CyclesPerByte;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::date;
use time::{Date, Month, Time, Weekday};

setup_benchmark! {
    "Date",

    // region: constructors
    fn from_calendar_date(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Date::from_calendar_date(2019, Month::January, 1));
        ben.iter(|| Date::from_calendar_date(2019, Month::December, 31));
        ben.iter(|| Date::from_calendar_date(2020, Month::January, 1));
        ben.iter(|| Date::from_calendar_date(2020, Month::December, 31));
    }

    fn from_ordinal_date(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Date::from_ordinal_date(2019, 1));
        ben.iter(|| Date::from_ordinal_date(2019, 365));
        ben.iter(|| Date::from_ordinal_date(2020, 1));
        ben.iter(|| Date::from_ordinal_date(2020, 366));
    }

    fn from_iso_week_date(ben: &mut Bencher<'_, CyclesPerByte>) {
        use Weekday::*;
        ben.iter(|| Date::from_iso_week_date(2019, 1, Tuesday));
        ben.iter(|| Date::from_iso_week_date(2020, 1, Tuesday));
        ben.iter(|| Date::from_iso_week_date(2020, 1, Wednesday));
        ben.iter(|| Date::from_iso_week_date(2020, 53, Thursday));
    }

    fn from_julian_day(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Date::from_julian_day(-34_803_190));
    }
    // endregion constructors

    // region: getters
    fn year(ben: &mut Bencher<'_, CyclesPerByte>) {
        let d = date!(2019-002);
        ben.iter(|| d.year());
    }

    fn month(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).month());
        ben.iter(|| date!(2019-02-01).month());
        ben.iter(|| date!(2019-03-01).month());
        ben.iter(|| date!(2019-04-01).month());
        ben.iter(|| date!(2019-05-01).month());
        ben.iter(|| date!(2019-06-01).month());
        ben.iter(|| date!(2019-07-01).month());
        ben.iter(|| date!(2019-08-01).month());
        ben.iter(|| date!(2019-09-01).month());
        ben.iter(|| date!(2019-10-01).month());
        ben.iter(|| date!(2019-11-01).month());
        ben.iter(|| date!(2019-12-01).month());
    }

    fn day(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).day());
        ben.iter(|| date!(2019-02-01).day());
        ben.iter(|| date!(2019-03-01).day());
        ben.iter(|| date!(2019-04-01).day());
        ben.iter(|| date!(2019-05-01).day());
        ben.iter(|| date!(2019-06-01).day());
        ben.iter(|| date!(2019-07-01).day());
        ben.iter(|| date!(2019-08-01).day());
        ben.iter(|| date!(2019-09-01).day());
        ben.iter(|| date!(2019-10-01).day());
        ben.iter(|| date!(2019-11-01).day());
        ben.iter(|| date!(2019-12-01).day());
    }

    fn ordinal(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).ordinal());
        ben.iter(|| date!(2019-02-01).ordinal());
        ben.iter(|| date!(2019-03-01).ordinal());
        ben.iter(|| date!(2019-04-01).ordinal());
        ben.iter(|| date!(2019-05-01).ordinal());
        ben.iter(|| date!(2019-06-01).ordinal());
        ben.iter(|| date!(2019-07-01).ordinal());
        ben.iter(|| date!(2019-08-01).ordinal());
        ben.iter(|| date!(2019-09-01).ordinal());
        ben.iter(|| date!(2019-10-01).ordinal());
        ben.iter(|| date!(2019-11-01).ordinal());
        ben.iter(|| date!(2019-12-01).ordinal());
    }

    fn iso_week(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).iso_week());
        ben.iter(|| date!(2019-02-01).iso_week());
        ben.iter(|| date!(2019-03-01).iso_week());
        ben.iter(|| date!(2019-04-01).iso_week());
        ben.iter(|| date!(2019-05-01).iso_week());
        ben.iter(|| date!(2019-06-01).iso_week());
        ben.iter(|| date!(2019-07-01).iso_week());
        ben.iter(|| date!(2019-08-01).iso_week());
        ben.iter(|| date!(2019-09-01).iso_week());
        ben.iter(|| date!(2019-10-01).iso_week());
        ben.iter(|| date!(2019-11-01).iso_week());
        ben.iter(|| date!(2019-12-01).iso_week());
    }

    fn sunday_based_week(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).sunday_based_week());
        ben.iter(|| date!(2019-02-01).sunday_based_week());
        ben.iter(|| date!(2019-03-01).sunday_based_week());
        ben.iter(|| date!(2019-04-01).sunday_based_week());
        ben.iter(|| date!(2019-05-01).sunday_based_week());
        ben.iter(|| date!(2019-06-01).sunday_based_week());
        ben.iter(|| date!(2019-07-01).sunday_based_week());
        ben.iter(|| date!(2019-08-01).sunday_based_week());
        ben.iter(|| date!(2019-09-01).sunday_based_week());
        ben.iter(|| date!(2019-10-01).sunday_based_week());
        ben.iter(|| date!(2019-11-01).sunday_based_week());
        ben.iter(|| date!(2019-12-01).sunday_based_week());
    }

    fn monday_based_week(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).monday_based_week());
        ben.iter(|| date!(2019-02-01).monday_based_week());
        ben.iter(|| date!(2019-03-01).monday_based_week());
        ben.iter(|| date!(2019-04-01).monday_based_week());
        ben.iter(|| date!(2019-05-01).monday_based_week());
        ben.iter(|| date!(2019-06-01).monday_based_week());
        ben.iter(|| date!(2019-07-01).monday_based_week());
        ben.iter(|| date!(2019-08-01).monday_based_week());
        ben.iter(|| date!(2019-09-01).monday_based_week());
        ben.iter(|| date!(2019-10-01).monday_based_week());
        ben.iter(|| date!(2019-11-01).monday_based_week());
        ben.iter(|| date!(2019-12-01).monday_based_week());
    }

    fn to_calendar_date(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).to_calendar_date());
        ben.iter(|| date!(2019-02-01).to_calendar_date());
        ben.iter(|| date!(2019-03-01).to_calendar_date());
        ben.iter(|| date!(2019-04-01).to_calendar_date());
        ben.iter(|| date!(2019-05-01).to_calendar_date());
        ben.iter(|| date!(2019-06-01).to_calendar_date());
        ben.iter(|| date!(2019-07-01).to_calendar_date());
        ben.iter(|| date!(2019-08-01).to_calendar_date());
        ben.iter(|| date!(2019-09-01).to_calendar_date());
        ben.iter(|| date!(2019-10-01).to_calendar_date());
        ben.iter(|| date!(2019-11-01).to_calendar_date());
        ben.iter(|| date!(2019-12-01).to_calendar_date());
    }

    fn to_ordinal_date(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).to_ordinal_date());
        ben.iter(|| date!(2019-02-01).to_ordinal_date());
        ben.iter(|| date!(2019-03-01).to_ordinal_date());
        ben.iter(|| date!(2019-04-01).to_ordinal_date());
        ben.iter(|| date!(2019-05-01).to_ordinal_date());
        ben.iter(|| date!(2019-06-01).to_ordinal_date());
        ben.iter(|| date!(2019-07-01).to_ordinal_date());
        ben.iter(|| date!(2019-08-01).to_ordinal_date());
        ben.iter(|| date!(2019-09-01).to_ordinal_date());
        ben.iter(|| date!(2019-10-01).to_ordinal_date());
        ben.iter(|| date!(2019-11-01).to_ordinal_date());
        ben.iter(|| date!(2019-12-01).to_ordinal_date());
    }

    fn to_iso_week_date(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).to_iso_week_date());
        ben.iter(|| date!(2019-02-01).to_iso_week_date());
        ben.iter(|| date!(2019-03-01).to_iso_week_date());
        ben.iter(|| date!(2019-04-01).to_iso_week_date());
        ben.iter(|| date!(2019-05-01).to_iso_week_date());
        ben.iter(|| date!(2019-06-01).to_iso_week_date());
        ben.iter(|| date!(2019-07-01).to_iso_week_date());
        ben.iter(|| date!(2019-08-01).to_iso_week_date());
        ben.iter(|| date!(2019-09-01).to_iso_week_date());
        ben.iter(|| date!(2019-10-01).to_iso_week_date());
        ben.iter(|| date!(2019-11-01).to_iso_week_date());
        ben.iter(|| date!(2019-12-01).to_iso_week_date());
    }

    fn weekday(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).weekday());
        ben.iter(|| date!(2019-02-01).weekday());
        ben.iter(|| date!(2019-03-01).weekday());
        ben.iter(|| date!(2019-04-01).weekday());
        ben.iter(|| date!(2019-05-01).weekday());
        ben.iter(|| date!(2019-06-01).weekday());
        ben.iter(|| date!(2019-07-01).weekday());
        ben.iter(|| date!(2019-08-01).weekday());
        ben.iter(|| date!(2019-09-01).weekday());
        ben.iter(|| date!(2019-10-01).weekday());
        ben.iter(|| date!(2019-11-01).weekday());
        ben.iter(|| date!(2019-12-01).weekday());
    }

    fn next_day(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).next_day());
        ben.iter(|| date!(2019-02-01).next_day());
        ben.iter(|| date!(2019-12-31).next_day());
        ben.iter(|| date!(2020-12-31).next_day());
        ben.iter(|| Date::MAX.next_day());
    }

    fn previous_day(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-02).previous_day());
        ben.iter(|| date!(2019-02-01).previous_day());
        ben.iter(|| date!(2020-01-01).previous_day());
        ben.iter(|| date!(2021-01-01).previous_day());
        ben.iter(|| Date::MIN.previous_day());
    }

    fn to_julian_day(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-01).to_julian_day());
        ben.iter(|| date!(2019-02-01).to_julian_day());
        ben.iter(|| date!(2019-03-01).to_julian_day());
        ben.iter(|| date!(2019-04-01).to_julian_day());
        ben.iter(|| date!(2019-05-01).to_julian_day());
        ben.iter(|| date!(2019-06-01).to_julian_day());
        ben.iter(|| date!(2019-07-01).to_julian_day());
        ben.iter(|| date!(2019-08-01).to_julian_day());
        ben.iter(|| date!(2019-09-01).to_julian_day());
        ben.iter(|| date!(2019-10-01).to_julian_day());
        ben.iter(|| date!(2019-11-01).to_julian_day());
        ben.iter(|| date!(2019-12-01).to_julian_day());
    }
    // endregion getters

    // region: attach time
    fn midnight(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(1970-01-01).midnight());
    }

    fn with_time(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(1970-01-01).with_time(Time::MIDNIGHT));
    }

    fn with_hms(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(1970-01-01).with_hms(0, 0, 0));
    }

    fn with_hms_milli(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(1970-01-01).with_hms_milli(0, 0, 0, 0));
    }

    fn with_hms_micro(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(1970-01-01).with_hms_micro(0, 0, 0, 0));
    }

    fn with_hms_nano(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(1970-01-01).with_hms_nano(0, 0, 0, 0));
    }
    // endregion attach time

    // region: trait impls
    fn add(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt = 5.days();
        ben.iter(|| date!(2019-01-01) + dt);
    }

    fn add_std(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt = 5.std_days();
        ben.iter(|| date!(2019-01-01) + dt);
    }

    fn add_assign(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt = 1.days();
        iter_batched_ref!(
            ben,
            || date!(2019-12-31),
            [|date| *date += dt]
        );
    }

    fn add_assign_std(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt = 1.std_days();
        iter_batched_ref!(
            ben,
            || date!(2019-12-31),
            [|date| *date += dt]
        );
    }

    fn sub(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt = 5.days();
        ben.iter(|| date!(2019-01-06) - dt);
    }

    fn sub_std(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt = 5.std_days();
        ben.iter(|| date!(2019-01-06) - dt);
    }

    fn sub_assign(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt = 1.days();
        iter_batched_ref!(
            ben,
            || date!(2019-12-31),
            [|date| *date -= dt]
        );
    }

    fn sub_assign_std(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt = 1.std_days();
        iter_batched_ref!(
            ben,
            || date!(2019-12-31),
            [|date| *date -= dt]
        );
    }

    fn sub_self(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| date!(2019-01-02) - date!(2019-01-01));
    }

    fn partial_ord(ben: &mut Bencher<'_, CyclesPerByte>) {
        let first = date!(2019-01-01);
        let second = date!(2019-01-02);
        ben.iter(|| first.partial_cmp(&first));
        ben.iter(|| first.partial_cmp(&second));
        ben.iter(|| second.partial_cmp(&first));
    }

    fn ord(ben: &mut Bencher<'_, CyclesPerByte>) {
        let first = date!(2019-01-01);
        let second = date!(2019-01-02);
        ben.iter(|| first.cmp(&first));
        ben.iter(|| first.cmp(&second));
        ben.iter(|| second.cmp(&first));
    }
    // endregion trait impls
}
