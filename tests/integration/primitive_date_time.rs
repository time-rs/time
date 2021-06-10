use std::cmp::Ordering;

use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{date, datetime, offset, time};
use time::{Month, PrimitiveDateTime, Weekday};

#[test]
fn new() {
    assert_eq!(
        PrimitiveDateTime::new(date!(2019 - 01 - 01), time!(0:00)),
        datetime!(2019-01-01 0:00),
    );
}

#[test]
fn date() {
    assert_eq!(datetime!(2019-01-01 0:00).date(), date!(2019 - 01 - 01));
}

#[test]
fn time() {
    assert_eq!(datetime!(2019-01-01 0:00).time(), time!(0:00));
}

#[test]
fn year() {
    assert_eq!(datetime!(2019-01-01 0:00).year(), 2019);
    assert_eq!(datetime!(2019-12-31 0:00).year(), 2019);
    assert_eq!(datetime!(2020-01-01 0:00).year(), 2020);
}

#[test]
fn month() {
    assert_eq!(datetime!(2019-01-01 0:00).month(), Month::January);
    assert_eq!(datetime!(2019-12-31 0:00).month(), Month::December);
}

#[test]
fn day() {
    assert_eq!(datetime!(2019-01-01 0:00).day(), 1);
    assert_eq!(datetime!(2019-12-31 0:00).day(), 31);
}

#[test]
fn ordinal() {
    assert_eq!(datetime!(2019-01-01 0:00).ordinal(), 1);
    assert_eq!(datetime!(2019-12-31 0:00).ordinal(), 365);
}

#[test]
fn iso_week() {
    assert_eq!(datetime!(2019-01-01 0:00).iso_week(), 1);
    assert_eq!(datetime!(2019-10-04 0:00).iso_week(), 40);
    assert_eq!(datetime!(2020-01-01 0:00).iso_week(), 1);
    assert_eq!(datetime!(2020-12-31 0:00).iso_week(), 53);
    assert_eq!(datetime!(2021-01-01 0:00).iso_week(), 53);
}

#[test]
fn sunday_based_week() {
    assert_eq!(datetime!(2019-01-01 0:00).sunday_based_week(), 0);
    assert_eq!(datetime!(2020-01-01 0:00).sunday_based_week(), 0);
    assert_eq!(datetime!(2020-12-31 0:00).sunday_based_week(), 52);
    assert_eq!(datetime!(2021-01-01 0:00).sunday_based_week(), 0);
}

#[test]
fn monday_based_week() {
    assert_eq!(datetime!(2019-01-01 0:00).monday_based_week(), 0);
    assert_eq!(datetime!(2020-01-01 0:00).monday_based_week(), 0);
    assert_eq!(datetime!(2020-12-31 0:00).monday_based_week(), 52);
    assert_eq!(datetime!(2021-01-01 0:00).monday_based_week(), 0);
}

#[test]
fn to_calendar_date() {
    assert_eq!(
        datetime!(2019-01-02 0:00).to_calendar_date(),
        (2019, Month::January, 2)
    );
}

#[test]
fn to_ordinal_date() {
    assert_eq!(datetime!(2019-01-01 0:00).to_ordinal_date(), (2019, 1));
}

#[test]
fn to_iso_week_date() {
    use Weekday::*;
    assert_eq!(
        datetime!(2019-01-01 0:00).to_iso_week_date(),
        (2019, 1, Tuesday)
    );
    assert_eq!(
        datetime!(2019-10-04 0:00).to_iso_week_date(),
        (2019, 40, Friday)
    );
    assert_eq!(
        datetime!(2020-01-01 0:00).to_iso_week_date(),
        (2020, 1, Wednesday)
    );
    assert_eq!(
        datetime!(2020-12-31 0:00).to_iso_week_date(),
        (2020, 53, Thursday)
    );
    assert_eq!(
        datetime!(2021-01-01 0:00).to_iso_week_date(),
        (2020, 53, Friday)
    );
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_eq!(datetime!(2019-01-01 0:00).weekday(), Tuesday);
    assert_eq!(datetime!(2019-02-01 0:00).weekday(), Friday);
    assert_eq!(datetime!(2019-03-01 0:00).weekday(), Friday);
    assert_eq!(datetime!(2019-04-01 0:00).weekday(), Monday);
    assert_eq!(datetime!(2019-05-01 0:00).weekday(), Wednesday);
    assert_eq!(datetime!(2019-06-01 0:00).weekday(), Saturday);
    assert_eq!(datetime!(2019-07-01 0:00).weekday(), Monday);
    assert_eq!(datetime!(2019-08-01 0:00).weekday(), Thursday);
    assert_eq!(datetime!(2019-09-01 0:00).weekday(), Sunday);
    assert_eq!(datetime!(2019-10-01 0:00).weekday(), Tuesday);
    assert_eq!(datetime!(2019-11-01 0:00).weekday(), Friday);
    assert_eq!(datetime!(2019-12-01 0:00).weekday(), Sunday);
}

#[test]
fn to_julian_day() {
    assert_eq!(datetime!(-999_999-01-01 0:00).to_julian_day(), -363_521_074);
    assert_eq!(datetime!(-4713-11-24 0:00).to_julian_day(), 0);
    assert_eq!(datetime!(2000-01-01 0:00).to_julian_day(), 2_451_545);
    assert_eq!(datetime!(2019-01-01 0:00).to_julian_day(), 2_458_485);
    assert_eq!(datetime!(2019-12-31 0:00).to_julian_day(), 2_458_849);
}

#[test]
fn as_hms() {
    assert_eq!(datetime!(2020-01-01 1:02:03).as_hms(), (1, 2, 3));
}

#[test]
fn as_hms_milli() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004).as_hms_milli(),
        (1, 2, 3, 4)
    );
}

#[test]
fn as_hms_micro() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004_005).as_hms_micro(),
        (1, 2, 3, 4_005)
    );
}

#[test]
fn as_hms_nano() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004_005_006).as_hms_nano(),
        (1, 2, 3, 4_005_006)
    );
}

#[test]
fn hour() {
    assert_eq!(datetime!(2019-01-01 0:00).hour(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59).hour(), 23);
}

#[test]
fn minute() {
    assert_eq!(datetime!(2019-01-01 0:00).minute(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59).minute(), 59);
}

#[test]
fn second() {
    assert_eq!(datetime!(2019-01-01 0:00).second(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59).second(), 59);
}

#[test]
fn millisecond() {
    assert_eq!(datetime!(2019-01-01 0:00).millisecond(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59.999).millisecond(), 999);
}

#[test]
fn microsecond() {
    assert_eq!(datetime!(2019-01-01 0:00).microsecond(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59.999_999).microsecond(),
        999_999
    );
}

#[test]
fn nanosecond() {
    assert_eq!(datetime!(2019-01-01 0:00).nanosecond(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59.999_999_999).nanosecond(),
        999_999_999
    );
}

#[test]
fn assume_offset() {
    assert_eq!(
        datetime!(2019-01-01 0:00)
            .assume_offset(offset!(UTC))
            .unix_timestamp(),
        1_546_300_800,
    );
    assert_eq!(
        datetime!(2019-01-01 0:00)
            .assume_offset(offset!(-1))
            .unix_timestamp(),
        1_546_304_400,
    );
}

#[test]
fn assume_utc() {
    assert_eq!(
        datetime!(2019-01-01 0:00).assume_utc().unix_timestamp(),
        1_546_300_800,
    );
}

#[test]
fn replace_time() {
    assert_eq!(
        datetime!(2020-01-01 12:00).replace_time(time!(5:00)),
        datetime!(2020-01-01 5:00)
    );
}

#[test]
fn replace_date() {
    assert_eq!(
        datetime!(2020-01-01 12:00).replace_date(date!(2020 - 01 - 30)),
        datetime!(2020-01-30 12:00)
    );
}

#[test]
fn add_duration() {
    assert_eq!(
        datetime!(2019-01-01 0:00) + 5.days(),
        datetime!(2019-01-06 0:00),
    );
    assert_eq!(
        datetime!(2019-12-31 0:00) + 1.days(),
        datetime!(2020-01-01 0:00),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59) + 2.seconds(),
        datetime!(2020-01-01 0:00:01),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01) + (-2).seconds(),
        datetime!(2019-12-31 23:59:59),
    );
    assert_eq!(
        datetime!(1999-12-31 23:00) + 1.hours(),
        datetime!(2000-01-01 0:00),
    );
}

#[test]
fn add_std_duration() {
    assert_eq!(
        datetime!(2019-01-01 0:00) + 5.std_days(),
        datetime!(2019-01-06 0:00),
    );
    assert_eq!(
        datetime!(2019-12-31 0:00) + 1.std_days(),
        datetime!(2020-01-01 0:00),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59) + 2.std_seconds(),
        datetime!(2020-01-01 0:00:01),
    );
}

#[test]
fn add_assign_duration() {
    let mut new_years_day_2019 = datetime!(2019-01-01 0:00);
    new_years_day_2019 += 5.days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-06 0:00));

    let mut new_years_eve_2020_days = datetime!(2019-12-31 0:00);
    new_years_eve_2020_days += 1.days();
    assert_eq!(new_years_eve_2020_days, datetime!(2020-01-01 0:00));

    let mut new_years_eve_2020_seconds = datetime!(2019-12-31 23:59:59);
    new_years_eve_2020_seconds += 2.seconds();
    assert_eq!(new_years_eve_2020_seconds, datetime!(2020-01-01 0:00:01));

    let mut new_years_day_2020_days = datetime!(2020-01-01 0:00:01);
    new_years_day_2020_days += (-2).seconds();
    assert_eq!(new_years_day_2020_days, datetime!(2019-12-31 23:59:59));
}

#[test]
fn add_assign_std_duration() {
    let mut ny19 = datetime!(2019-01-01 0:00);
    ny19 += 5.std_days();
    assert_eq!(ny19, datetime!(2019-01-06 0:00));

    let mut nye20 = datetime!(2019-12-31 0:00);
    nye20 += 1.std_days();
    assert_eq!(nye20, datetime!(2020-01-01 0:00));

    let mut nye20t = datetime!(2019-12-31 23:59:59);
    nye20t += 2.std_seconds();
    assert_eq!(nye20t, datetime!(2020-01-01 0:00:01));
}

#[test]
fn sub_duration() {
    assert_eq!(
        datetime!(2019-01-06 0:00) - 5.days(),
        datetime!(2019-01-01 0:00),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00) - 1.days(),
        datetime!(2019-12-31 0:00),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01) - 2.seconds(),
        datetime!(2019-12-31 23:59:59),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59) - (-2).seconds(),
        datetime!(2020-01-01 0:00:01),
    );
    assert_eq!(
        datetime!(1999-12-31 23:00) - (-1).hours(),
        datetime!(2000-01-01 0:00),
    );
}

#[test]
fn sub_std_duration() {
    assert_eq!(
        datetime!(2019-01-06 0:00) - 5.std_days(),
        datetime!(2019-01-01 0:00),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00) - 1.std_days(),
        datetime!(2019-12-31 0:00),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01) - 2.std_seconds(),
        datetime!(2019-12-31 23:59:59),
    );
}

#[test]
fn sub_assign_duration() {
    let mut new_years_day_2019 = datetime!(2019-01-06 0:00);
    new_years_day_2019 -= 5.days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-01 0:00));

    let mut new_years_day_2020_days = datetime!(2020-01-01 0:00);
    new_years_day_2020_days -= 1.days();
    assert_eq!(new_years_day_2020_days, datetime!(2019-12-31 0:00));

    let mut new_years_day_2020_seconds = datetime!(2020-01-01 0:00:01);
    new_years_day_2020_seconds -= 2.seconds();
    assert_eq!(new_years_day_2020_seconds, datetime!(2019-12-31 23:59:59));

    let mut new_years_eve_2020_seconds = datetime!(2019-12-31 23:59:59);
    new_years_eve_2020_seconds -= (-2).seconds();
    assert_eq!(new_years_eve_2020_seconds, datetime!(2020-01-01 0:00:01));
}

#[test]
fn sub_assign_std_duration() {
    let mut ny19 = datetime!(2019-01-06 0:00);
    ny19 -= 5.std_days();
    assert_eq!(ny19, datetime!(2019-01-01 0:00));

    let mut ny20 = datetime!(2020-01-01 0:00);
    ny20 -= 1.std_days();
    assert_eq!(ny20, datetime!(2019-12-31 0:00));

    let mut ny20t = datetime!(2020-01-01 0:00:01);
    ny20t -= 2.std_seconds();
    assert_eq!(ny20t, datetime!(2019-12-31 23:59:59));
}

#[test]
fn sub_datetime() {
    assert_eq!(
        datetime!(2019-01-02 0:00) - datetime!(2019-01-01 0:00),
        1.days()
    );
    assert_eq!(
        datetime!(2019-01-01 0:00) - datetime!(2019-01-02 0:00),
        (-1).days()
    );
    assert_eq!(
        datetime!(2020-01-01 0:00) - datetime!(2019-12-31 0:00),
        1.days()
    );
    assert_eq!(
        datetime!(2019-12-31 0:00) - datetime!(2020-01-01 0:00),
        (-1).days()
    );
}

#[test]
fn ord() {
    use Ordering::*;
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Equal)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2020-01-01 0:00)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-02-01 0:00)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-02 0:00)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 1:00)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:01)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00:01)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00:00.000_000_001)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2020-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-02-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-02 0:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-01 1:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-01 0:01).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00:01).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00:00.000_000_001).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
}
