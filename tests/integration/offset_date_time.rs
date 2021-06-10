use std::cmp::Ordering;
use std::time::{Duration as StdDuration, SystemTime};

use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{date, datetime, offset, time};
use time::{Date, Duration, Month, OffsetDateTime, Weekday};

#[test]
fn now_utc() {
    assert!(OffsetDateTime::now_utc().year() >= 2019);
    assert_eq!(OffsetDateTime::now_utc().offset(), offset!(UTC));
}

#[test]
fn now_local() {
    #[cfg(not(target_family = "unix"))]
    assert!(OffsetDateTime::now_local().is_ok());

    // Include for test coverage.
    #[cfg(target_family = "unix")]
    let _ = OffsetDateTime::now_local();
}

#[test]
fn to_offset() {
    assert_eq!(
        datetime!(2000-01-01 0:00 UTC).to_offset(offset!(-1)).year(),
        1999,
    );

    let sydney = datetime!(2000-01-01 0:00 +11);
    let new_york = sydney.to_offset(offset!(-5));
    let los_angeles = sydney.to_offset(offset!(-8));
    assert_eq!(sydney.hour(), 0);
    assert_eq!(sydney.day(), 1);
    assert_eq!(new_york.hour(), 8);
    assert_eq!(new_york.day(), 31);
    assert_eq!(los_angeles.hour(), 5);
    assert_eq!(los_angeles.day(), 31);
}

#[test]
fn from_unix_timestamp() {
    assert_eq!(
        OffsetDateTime::from_unix_timestamp(0),
        Ok(OffsetDateTime::UNIX_EPOCH),
    );
    assert_eq!(
        OffsetDateTime::from_unix_timestamp(1_546_300_800),
        Ok(datetime!(2019-01-01 0:00 UTC)),
    );
}

#[test]
fn from_unix_timestamp_nanos() {
    assert_eq!(
        OffsetDateTime::from_unix_timestamp_nanos(0),
        Ok(OffsetDateTime::UNIX_EPOCH),
    );
    assert_eq!(
        OffsetDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
        Ok(datetime!(2019-01-01 0:00 UTC)),
    );
}

#[test]
fn offset() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).offset(), offset!(UTC));
    assert_eq!(datetime!(2019-01-01 0:00 +1).offset(), offset!(+1));
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC)
            .to_offset(offset!(+1))
            .offset(),
        offset!(+1),
    );
}

#[test]
fn unix_timestamp() {
    assert_eq!(OffsetDateTime::UNIX_EPOCH.unix_timestamp(), 0);
    assert_eq!(
        OffsetDateTime::UNIX_EPOCH
            .to_offset(offset!(+1))
            .unix_timestamp(),
        0,
    );
    assert_eq!(datetime!(1970-01-01 0:00 -1).unix_timestamp(), 3_600);
}

#[test]
fn unix_timestamp_nanos() {
    assert_eq!(datetime!(1970-01-01 0:00 UTC).unix_timestamp_nanos(), 0);
    assert_eq!(
        datetime!(1970-01-01 1:00 UTC)
            .to_offset(offset!(-1))
            .unix_timestamp_nanos(),
        3_600_000_000_000,
    );
}

#[test]
fn date() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).date(), date!(2019 - 01 - 01));
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC).to_offset(offset!(-1)).date(),
        date!(2018 - 12 - 31),
    );
}

#[test]
fn time() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).time(), time!(0:00));
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC).to_offset(offset!(-1)).time(),
        time!(23:00),
    );
}

#[test]
fn year() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).year(), 2019);
    assert_eq!(
        datetime!(2019-12-31 23:00 UTC)
            .to_offset(offset!(+1))
            .year(),
        2020,
    );
    assert_eq!(datetime!(2020-01-01 0:00 UTC).year(), 2020);
}

#[test]
fn month() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).month(), Month::January);
    assert_eq!(
        datetime!(2019-12-31 23:00 UTC)
            .to_offset(offset!(+1))
            .month(),
        Month::January,
    );
}

#[test]
fn day() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).day(), 1);
    assert_eq!(
        datetime!(2019-12-31 23:00 UTC).to_offset(offset!(+1)).day(),
        1,
    );
}

#[test]
fn ordinal() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).ordinal(), 1);
    assert_eq!(
        datetime!(2019-12-31 23:00 UTC)
            .to_offset(offset!(+1))
            .ordinal(),
        1,
    );
}

#[test]
fn iso_week() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).iso_week(), 1);
    assert_eq!(datetime!(2020-01-01 0:00 UTC).iso_week(), 1);
    assert_eq!(datetime!(2020-12-31 0:00 UTC).iso_week(), 53);
    assert_eq!(datetime!(2021-01-01 0:00 UTC).iso_week(), 53);
}

#[test]
fn sunday_based_week() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).sunday_based_week(), 0);
    assert_eq!(datetime!(2020-01-01 0:00 UTC).sunday_based_week(), 0);
    assert_eq!(datetime!(2020-12-31 0:00 UTC).sunday_based_week(), 52);
    assert_eq!(datetime!(2021-01-01 0:00 UTC).sunday_based_week(), 0);
}

#[test]
fn monday_based_week() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).monday_based_week(), 0);
    assert_eq!(datetime!(2020-01-01 0:00 UTC).monday_based_week(), 0);
    assert_eq!(datetime!(2020-12-31 0:00 UTC).monday_based_week(), 52);
    assert_eq!(datetime!(2021-01-01 0:00 UTC).monday_based_week(), 0);
}

#[test]
fn to_calendar_date() {
    assert_eq!(
        datetime!(2019-01-02 0:00 UTC).to_calendar_date(),
        (2019, Month::January, 2)
    );
}

#[test]
fn to_ordinal_date() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).to_ordinal_date(), (2019, 1));
}

#[test]
fn to_iso_week_date() {
    use Weekday::*;
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC).to_iso_week_date(),
        (2019, 1, Tuesday)
    );
    assert_eq!(
        datetime!(2019-10-04 0:00 UTC).to_iso_week_date(),
        (2019, 40, Friday)
    );
    assert_eq!(
        datetime!(2020-01-01 0:00 UTC).to_iso_week_date(),
        (2020, 1, Wednesday)
    );
    assert_eq!(
        datetime!(2020-12-31 0:00 UTC).to_iso_week_date(),
        (2020, 53, Thursday)
    );
    assert_eq!(
        datetime!(2021-01-01 0:00 UTC).to_iso_week_date(),
        (2020, 53, Friday)
    );
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_eq!(datetime!(2019-01-01 0:00 UTC).weekday(), Tuesday);
    assert_eq!(datetime!(2019-02-01 0:00 UTC).weekday(), Friday);
    assert_eq!(datetime!(2019-03-01 0:00 UTC).weekday(), Friday);
}

#[test]
fn to_julian_day() {
    assert_eq!(
        datetime!(-999_999-01-01 0:00 UTC).to_julian_day(),
        -363_521_074
    );
    assert_eq!(datetime!(-4713-11-24 0:00 UTC).to_julian_day(), 0);
    assert_eq!(datetime!(2000-01-01 0:00 UTC).to_julian_day(), 2_451_545);
    assert_eq!(datetime!(2019-01-01 0:00 UTC).to_julian_day(), 2_458_485);
    assert_eq!(datetime!(2019-12-31 0:00 UTC).to_julian_day(), 2_458_849);
}

#[test]
fn to_hms() {
    assert_eq!(datetime!(2020-01-01 1:02:03 UTC).to_hms(), (1, 2, 3));
}

#[test]
fn to_hms_milli() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004 UTC).to_hms_milli(),
        (1, 2, 3, 4)
    );
}

#[test]
fn to_hms_micro() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004_005 UTC).to_hms_micro(),
        (1, 2, 3, 4_005)
    );
}

#[test]
fn to_hms_nano() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004_005_006 UTC).to_hms_nano(),
        (1, 2, 3, 4_005_006)
    );
}

#[test]
fn hour() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).hour(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59 UTC)
            .to_offset(offset!(-2))
            .hour(),
        21,
    );
}

#[test]
fn minute() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).minute(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59 UTC)
            .to_offset(offset!(+0:30))
            .minute(),
        29,
    );
}

#[test]
fn second() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).second(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59 UTC)
            .to_offset(offset!(+0:00:30))
            .second(),
        29,
    );
}

#[test]
fn millisecond() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).millisecond(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59.999 UTC).millisecond(), 999);
}

#[test]
fn microsecond() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).microsecond(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59.999_999 UTC).microsecond(),
        999_999,
    );
}

#[test]
fn nanosecond() {
    assert_eq!(datetime!(2019-01-01 0:00 UTC).nanosecond(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59.999_999_999 UTC).nanosecond(),
        999_999_999,
    );
}

#[test]
fn replace_time() {
    assert_eq!(
        datetime!(2020-01-01 5:00 UTC).replace_time(time!(12:00)),
        datetime!(2020-01-01 12:00 UTC)
    );
    assert_eq!(
        datetime!(2020-01-01 12:00 -5).replace_time(time!(7:00)),
        datetime!(2020-01-01 7:00 -5)
    );
    assert_eq!(
        datetime!(2020-01-01 0:00 +1).replace_time(time!(12:00)),
        datetime!(2020-01-01 12:00 +1)
    );
}

#[test]
fn replace_date() {
    assert_eq!(
        datetime!(2020-01-01 12:00 UTC).replace_date(date!(2020 - 01 - 30)),
        datetime!(2020-01-30 12:00 UTC)
    );
    assert_eq!(
        datetime!(2020-01-01 0:00 +1).replace_date(date!(2020 - 01 - 30)),
        datetime!(2020-01-30 0:00 +1)
    );
}

#[test]
fn replace_date_time() {
    assert_eq!(
        datetime!(2020-01-01 12:00 UTC).replace_date_time(datetime!(2020-01-30 16:00)),
        datetime!(2020-01-30 16:00 UTC)
    );
    assert_eq!(
        datetime!(2020-01-01 12:00 +1).replace_date_time(datetime!(2020-01-30 0:00)),
        datetime!(2020-01-30 0:00 +1)
    );
}

#[test]
fn replace_offset() {
    assert_eq!(
        datetime!(2020-01-01 0:00 UTC).replace_offset(offset!(-5)),
        datetime!(2020-01-01 0:00 -5)
    );
}

#[test]
fn partial_eq() {
    assert_eq!(
        datetime!(2000-01-01 0:00 UTC).to_offset(offset!(-1)),
        datetime!(2000-01-01 0:00 UTC),
    );
}

#[test]
fn partial_ord() {
    let t1 = datetime!(2019-01-01 0:00 UTC);
    let t2 = datetime!(2019-01-01 0:00 UTC).to_offset(offset!(-1));
    assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Equal));
}

#[test]
fn ord() {
    let t1 = datetime!(2019-01-01 0:00 UTC);
    let t2 = datetime!(2019-01-01 0:00 UTC).to_offset(offset!(-1));
    assert_eq!(t1, t2);

    let t1 = datetime!(2019-01-01 0:00 UTC);
    let t2 = datetime!(2019-01-01 0:00:00.000_000_001 UTC);
    assert!(t2 > t1);
}

#[test]
fn hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    assert_eq!(
        {
            let mut hasher = DefaultHasher::new();
            datetime!(2019-01-01 0:00 UTC).hash(&mut hasher);
            hasher.finish()
        },
        {
            let mut hasher = DefaultHasher::new();
            datetime!(2019-01-01 0:00 UTC)
                .to_offset(offset!(-1))
                .hash(&mut hasher);
            hasher.finish()
        }
    );

    // Ensure that a `PrimitiveDateTime` and `OffsetDateTime` don't collide,
    // even if the UTC time is the same.
    assert_ne!(
        {
            let mut hasher = DefaultHasher::new();
            datetime!(2019-01-01 0:00).hash(&mut hasher);
            hasher.finish()
        },
        {
            let mut hasher = DefaultHasher::new();
            datetime!(2019-01-01 0:00 UTC).hash(&mut hasher);
            hasher.finish()
        }
    );
}

#[test]
fn arithmetic_regression() {
    let val = Date::MIN
        .next_day()
        .expect("date is not maximum")
        .midnight()
        .assume_offset(offset!(+12:00:01));

    assert_eq!(val + Duration::ZERO, val);
    assert_eq!(val - Duration::ZERO, val);
    assert_eq!(val + StdDuration::from_secs(0), val);
    assert_eq!(val - StdDuration::from_secs(0), val);
}

#[test]
fn add_duration() {
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC) + 5.days(),
        datetime!(2019-01-06 0:00 UTC),
    );
    assert_eq!(
        datetime!(2019-12-31 0:00 UTC) + 1.days(),
        datetime!(2020-01-01 0:00 UTC),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59 UTC) + 2.seconds(),
        datetime!(2020-01-01 0:00:01 UTC),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01 UTC) + (-2).seconds(),
        datetime!(2019-12-31 23:59:59 UTC),
    );
    assert_eq!(
        datetime!(1999-12-31 23:00 UTC) + 1.hours(),
        datetime!(2000-01-01 0:00 UTC),
    );
}

#[test]
fn add_std_duration() {
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC) + 5.std_days(),
        datetime!(2019-01-06 0:00 UTC),
    );
    assert_eq!(
        datetime!(2019-12-31 0:00 UTC) + 1.std_days(),
        datetime!(2020-01-01 0:00 UTC),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59 UTC) + 2.std_seconds(),
        datetime!(2020-01-01 0:00:01 UTC),
    );
}

#[test]
fn add_assign_duration() {
    let mut new_years_day_2019 = datetime!(2019-01-01 0:00 UTC);
    new_years_day_2019 += 5.days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-06 0:00 UTC));

    let mut new_years_eve_2020_days = datetime!(2019-12-31 0:00 UTC);
    new_years_eve_2020_days += 1.days();
    assert_eq!(new_years_eve_2020_days, datetime!(2020-01-01 0:00 UTC));

    let mut new_years_eve_2020_seconds = datetime!(2019-12-31 23:59:59 UTC);
    new_years_eve_2020_seconds += 2.seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        datetime!(2020-01-01 0:00:01 UTC)
    );

    let mut new_years_day_2020_seconds = datetime!(2020-01-01 0:00:01 UTC);
    new_years_day_2020_seconds += (-2).seconds();
    assert_eq!(
        new_years_day_2020_seconds,
        datetime!(2019-12-31 23:59:59 UTC)
    );
}

#[test]
fn add_assign_std_duration() {
    let mut new_years_day_2019 = datetime!(2019-01-01 0:00 UTC);
    new_years_day_2019 += 5.std_days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-06 0:00 UTC));

    let mut new_years_eve_2020_days = datetime!(2019-12-31 0:00 UTC);
    new_years_eve_2020_days += 1.std_days();
    assert_eq!(new_years_eve_2020_days, datetime!(2020-01-01 0:00 UTC));

    let mut new_years_eve_2020_seconds = datetime!(2019-12-31 23:59:59 UTC);
    new_years_eve_2020_seconds += 2.std_seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        datetime!(2020-01-01 0:00:01 UTC)
    );
}

#[test]
fn sub_duration() {
    assert_eq!(
        datetime!(2019-01-06 0:00 UTC) - 5.days(),
        datetime!(2019-01-01 0:00 UTC),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00 UTC) - 1.days(),
        datetime!(2019-12-31 0:00 UTC),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01 UTC) - 2.seconds(),
        datetime!(2019-12-31 23:59:59 UTC),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59 UTC) - (-2).seconds(),
        datetime!(2020-01-01 0:00:01 UTC),
    );
    assert_eq!(
        datetime!(1999-12-31 23:00 UTC) - (-1).hours(),
        datetime!(2000-01-01 0:00 UTC),
    );
}

#[test]
fn sub_std_duration() {
    assert_eq!(
        datetime!(2019-01-06 0:00 UTC) - 5.std_days(),
        datetime!(2019-01-01 0:00 UTC),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00 UTC) - 1.std_days(),
        datetime!(2019-12-31 0:00 UTC),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01 UTC) - 2.std_seconds(),
        datetime!(2019-12-31 23:59:59 UTC),
    );
}

#[test]
fn sub_assign_duration() {
    let mut new_years_day_2019 = datetime!(2019-01-06 0:00 UTC);
    new_years_day_2019 -= 5.days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-01 0:00 UTC));

    let mut new_years_day_2020_days = datetime!(2020-01-01 0:00 UTC);
    new_years_day_2020_days -= 1.days();
    assert_eq!(new_years_day_2020_days, datetime!(2019-12-31 0:00 UTC));

    let mut new_years_day_2020_seconds = datetime!(2020-01-01 0:00:01 UTC);
    new_years_day_2020_seconds -= 2.seconds();
    assert_eq!(
        new_years_day_2020_seconds,
        datetime!(2019-12-31 23:59:59 UTC)
    );

    let mut new_years_eve_2020_seconds = datetime!(2019-12-31 23:59:59 UTC);
    new_years_eve_2020_seconds -= (-2).seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        datetime!(2020-01-01 0:00:01 UTC)
    );
}

#[test]
fn sub_assign_std_duration() {
    let mut ny19 = datetime!(2019-01-06 0:00 UTC);
    ny19 -= 5.std_days();
    assert_eq!(ny19, datetime!(2019-01-01 0:00 UTC));

    let mut ny20 = datetime!(2020-01-01 0:00 UTC);
    ny20 -= 1.std_days();
    assert_eq!(ny20, datetime!(2019-12-31 0:00 UTC));

    let mut ny20t = datetime!(2020-01-01 0:00:01 UTC);
    ny20t -= 2.std_seconds();
    assert_eq!(ny20t, datetime!(2019-12-31 23:59:59 UTC));
}

#[test]
fn std_add_duration() {
    assert_eq!(
        SystemTime::from(datetime!(2019-01-01 0:00 UTC)) + 0.seconds(),
        SystemTime::from(datetime!(2019-01-01 0:00 UTC)),
    );
    assert_eq!(
        SystemTime::from(datetime!(2019-01-01 0:00 UTC)) + 5.days(),
        SystemTime::from(datetime!(2019-01-06 0:00 UTC)),
    );
    assert_eq!(
        SystemTime::from(datetime!(2019-12-31 0:00 UTC)) + 1.days(),
        SystemTime::from(datetime!(2020-01-01 0:00 UTC)),
    );
    assert_eq!(
        SystemTime::from(datetime!(2019-12-31 23:59:59 UTC)) + 2.seconds(),
        SystemTime::from(datetime!(2020-01-01 0:00:01 UTC)),
    );
    assert_eq!(
        SystemTime::from(datetime!(2020-01-01 0:00:01 UTC)) + (-2).seconds(),
        SystemTime::from(datetime!(2019-12-31 23:59:59 UTC)),
    );
}

#[test]
fn std_add_assign_duration() {
    let mut new_years_day_2019 = SystemTime::from(datetime!(2019-01-01 0:00 UTC));
    new_years_day_2019 += 5.days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-06 0:00 UTC));

    let mut new_years_eve_2020_days = SystemTime::from(datetime!(2019-12-31 0:00 UTC));
    new_years_eve_2020_days += 1.days();
    assert_eq!(new_years_eve_2020_days, datetime!(2020-01-01 0:00 UTC));

    let mut new_years_eve_2020_seconds = SystemTime::from(datetime!(2019-12-31 23:59:59 UTC));
    new_years_eve_2020_seconds += 2.seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        datetime!(2020-01-01 0:00:01 UTC)
    );

    let mut new_years_day_2020_seconds = SystemTime::from(datetime!(2020-01-01 0:00:01 UTC));
    new_years_day_2020_seconds += (-2).seconds();
    assert_eq!(
        new_years_day_2020_seconds,
        datetime!(2019-12-31 23:59:59 UTC)
    );
}

#[test]
fn std_sub_duration() {
    assert_eq!(
        SystemTime::from(datetime!(2019-01-06 0:00 UTC)) - 5.days(),
        SystemTime::from(datetime!(2019-01-01 0:00 UTC)),
    );
    assert_eq!(
        SystemTime::from(datetime!(2020-01-01 0:00 UTC)) - 1.days(),
        SystemTime::from(datetime!(2019-12-31 0:00 UTC)),
    );
    assert_eq!(
        SystemTime::from(datetime!(2020-01-01 0:00:01 UTC)) - 2.seconds(),
        SystemTime::from(datetime!(2019-12-31 23:59:59 UTC)),
    );
    assert_eq!(
        SystemTime::from(datetime!(2019-12-31 23:59:59 UTC)) - (-2).seconds(),
        SystemTime::from(datetime!(2020-01-01 0:00:01 UTC)),
    );
}

#[test]
fn std_sub_assign_duration() {
    let mut new_years_day_2019 = SystemTime::from(datetime!(2019-01-06 0:00 UTC));
    new_years_day_2019 -= 5.days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-01 0:00 UTC));

    let mut new_years_day_2020 = SystemTime::from(datetime!(2020-01-01 0:00 UTC));
    new_years_day_2020 -= 1.days();
    assert_eq!(new_years_day_2020, datetime!(2019-12-31 0:00 UTC));

    let mut new_years_day_2020_seconds = SystemTime::from(datetime!(2020-01-01 0:00:01 UTC));
    new_years_day_2020_seconds -= 2.seconds();
    assert_eq!(
        new_years_day_2020_seconds,
        datetime!(2019-12-31 23:59:59 UTC)
    );

    let mut new_years_eve_2020_seconds = SystemTime::from(datetime!(2019-12-31 23:59:59 UTC));
    new_years_eve_2020_seconds -= (-2).seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        datetime!(2020-01-01 0:00:01 UTC)
    );
}

#[test]
fn sub_self() {
    assert_eq!(
        datetime!(2019-01-02 0:00 UTC) - datetime!(2019-01-01 0:00 UTC),
        1.days(),
    );
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC) - datetime!(2019-01-02 0:00 UTC),
        (-1).days(),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00 UTC) - datetime!(2019-12-31 0:00 UTC),
        1.days(),
    );
    assert_eq!(
        datetime!(2019-12-31 0:00 UTC) - datetime!(2020-01-01 0:00 UTC),
        (-1).days(),
    );
}

#[test]
fn std_sub() {
    assert_eq!(
        SystemTime::from(datetime!(2019-01-02 0:00 UTC)) - datetime!(2019-01-01 0:00 UTC),
        1.days()
    );
    assert_eq!(
        SystemTime::from(datetime!(2019-01-01 0:00 UTC)) - datetime!(2019-01-02 0:00 UTC),
        (-1).days()
    );
    assert_eq!(
        SystemTime::from(datetime!(2020-01-01 0:00 UTC)) - datetime!(2019-12-31 0:00 UTC),
        1.days()
    );
    assert_eq!(
        SystemTime::from(datetime!(2019-12-31 0:00 UTC)) - datetime!(2020-01-01 0:00 UTC),
        (-1).days()
    );
}

#[test]
fn sub_std() {
    assert_eq!(
        datetime!(2019-01-02 0:00 UTC) - SystemTime::from(datetime!(2019-01-01 0:00 UTC)),
        1.days()
    );
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC) - SystemTime::from(datetime!(2019-01-02 0:00 UTC)),
        (-1).days()
    );
    assert_eq!(
        datetime!(2020-01-01 0:00 UTC) - SystemTime::from(datetime!(2019-12-31 0:00 UTC)),
        1.days()
    );
    assert_eq!(
        datetime!(2019-12-31 0:00 UTC) - SystemTime::from(datetime!(2020-01-01 0:00 UTC)),
        (-1).days()
    );
}

#[test]
fn eq_std() {
    let now_datetime = OffsetDateTime::now_utc();
    let now_systemtime = SystemTime::from(now_datetime);
    assert_eq!(now_datetime, now_systemtime);
}

#[test]
fn std_eq() {
    let now_datetime = OffsetDateTime::now_utc();
    let now_systemtime = SystemTime::from(now_datetime);
    assert_eq!(now_systemtime, now_datetime);
}

#[test]
fn ord_std() {
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC),
        SystemTime::from(datetime!(2019-01-01 0:00 UTC))
    );
    assert!(datetime!(2019-01-01 0:00 UTC) < SystemTime::from(datetime!(2020-01-01 0:00 UTC)));
    assert!(datetime!(2019-01-01 0:00 UTC) < SystemTime::from(datetime!(2019-02-01 0:00 UTC)));
    assert!(datetime!(2019-01-01 0:00 UTC) < SystemTime::from(datetime!(2019-01-02 0:00 UTC)));
    assert!(datetime!(2019-01-01 0:00 UTC) < SystemTime::from(datetime!(2019-01-01 1:00:00 UTC)));
    assert!(datetime!(2019-01-01 0:00 UTC) < SystemTime::from(datetime!(2019-01-01 0:01:00 UTC)));
    assert!(datetime!(2019-01-01 0:00 UTC) < SystemTime::from(datetime!(2019-01-01 0:00:01 UTC)));
    assert!(
        datetime!(2019-01-01 0:00 UTC) < SystemTime::from(datetime!(2019-01-01 0:00:00.001 UTC))
    );
    assert!(datetime!(2020-01-01 0:00 UTC) > SystemTime::from(datetime!(2019-01-01 0:00 UTC)));
    assert!(datetime!(2019-02-01 0:00 UTC) > SystemTime::from(datetime!(2019-01-01 0:00 UTC)));
    assert!(datetime!(2019-01-02 0:00 UTC) > SystemTime::from(datetime!(2019-01-01 0:00 UTC)));
    assert!(datetime!(2019-01-01 1:00:00 UTC) > SystemTime::from(datetime!(2019-01-01 0:00 UTC)));
    assert!(datetime!(2019-01-01 0:01:00 UTC) > SystemTime::from(datetime!(2019-01-01 0:00 UTC)));
    assert!(datetime!(2019-01-01 0:00:01 UTC) > SystemTime::from(datetime!(2019-01-01 0:00 UTC)));
    assert!(
        datetime!(2019-01-01 0:00:00.000_000_001 UTC)
            > SystemTime::from(datetime!(2019-01-01 0:00 UTC))
    );
}

#[test]
fn std_ord() {
    assert_eq!(
        SystemTime::from(datetime!(2019-01-01 0:00 UTC)),
        datetime!(2019-01-01 0:00 UTC)
    );
    assert!(SystemTime::from(datetime!(2019-01-01 0:00 UTC)) < datetime!(2020-01-01 0:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-01 0:00 UTC)) < datetime!(2019-02-01 0:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-01 0:00 UTC)) < datetime!(2019-01-02 0:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-01 0:00 UTC)) < datetime!(2019-01-01 1:00:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-01 0:00 UTC)) < datetime!(2019-01-01 0:01:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-01 0:00 UTC)) < datetime!(2019-01-01 0:00:01 UTC));
    assert!(
        SystemTime::from(datetime!(2019-01-01 0:00 UTC))
            < datetime!(2019-01-01 0:00:00.000_000_001 UTC)
    );
    assert!(SystemTime::from(datetime!(2020-01-01 0:00 UTC)) > datetime!(2019-01-01 0:00 UTC));
    assert!(SystemTime::from(datetime!(2019-02-01 0:00 UTC)) > datetime!(2019-01-01 0:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-02 0:00 UTC)) > datetime!(2019-01-01 0:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-01 1:00:00 UTC)) > datetime!(2019-01-01 0:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-01 0:01:00 UTC)) > datetime!(2019-01-01 0:00 UTC));
    assert!(SystemTime::from(datetime!(2019-01-01 0:00:01 UTC)) > datetime!(2019-01-01 0:00 UTC));
    assert!(
        SystemTime::from(datetime!(2019-01-01 0:00:00.001 UTC)) > datetime!(2019-01-01 0:00 UTC)
    );
}

#[test]
fn from_std() {
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH),
        OffsetDateTime::UNIX_EPOCH
    );
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH - 1.std_days()),
        OffsetDateTime::UNIX_EPOCH - 1.days()
    );
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH + 1.std_days()),
        OffsetDateTime::UNIX_EPOCH + 1.days()
    );
}

#[test]
fn to_std() {
    assert_eq!(
        SystemTime::from(OffsetDateTime::UNIX_EPOCH),
        SystemTime::UNIX_EPOCH
    );
    assert_eq!(
        SystemTime::from(OffsetDateTime::UNIX_EPOCH + 1.days()),
        SystemTime::UNIX_EPOCH + 1.std_days()
    );
    assert_eq!(
        SystemTime::from(OffsetDateTime::UNIX_EPOCH - 1.days()),
        SystemTime::UNIX_EPOCH - 1.std_days()
    );
}
