use std::cmp::Ordering;
#[cfg(feature = "std")]
use std::time::SystemTime;
#[cfg(feature = "alloc")]
use time::{error, Format};
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    OffsetDateTime, Weekday,
};
use time_macros::{date, datetime, offset, time};

#[test]
#[cfg(feature = "std")]
fn now_utc() {
    assert!(OffsetDateTime::now_utc().year() >= 2019);
    assert_eq!(OffsetDateTime::now_utc().offset(), offset!("UTC"));
}

#[test]
#[cfg(feature = "local-offset")]
fn now_local() {
    assert!(OffsetDateTime::now_local().is_ok());
}

#[test]
fn to_offset() {
    assert_eq!(
        datetime!("2000-01-01 0:00 UTC")
            .to_offset(offset!("-1"))
            .year(),
        1999,
    );

    let sydney = datetime!("2000-01-01 0:00 +11");
    let new_york = sydney.to_offset(offset!("-5"));
    let los_angeles = sydney.to_offset(offset!("-8"));
    assert_eq!(sydney.hour(), 0);
    assert_eq!(sydney.day(), 1);
    assert_eq!(new_york.hour(), 8);
    assert_eq!(new_york.day(), 31);
    assert_eq!(los_angeles.hour(), 5);
    assert_eq!(los_angeles.day(), 31);
}

#[test]
fn unix_epoch() {
    assert_eq!(
        OffsetDateTime::unix_epoch(),
        datetime!("1970-01-01 0:00 UTC"),
    );
}

#[test]
fn from_unix_timestamp() {
    assert_eq!(
        OffsetDateTime::from_unix_timestamp(0),
        Ok(OffsetDateTime::unix_epoch()),
    );
    assert_eq!(
        OffsetDateTime::from_unix_timestamp(1_546_300_800),
        Ok(datetime!("2019-01-01 0:00 UTC")),
    );
}

#[test]
fn from_unix_timestamp_nanos() {
    assert_eq!(
        OffsetDateTime::from_unix_timestamp_nanos(0),
        Ok(OffsetDateTime::unix_epoch()),
    );
    assert_eq!(
        OffsetDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
        Ok(datetime!("2019-01-01 0:00 UTC")),
    );
}

#[test]
fn offset() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").offset(), offset!("UTC"));
    assert_eq!(datetime!("2019-01-01 0:00 +1").offset(), offset!("+1"));
    assert_eq!(
        datetime!("2019-01-01 0:00 UTC")
            .to_offset(offset!("+1"))
            .offset(),
        offset!("+1"),
    );
}

#[test]
fn unix_timestamp() {
    assert_eq!(OffsetDateTime::unix_epoch().unix_timestamp(), 0);
    assert_eq!(
        OffsetDateTime::unix_epoch()
            .to_offset(offset!("+1"))
            .unix_timestamp(),
        0,
    );
    assert_eq!(datetime!("1970-01-01 0:00 -1").unix_timestamp(), 3_600);
}

#[test]
fn unix_timestamp_nanos() {
    assert_eq!(datetime!("1970-01-01 0:00 UTC").unix_timestamp_nanos(), 0);
    assert_eq!(
        datetime!("1970-01-01 1:00 UTC")
            .to_offset(offset!("-1"))
            .unix_timestamp_nanos(),
        3_600_000_000_000,
    );
}

#[test]
fn date() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").date(), date!("2019-01-01"));
    assert_eq!(
        datetime!("2019-01-01 0:00 UTC")
            .to_offset(offset!("-1"))
            .date(),
        date!("2018-12-31"),
    );
}

#[test]
fn time() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").time(), time!("0:00"));
    assert_eq!(
        datetime!("2019-01-01 0:00 UTC")
            .to_offset(offset!("-1"))
            .time(),
        time!("23:00"),
    );
}

#[test]
fn year() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").year(), 2019);
    assert_eq!(
        datetime!("2019-12-31 23:00 UTC")
            .to_offset(offset!("+1"))
            .year(),
        2020,
    );
    assert_eq!(datetime!("2020-01-01 0:00 UTC").year(), 2020);
}

#[test]
fn month() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").month(), 1);
    assert_eq!(
        datetime!("2019-12-31 23:00 UTC")
            .to_offset(offset!("+1"))
            .month(),
        1,
    );
}

#[test]
fn day() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").day(), 1);
    assert_eq!(
        datetime!("2019-12-31 23:00 UTC")
            .to_offset(offset!("+1"))
            .day(),
        1,
    );
}

#[test]
fn month_day() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").month_day(), (1, 1));
    assert_eq!(
        datetime!("2019-12-31 23:00 UTC")
            .to_offset(offset!("+1"))
            .month_day(),
        (1, 1),
    );
}

#[test]
fn ordinal() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").ordinal(), 1);
    assert_eq!(
        datetime!("2019-12-31 23:00 UTC")
            .to_offset(offset!("+1"))
            .ordinal(),
        1,
    );
}

#[test]
fn iso_year_week() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").iso_year_week(), (2019, 1));
    assert_eq!(datetime!("2019-10-04 0:00 UTC").iso_year_week(), (2019, 40));
    assert_eq!(datetime!("2020-01-01 0:00 UTC").iso_year_week(), (2020, 1));
    assert_eq!(datetime!("2020-12-31 0:00 UTC").iso_year_week(), (2020, 53));
    assert_eq!(datetime!("2021-01-01 0:00 UTC").iso_year_week(), (2020, 53));
}

#[test]
fn week() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").week(), 1);
    assert_eq!(datetime!("2020-01-01 0:00 UTC").week(), 1);
    assert_eq!(datetime!("2020-12-31 0:00 UTC").week(), 53);
    assert_eq!(datetime!("2021-01-01 0:00 UTC").week(), 53);
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_eq!(datetime!("2019-01-01 0:00 UTC").weekday(), Tuesday);
    assert_eq!(datetime!("2019-02-01 0:00 UTC").weekday(), Friday);
    assert_eq!(datetime!("2019-03-01 0:00 UTC").weekday(), Friday);
}

#[test]
fn hour() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").hour(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59 UTC")
            .to_offset(offset!("-2"))
            .hour(),
        21,
    );
}

#[test]
fn minute() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").minute(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59 UTC")
            .to_offset(offset!("+0:30"))
            .minute(),
        29,
    );
}

#[test]
fn second() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").second(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59 UTC")
            .to_offset(offset!("+0:00:30"))
            .second(),
        29,
    );
}

#[test]
fn millisecond() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").millisecond(), 0);
    assert_eq!(datetime!("2019-01-01 23:59:59.999 UTC").millisecond(), 999);
}

#[test]
fn microsecond() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").microsecond(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59.999_999 UTC").microsecond(),
        999_999,
    );
}

#[test]
fn nanosecond() {
    assert_eq!(datetime!("2019-01-01 0:00 UTC").nanosecond(), 0);
    assert_eq!(
        datetime!("2019-01-01 23:59:59.999_999_999 UTC").nanosecond(),
        999_999_999,
    );
}

#[test]
fn replace_time() {
    assert_eq!(
        datetime!("2020-01-01 5:00 UTC").replace_time(time!("12:00")),
        datetime!("2020-01-01 12:00 UTC")
    );
    assert_eq!(
        datetime!("2020-01-01 12:00 -5").replace_time(time!("7:00")),
        datetime!("2020-01-01 7:00 -5")
    );
    assert_eq!(
        datetime!("2020-01-01 0:00 +1").replace_time(time!("12:00")),
        datetime!("2020-01-01 12:00 +1")
    );
}

#[test]
fn replace_date() {
    assert_eq!(
        datetime!("2020-01-01 12:00 UTC").replace_date(date!("2020-01-30")),
        datetime!("2020-01-30 12:00 UTC")
    );
    assert_eq!(
        datetime!("2020-01-01 0:00 +1").replace_date(date!("2020-01-30")),
        datetime!("2020-01-30 0:00 +1")
    );
}

#[test]
fn replace_date_time() {
    assert_eq!(
        datetime!("2020-01-01 12:00 UTC").replace_date_time(datetime!("2020-01-30 16:00")),
        datetime!("2020-01-30 16:00 UTC")
    );
    assert_eq!(
        datetime!("2020-01-01 12:00 +1").replace_date_time(datetime!("2020-01-30 0:00")),
        datetime!("2020-01-30 0:00 +1")
    );
}

#[test]
fn replace_offset() {
    assert_eq!(
        datetime!("2020-01-01 0:00 UTC").replace_offset(offset!("-5")),
        datetime!("2020-01-01 0:00 -5")
    );
}

#[test]
#[cfg(feature = "alloc")]
fn format() {
    assert_eq!(
        datetime!("2019-01-02 0:00 UTC").format("%F %r %z"),
        "2019-01-02 12:00:00 am +0000",
    );
    assert_eq!(
        datetime!("2019-01-02 3:04:05.678_901_234 +6:07").format(Format::Rfc3339),
        "2019-01-02T03:04:05+06:07"
    );
}

#[test]
#[cfg(feature = "alloc")]
fn parse() {
    assert_eq!(
        OffsetDateTime::parse("2019-01-02 00:00:00 +0000", "%F %T %z"),
        Ok(datetime!("2019-01-02 0:00 UTC")),
    );
    assert_eq!(
        OffsetDateTime::parse("2019-002 23:59:59 +0000", "%Y-%j %T %z"),
        Ok(datetime!("2019-002 23:59:59 UTC"))
    );
    assert_eq!(
        OffsetDateTime::parse("2019-W01-3 12:00:00 pm +0000", "%G-W%V-%u %r %z"),
        Ok(datetime!("2019-002 12:00 UTC"))
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02 03:04:05 +0600", "%F %T %z"),
        Ok(datetime!("2019-01-02 3:04:05 +6"))
    );
    assert_eq!(
        OffsetDateTime::parse("2020-09-08T08:44:31+02:30", Format::Rfc3339),
        Ok(datetime!("2020-09-08 08:44:31 +2:30"))
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05.678901234+05:06", Format::Rfc3339),
        Ok(datetime!("2019-01-02 3:04:05.678_901_234 +5:06"))
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05.678901234Z", Format::Rfc3339),
        Ok(datetime!("2019-01-02 3:04:05.678_901_234 UTC"))
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05/", Format::Rfc3339),
        Err(error::Parse::UnexpectedCharacter {
            actual: '/',
            expected: '+'
        })
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05", Format::Rfc3339),
        Err(error::Parse::UnexpectedEndOfString)
    );
    assert_eq!(
        OffsetDateTime::parse("2019-01-02T03:04:05.", Format::Rfc3339),
        Err(error::Parse::InvalidNanosecond)
    );
}

#[test]
fn partial_eq() {
    assert_eq!(
        datetime!("2000-01-01 0:00 UTC").to_offset(offset!("-1")),
        datetime!("2000-01-01 0:00 UTC"),
    );
}

#[test]
fn partial_ord() {
    let t1 = datetime!("2019-01-01 0:00 UTC");
    let t2 = datetime!("2019-01-01 0:00 UTC").to_offset(offset!("-1"));
    assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Equal));
}

#[test]
fn ord() {
    let t1 = datetime!("2019-01-01 0:00 UTC");
    let t2 = datetime!("2019-01-01 0:00 UTC").to_offset(offset!("-1"));
    assert_eq!(t1, t2);

    let t1 = datetime!("2019-01-01 0:00 UTC");
    let t2 = datetime!("2019-01-01 0:00:00.000_000_001 UTC");
    assert!(t2 > t1);
}

#[test]
#[cfg(feature = "std")]
fn hash() {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    assert_eq!(
        {
            let mut hasher = DefaultHasher::new();
            datetime!("2019-01-01 0:00 UTC").hash(&mut hasher);
            hasher.finish()
        },
        {
            let mut hasher = DefaultHasher::new();
            datetime!("2019-01-01 0:00 UTC")
                .to_offset(offset!("-1"))
                .hash(&mut hasher);
            hasher.finish()
        }
    );

    // Ensure that a `PrimitiveDateTime` and `OffsetDateTime` don't collide,
    // even if the UTC time is the same.
    assert_ne!(
        {
            let mut hasher = DefaultHasher::new();
            datetime!("2019-01-01 0:00").hash(&mut hasher);
            hasher.finish()
        },
        {
            let mut hasher = DefaultHasher::new();
            datetime!("2019-01-01 0:00 UTC").hash(&mut hasher);
            hasher.finish()
        }
    );
}

#[test]
fn add_duration() {
    assert_eq!(
        datetime!("2019-01-01 0:00 UTC") + 5.days(),
        datetime!("2019-01-06 0:00 UTC"),
    );
    assert_eq!(
        datetime!("2019-12-31 0:00 UTC") + 1.days(),
        datetime!("2020-01-01 0:00 UTC"),
    );
    assert_eq!(
        datetime!("2019-12-31 23:59:59 UTC") + 2.seconds(),
        datetime!("2020-01-01 0:00:01 UTC"),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00:01 UTC") + (-2).seconds(),
        datetime!("2019-12-31 23:59:59 UTC"),
    );
    assert_eq!(
        datetime!("1999-12-31 23:00 UTC") + 1.hours(),
        datetime!("2000-01-01 0:00 UTC"),
    );
}

#[test]
fn add_std_duration() {
    assert_eq!(
        datetime!("2019-01-01 0:00 UTC") + 5.std_days(),
        datetime!("2019-01-06 0:00 UTC"),
    );
    assert_eq!(
        datetime!("2019-12-31 0:00 UTC") + 1.std_days(),
        datetime!("2020-01-01 0:00 UTC"),
    );
    assert_eq!(
        datetime!("2019-12-31 23:59:59 UTC") + 2.std_seconds(),
        datetime!("2020-01-01 0:00:01 UTC"),
    );
}

#[test]
fn add_assign_duration() {
    let mut ny19 = datetime!("2019-01-01 0:00 UTC");
    ny19 += 5.days();
    assert_eq!(ny19, datetime!("2019-01-06 0:00 UTC"));

    let mut nye20 = datetime!("2019-12-31 0:00 UTC");
    nye20 += 1.days();
    assert_eq!(nye20, datetime!("2020-01-01 0:00 UTC"));

    let mut nye20t = datetime!("2019-12-31 23:59:59 UTC");
    nye20t += 2.seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01 UTC"));

    let mut ny20t = datetime!("2020-01-01 0:00:01 UTC");
    ny20t += (-2).seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59 UTC"));
}

#[test]
fn add_assign_std_duration() {
    let mut ny19 = datetime!("2019-01-01 0:00 UTC");
    ny19 += 5.std_days();
    assert_eq!(ny19, datetime!("2019-01-06 0:00 UTC"));

    let mut nye20 = datetime!("2019-12-31 0:00 UTC");
    nye20 += 1.std_days();
    assert_eq!(nye20, datetime!("2020-01-01 0:00 UTC"));

    let mut nye20t = datetime!("2019-12-31 23:59:59 UTC");
    nye20t += 2.std_seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01 UTC"));
}

#[test]
fn sub_duration() {
    assert_eq!(
        datetime!("2019-01-06 0:00 UTC") - 5.days(),
        datetime!("2019-01-01 0:00 UTC"),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00 UTC") - 1.days(),
        datetime!("2019-12-31 0:00 UTC"),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00:01 UTC") - 2.seconds(),
        datetime!("2019-12-31 23:59:59 UTC"),
    );
    assert_eq!(
        datetime!("2019-12-31 23:59:59 UTC") - (-2).seconds(),
        datetime!("2020-01-01 0:00:01 UTC"),
    );
    assert_eq!(
        datetime!("1999-12-31 23:00 UTC") - (-1).hours(),
        datetime!("2000-01-01 0:00 UTC"),
    );
}

#[test]
fn sub_std_duration() {
    assert_eq!(
        datetime!("2019-01-06 0:00 UTC") - 5.std_days(),
        datetime!("2019-01-01 0:00 UTC"),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00 UTC") - 1.std_days(),
        datetime!("2019-12-31 0:00 UTC"),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00:01 UTC") - 2.std_seconds(),
        datetime!("2019-12-31 23:59:59 UTC"),
    );
}

#[test]
fn sub_assign_duration() {
    let mut ny19 = datetime!("2019-01-06 0:00 UTC");
    ny19 -= 5.days();
    assert_eq!(ny19, datetime!("2019-01-01 0:00 UTC"));

    let mut ny20 = datetime!("2020-01-01 0:00 UTC");
    ny20 -= 1.days();
    assert_eq!(ny20, datetime!("2019-12-31 0:00 UTC"));

    let mut ny20t = datetime!("2020-01-01 0:00:01 UTC");
    ny20t -= 2.seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59 UTC"));

    let mut nye20t = datetime!("2019-12-31 23:59:59 UTC");
    nye20t -= (-2).seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01 UTC"));
}

#[test]
fn sub_assign_std_duration() {
    let mut ny19 = datetime!("2019-01-06 0:00 UTC");
    ny19 -= 5.std_days();
    assert_eq!(ny19, datetime!("2019-01-01 0:00 UTC"));

    let mut ny20 = datetime!("2020-01-01 0:00 UTC");
    ny20 -= 1.std_days();
    assert_eq!(ny20, datetime!("2019-12-31 0:00 UTC"));

    let mut ny20t = datetime!("2020-01-01 0:00:01 UTC");
    ny20t -= 2.std_seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59 UTC"));
}

#[test]
#[cfg(feature = "std")]
fn std_add_duration() {
    assert_eq!(
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")) + 0.seconds(),
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")),
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")) + 5.days(),
        SystemTime::from(datetime!("2019-01-06 0:00 UTC")),
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-12-31 0:00 UTC")) + 1.days(),
        SystemTime::from(datetime!("2020-01-01 0:00 UTC")),
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-12-31 23:59:59 UTC")) + 2.seconds(),
        SystemTime::from(datetime!("2020-01-01 0:00:01 UTC")),
    );
    assert_eq!(
        SystemTime::from(datetime!("2020-01-01 0:00:01 UTC")) + (-2).seconds(),
        SystemTime::from(datetime!("2019-12-31 23:59:59 UTC")),
    );
}

#[test]
#[cfg(feature = "std")]
fn std_add_assign_duration() {
    let mut ny19 = SystemTime::from(datetime!("2019-01-01 0:00 UTC"));
    ny19 += 5.days();
    assert_eq!(ny19, datetime!("2019-01-06 0:00 UTC"));

    let mut nye20 = SystemTime::from(datetime!("2019-12-31 0:00 UTC"));
    nye20 += 1.days();
    assert_eq!(nye20, datetime!("2020-01-01 0:00 UTC"));

    let mut nye20t = SystemTime::from(datetime!("2019-12-31 23:59:59 UTC"));
    nye20t += 2.seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01 UTC"));

    let mut ny20t = SystemTime::from(datetime!("2020-01-01 0:00:01 UTC"));
    ny20t += (-2).seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59 UTC"));
}

#[test]
#[cfg(feature = "std")]
fn std_sub_duration() {
    assert_eq!(
        SystemTime::from(datetime!("2019-01-06 0:00 UTC")) - 5.days(),
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")),
    );
    assert_eq!(
        SystemTime::from(datetime!("2020-01-01 0:00 UTC")) - 1.days(),
        SystemTime::from(datetime!("2019-12-31 0:00 UTC")),
    );
    assert_eq!(
        SystemTime::from(datetime!("2020-01-01 0:00:01 UTC")) - 2.seconds(),
        SystemTime::from(datetime!("2019-12-31 23:59:59 UTC")),
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-12-31 23:59:59 UTC")) - (-2).seconds(),
        SystemTime::from(datetime!("2020-01-01 0:00:01 UTC")),
    );
}

#[test]
#[cfg(feature = "std")]
fn std_sub_assign_duration() {
    let mut ny19 = SystemTime::from(datetime!("2019-01-06 0:00 UTC"));
    ny19 -= 5.days();
    assert_eq!(ny19, datetime!("2019-01-01 0:00 UTC"));

    let mut ny20 = SystemTime::from(datetime!("2020-01-01 0:00 UTC"));
    ny20 -= 1.days();
    assert_eq!(ny20, datetime!("2019-12-31 0:00 UTC"));

    let mut ny20t = SystemTime::from(datetime!("2020-01-01 0:00:01 UTC"));
    ny20t -= 2.seconds();
    assert_eq!(ny20t, datetime!("2019-12-31 23:59:59 UTC"));

    let mut nye20t = SystemTime::from(datetime!("2019-12-31 23:59:59 UTC"));
    nye20t -= (-2).seconds();
    assert_eq!(nye20t, datetime!("2020-01-01 0:00:01 UTC"));
}

#[test]
fn sub_self() {
    assert_eq!(
        datetime!("2019-01-02 0:00 UTC") - datetime!("2019-01-01 0:00 UTC"),
        1.days(),
    );
    assert_eq!(
        datetime!("2019-01-01 0:00 UTC") - datetime!("2019-01-02 0:00 UTC"),
        (-1).days(),
    );
    assert_eq!(
        datetime!("2020-01-01 0:00 UTC") - datetime!("2019-12-31 0:00 UTC"),
        1.days(),
    );
    assert_eq!(
        datetime!("2019-12-31 0:00 UTC") - datetime!("2020-01-01 0:00 UTC"),
        (-1).days(),
    );
}

#[test]
#[cfg(feature = "std")]
fn std_sub() {
    assert_eq!(
        SystemTime::from(datetime!("2019-01-02 0:00 UTC")) - datetime!("2019-01-01 0:00 UTC"),
        1.days()
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")) - datetime!("2019-01-02 0:00 UTC"),
        (-1).days()
    );
    assert_eq!(
        SystemTime::from(datetime!("2020-01-01 0:00 UTC")) - datetime!("2019-12-31 0:00 UTC"),
        1.days()
    );
    assert_eq!(
        SystemTime::from(datetime!("2019-12-31 0:00 UTC")) - datetime!("2020-01-01 0:00 UTC"),
        (-1).days()
    );
}

#[test]
#[cfg(feature = "std")]
fn sub_std() {
    assert_eq!(
        datetime!("2019-01-02 0:00 UTC") - SystemTime::from(datetime!("2019-01-01 0:00 UTC")),
        1.days()
    );
    assert_eq!(
        datetime!("2019-01-01 0:00 UTC") - SystemTime::from(datetime!("2019-01-02 0:00 UTC")),
        (-1).days()
    );
    assert_eq!(
        datetime!("2020-01-01 0:00 UTC") - SystemTime::from(datetime!("2019-12-31 0:00 UTC")),
        1.days()
    );
    assert_eq!(
        datetime!("2019-12-31 0:00 UTC") - SystemTime::from(datetime!("2020-01-01 0:00 UTC")),
        (-1).days()
    );
}

#[test]
#[cfg(feature = "std")]
fn eq_std() {
    let now_datetime = OffsetDateTime::now_utc();
    let now_systemtime = SystemTime::from(now_datetime);
    assert_eq!(now_datetime, now_systemtime);
}

#[test]
#[cfg(feature = "std")]
fn std_eq() {
    let now_datetime = OffsetDateTime::now_utc();
    let now_systemtime = SystemTime::from(now_datetime);
    assert_eq!(now_systemtime, now_datetime);
}

#[test]
#[cfg(feature = "std")]
fn ord_std() {
    assert_eq!(
        datetime!("2019-01-01 0:00 UTC"),
        SystemTime::from(datetime!("2019-01-01 0:00 UTC"))
    );
    assert!(datetime!("2019-01-01 0:00 UTC") < SystemTime::from(datetime!("2020-01-01 0:00 UTC")));
    assert!(datetime!("2019-01-01 0:00 UTC") < SystemTime::from(datetime!("2019-02-01 0:00 UTC")));
    assert!(datetime!("2019-01-01 0:00 UTC") < SystemTime::from(datetime!("2019-01-02 0:00 UTC")));
    assert!(
        datetime!("2019-01-01 0:00 UTC") < SystemTime::from(datetime!("2019-01-01 1:00:00 UTC"))
    );
    assert!(
        datetime!("2019-01-01 0:00 UTC") < SystemTime::from(datetime!("2019-01-01 0:01:00 UTC"))
    );
    assert!(
        datetime!("2019-01-01 0:00 UTC") < SystemTime::from(datetime!("2019-01-01 0:00:01 UTC"))
    );
    assert!(
        datetime!("2019-01-01 0:00 UTC")
            < SystemTime::from(datetime!("2019-01-01 0:00:00.001 UTC"))
    );
    assert!(datetime!("2020-01-01 0:00 UTC") > SystemTime::from(datetime!("2019-01-01 0:00 UTC")));
    assert!(datetime!("2019-02-01 0:00 UTC") > SystemTime::from(datetime!("2019-01-01 0:00 UTC")));
    assert!(datetime!("2019-01-02 0:00 UTC") > SystemTime::from(datetime!("2019-01-01 0:00 UTC")));
    assert!(
        datetime!("2019-01-01 1:00:00 UTC") > SystemTime::from(datetime!("2019-01-01 0:00 UTC"))
    );
    assert!(
        datetime!("2019-01-01 0:01:00 UTC") > SystemTime::from(datetime!("2019-01-01 0:00 UTC"))
    );
    assert!(
        datetime!("2019-01-01 0:00:01 UTC") > SystemTime::from(datetime!("2019-01-01 0:00 UTC"))
    );
    assert!(
        datetime!("2019-01-01 0:00:00.000_000_001 UTC")
            > SystemTime::from(datetime!("2019-01-01 0:00 UTC"))
    );
}

#[test]
#[cfg(feature = "std")]
fn std_ord() {
    assert_eq!(
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")),
        datetime!("2019-01-01 0:00 UTC")
    );
    assert!(SystemTime::from(datetime!("2019-01-01 0:00 UTC")) < datetime!("2020-01-01 0:00 UTC"));
    assert!(SystemTime::from(datetime!("2019-01-01 0:00 UTC")) < datetime!("2019-02-01 0:00 UTC"));
    assert!(SystemTime::from(datetime!("2019-01-01 0:00 UTC")) < datetime!("2019-01-02 0:00 UTC"));
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")) < datetime!("2019-01-01 1:00:00 UTC")
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")) < datetime!("2019-01-01 0:01:00 UTC")
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00 UTC")) < datetime!("2019-01-01 0:00:01 UTC")
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00 UTC"))
            < datetime!("2019-01-01 0:00:00.000_000_001 UTC")
    );
    assert!(SystemTime::from(datetime!("2020-01-01 0:00 UTC")) > datetime!("2019-01-01 0:00 UTC"));
    assert!(SystemTime::from(datetime!("2019-02-01 0:00 UTC")) > datetime!("2019-01-01 0:00 UTC"));
    assert!(SystemTime::from(datetime!("2019-01-02 0:00 UTC")) > datetime!("2019-01-01 0:00 UTC"));
    assert!(
        SystemTime::from(datetime!("2019-01-01 1:00:00 UTC")) > datetime!("2019-01-01 0:00 UTC")
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:01:00 UTC")) > datetime!("2019-01-01 0:00 UTC")
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00:01 UTC")) > datetime!("2019-01-01 0:00 UTC")
    );
    assert!(
        SystemTime::from(datetime!("2019-01-01 0:00:00.001 UTC"))
            > datetime!("2019-01-01 0:00 UTC")
    );
}

#[test]
#[cfg(feature = "std")]
fn from_std() {
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH),
        OffsetDateTime::unix_epoch()
    );
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH - 1.std_days()),
        OffsetDateTime::unix_epoch() - 1.days()
    );
    assert_eq!(
        OffsetDateTime::from(SystemTime::UNIX_EPOCH + 1.std_days()),
        OffsetDateTime::unix_epoch() + 1.days()
    );
}

#[test]
#[cfg(feature = "std")]
fn to_std() {
    assert_eq!(
        SystemTime::from(OffsetDateTime::unix_epoch()),
        SystemTime::UNIX_EPOCH
    );
    assert_eq!(
        SystemTime::from(OffsetDateTime::unix_epoch() + 1.days()),
        SystemTime::UNIX_EPOCH + 1.std_days()
    );
    assert_eq!(
        SystemTime::from(OffsetDateTime::unix_epoch() - 1.days()),
        SystemTime::UNIX_EPOCH - 1.std_days()
    );
}

#[test]
#[cfg(feature = "alloc")]
fn display() {
    assert_eq!(
        datetime!("1970-01-01 0:00 UTC").to_string(),
        String::from("1970-01-01 0:00 +0")
    );
}
