use std::cmp::Ordering;
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    PrimitiveDateTime, Weekday,
};
use time_macros::{date, offset, time};

#[test]
fn new() {
    assert_eq!(
        PrimitiveDateTime::new(date!(2019 - 01 - 01), time!(0:00)),
        date!(2019 - 01 - 01).midnight(),
    );
}

#[test]
fn date() {
    assert_eq!(
        date!(2019 - 01 - 01).midnight().date(),
        date!(2019 - 01 - 01)
    );
}

#[test]
fn time() {
    assert_eq!(date!(2019 - 01 - 01).midnight().time(), time!(0:00));
}

#[test]
fn year() {
    assert_eq!(date!(2019 - 01 - 01).midnight().year(), 2019);
    assert_eq!(date!(2019 - 12 - 31).midnight().year(), 2019);
    assert_eq!(date!(2020 - 01 - 01).midnight().year(), 2020);
}

#[test]
fn month() {
    assert_eq!(date!(2019 - 01 - 01).midnight().month(), 1);
    assert_eq!(date!(2019 - 12 - 31).midnight().month(), 12);
}

#[test]
fn day() {
    assert_eq!(date!(2019 - 01 - 01).midnight().day(), 1);
    assert_eq!(date!(2019 - 12 - 31).midnight().day(), 31);
}

#[test]
fn month_day() {
    assert_eq!(date!(2019 - 01 - 01).midnight().month_day(), (1, 1));
    assert_eq!(date!(2019 - 12 - 31).midnight().month_day(), (12, 31));
}

#[test]
fn ordinal() {
    assert_eq!(date!(2019 - 01 - 01).midnight().ordinal(), 1);
    assert_eq!(date!(2019 - 12 - 31).midnight().ordinal(), 365);
}

#[test]
fn iso_year_week() {
    assert_eq!(date!(2019 - 01 - 01).midnight().iso_year_week(), (2019, 1));
    assert_eq!(date!(2019 - 10 - 04).midnight().iso_year_week(), (2019, 40));
    assert_eq!(date!(2020 - 01 - 01).midnight().iso_year_week(), (2020, 1));
    assert_eq!(date!(2020 - 12 - 31).midnight().iso_year_week(), (2020, 53));
    assert_eq!(date!(2021 - 01 - 01).midnight().iso_year_week(), (2020, 53));
}

#[test]
fn week() {
    assert_eq!(date!(2019 - 01 - 01).midnight().week(), 1);
    assert_eq!(date!(2019 - 10 - 04).midnight().week(), 40);
    assert_eq!(date!(2020 - 01 - 01).midnight().week(), 1);
    assert_eq!(date!(2020 - 12 - 31).midnight().week(), 53);
    assert_eq!(date!(2021 - 01 - 01).midnight().week(), 53);
}

#[test]
fn sunday_based_week() {
    assert_eq!(date!(2019 - 01 - 01).midnight().sunday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 01).midnight().sunday_based_week(), 0);
    assert_eq!(date!(2020 - 12 - 31).midnight().sunday_based_week(), 52);
    assert_eq!(date!(2021 - 01 - 01).midnight().sunday_based_week(), 0);
}

#[test]
fn monday_based_week() {
    assert_eq!(date!(2019 - 01 - 01).midnight().monday_based_week(), 0);
    assert_eq!(date!(2020 - 01 - 01).midnight().monday_based_week(), 0);
    assert_eq!(date!(2020 - 12 - 31).midnight().monday_based_week(), 52);
    assert_eq!(date!(2021 - 01 - 01).midnight().monday_based_week(), 0);
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_eq!(date!(2019 - 01 - 01).midnight().weekday(), Tuesday);
    assert_eq!(date!(2019 - 02 - 01).midnight().weekday(), Friday);
    assert_eq!(date!(2019 - 03 - 01).midnight().weekday(), Friday);
    assert_eq!(date!(2019 - 04 - 01).midnight().weekday(), Monday);
    assert_eq!(date!(2019 - 05 - 01).midnight().weekday(), Wednesday);
    assert_eq!(date!(2019 - 06 - 01).midnight().weekday(), Saturday);
    assert_eq!(date!(2019 - 07 - 01).midnight().weekday(), Monday);
    assert_eq!(date!(2019 - 08 - 01).midnight().weekday(), Thursday);
    assert_eq!(date!(2019 - 09 - 01).midnight().weekday(), Sunday);
    assert_eq!(date!(2019 - 10 - 01).midnight().weekday(), Tuesday);
    assert_eq!(date!(2019 - 11 - 01).midnight().weekday(), Friday);
    assert_eq!(date!(2019 - 12 - 01).midnight().weekday(), Sunday);
}

#[test]
fn hour() {
    assert_eq!(date!(2019 - 01 - 01).with_time(time!(0:00)).hour(), 0);
    assert_eq!(date!(2019 - 01 - 01).with_time(time!(23:59:59)).hour(), 23);
}

#[test]
fn minute() {
    assert_eq!(date!(2019 - 01 - 01).with_time(time!(0:00)).minute(), 0);
    assert_eq!(
        date!(2019 - 01 - 01).with_time(time!(23:59:59)).minute(),
        59
    );
}

#[test]
fn second() {
    assert_eq!(date!(2019 - 01 - 01).with_time(time!(0:00)).second(), 0);
    assert_eq!(
        date!(2019 - 01 - 01).with_time(time!(23:59:59)).second(),
        59
    );
}

#[test]
fn millisecond() {
    assert_eq!(date!(2019 - 01 - 01).midnight().millisecond(), 0);
    assert_eq!(
        date!(2019 - 01 - 01)
            .with_time(time!(23:59:59.999))
            .millisecond(),
        999
    );
}

#[test]
fn microsecond() {
    assert_eq!(date!(2019 - 01 - 01).midnight().microsecond(), 0);
    assert_eq!(
        date!(2019 - 01 - 01)
            .with_time(time!(23:59:59.999_999))
            .microsecond(),
        999_999
    );
}

#[test]
fn nanosecond() {
    assert_eq!(date!(2019 - 01 - 01).midnight().nanosecond(), 0);
    assert_eq!(
        date!(2019 - 01 - 01)
            .with_time(time!(23:59:59.999_999_999))
            .nanosecond(),
        999_999_999
    );
}

#[test]
fn assume_offset() {
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .assume_offset(offset!(UTC))
            .unix_timestamp(),
        1_546_300_800,
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .assume_offset(offset!(-1))
            .unix_timestamp(),
        1_546_304_400,
    );
}

#[test]
fn assume_utc() {
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .assume_utc()
            .unix_timestamp(),
        1_546_300_800,
    );
}

#[test]
fn format() {
    assert_eq!(
        date!(2019 - 01 - 02).with_time(time!(3:04:05)).format("%c"),
        "Wed Jan 2 3:04:05 2019"
    );
}

#[test]
fn parse() {
    assert_eq!(
        PrimitiveDateTime::parse("Wed Jan 2 3:04:05 2019", "%c"),
        Ok(date!(2019 - 01 - 02).with_time(time!(3:04:05))),
    );
    assert_eq!(
        PrimitiveDateTime::parse("2019-002 23:59:59", "%Y-%j %T"),
        Ok(date!(2019 - 002).with_time(time!(23:59:59)))
    );
    assert_eq!(
        PrimitiveDateTime::parse("2019-W01-3 12:00:00 pm", "%G-W%V-%u %r"),
        Ok(date!(2019 - 002).with_time(time!(12:00))),
    );
}

#[test]
fn add_duration() {
    assert_eq!(
        date!(2019 - 01 - 01).midnight() + 5.days(),
        date!(2019 - 01 - 06).midnight(),
    );
    assert_eq!(
        date!(2019 - 12 - 31).midnight() + 1.days(),
        date!(2020 - 01 - 01).midnight(),
    );
    assert_eq!(
        date!(2019 - 12 - 31).with_time(time!(23:59:59)) + 2.seconds(),
        date!(2020 - 01 - 01).with_time(time!(0:00:01)),
    );
    assert_eq!(
        date!(2020 - 01 - 01).with_time(time!(0:00:01)) + (-2).seconds(),
        date!(2019 - 12 - 31).with_time(time!(23:59:59)),
    );
    assert_eq!(
        date!(1999 - 12 - 31).with_time(time!(23:00)) + 1.hours(),
        date!(2000 - 01 - 01).midnight(),
    );
}

#[test]
fn add_std_duration() {
    assert_eq!(
        date!(2019 - 01 - 01).midnight() + 5.std_days(),
        date!(2019 - 01 - 06).midnight(),
    );
    assert_eq!(
        date!(2019 - 12 - 31).midnight() + 1.std_days(),
        date!(2020 - 01 - 01).midnight(),
    );
    assert_eq!(
        date!(2019 - 12 - 31).with_time(time!(23:59:59)) + 2.std_seconds(),
        date!(2020 - 01 - 01).with_time(time!(0:00:01)),
    );
}

#[test]
fn add_assign_duration() {
    let mut ny19 = date!(2019 - 01 - 01).midnight();
    ny19 += 5.days();
    assert_eq!(ny19, date!(2019 - 01 - 06).midnight());

    let mut nye20 = date!(2019 - 12 - 31).midnight();
    nye20 += 1.days();
    assert_eq!(nye20, date!(2020 - 01 - 01).midnight());

    let mut nye20t = date!(2019 - 12 - 31).with_time(time!(23:59:59));
    nye20t += 2.seconds();
    assert_eq!(nye20t, date!(2020 - 01 - 01).with_time(time!(0:00:01)));

    let mut ny20t = date!(2020 - 01 - 01).with_time(time!(0:00:01));
    ny20t += (-2).seconds();
    assert_eq!(ny20t, date!(2019 - 12 - 31).with_time(time!(23:59:59)));
}

#[test]
fn add_assign_std_duration() {
    let mut ny19 = date!(2019 - 01 - 01).midnight();
    ny19 += 5.std_days();
    assert_eq!(ny19, date!(2019 - 01 - 06).midnight());

    let mut nye20 = date!(2019 - 12 - 31).midnight();
    nye20 += 1.std_days();
    assert_eq!(nye20, date!(2020 - 01 - 01).midnight());

    let mut nye20t = date!(2019 - 12 - 31).with_time(time!(23:59:59));
    nye20t += 2.std_seconds();
    assert_eq!(nye20t, date!(2020 - 01 - 01).with_time(time!(0:00:01)));
}

#[test]
fn sub_duration() {
    assert_eq!(
        date!(2019 - 01 - 06).midnight() - 5.days(),
        date!(2019 - 01 - 01).midnight(),
    );
    assert_eq!(
        date!(2020 - 01 - 01).midnight() - 1.days(),
        date!(2019 - 12 - 31).midnight(),
    );
    assert_eq!(
        date!(2020 - 01 - 01).with_time(time!(0:00:01)) - 2.seconds(),
        date!(2019 - 12 - 31).with_time(time!(23:59:59)),
    );
    assert_eq!(
        date!(2019 - 12 - 31).with_time(time!(23:59:59)) - (-2).seconds(),
        date!(2020 - 01 - 01).with_time(time!(0:00:01)),
    );
    assert_eq!(
        date!(1999 - 12 - 31).with_time(time!(23:00)) - (-1).hours(),
        date!(2000 - 01 - 01).midnight(),
    );
}

#[test]
fn sub_std_duration() {
    assert_eq!(
        date!(2019 - 01 - 06).midnight() - 5.std_days(),
        date!(2019 - 01 - 01).midnight(),
    );
    assert_eq!(
        date!(2020 - 01 - 01).midnight() - 1.std_days(),
        date!(2019 - 12 - 31).midnight(),
    );
    assert_eq!(
        date!(2020 - 01 - 01).with_time(time!(0:00:01)) - 2.std_seconds(),
        date!(2019 - 12 - 31).with_time(time!(23:59:59)),
    );
}

#[test]
fn sub_assign_duration() {
    let mut ny19 = date!(2019 - 01 - 06).midnight();
    ny19 -= 5.days();
    assert_eq!(ny19, date!(2019 - 01 - 01).midnight());

    let mut ny20 = date!(2020 - 01 - 01).midnight();
    ny20 -= 1.days();
    assert_eq!(ny20, date!(2019 - 12 - 31).midnight());

    let mut ny20t = date!(2020 - 01 - 01).with_time(time!(0:00:01));
    ny20t -= 2.seconds();
    assert_eq!(ny20t, date!(2019 - 12 - 31).with_time(time!(23:59:59)));

    let mut nye20t = date!(2019 - 12 - 31).with_time(time!(23:59:59));
    nye20t -= (-2).seconds();
    assert_eq!(nye20t, date!(2020 - 01 - 01).with_time(time!(0:00:01)));
}

#[test]
fn sub_assign_std_duration() {
    let mut ny19 = date!(2019 - 01 - 06).midnight();
    ny19 -= 5.std_days();
    assert_eq!(ny19, date!(2019 - 01 - 01).midnight());

    let mut ny20 = date!(2020 - 01 - 01).midnight();
    ny20 -= 1.std_days();
    assert_eq!(ny20, date!(2019 - 12 - 31).midnight());

    let mut ny20t = date!(2020 - 01 - 01).with_time(time!(0:00:01));
    ny20t -= 2.std_seconds();
    assert_eq!(ny20t, date!(2019 - 12 - 31).with_time(time!(23:59:59)));
}

#[test]
fn sub_datetime() {
    assert_eq!(
        date!(2019 - 01 - 02).midnight() - date!(2019 - 01 - 01).midnight(),
        1.days()
    );
    assert_eq!(
        date!(2019 - 01 - 01).midnight() - date!(2019 - 01 - 02).midnight(),
        (-1).days()
    );
    assert_eq!(
        date!(2020 - 01 - 01).midnight() - date!(2019 - 12 - 31).midnight(),
        1.days()
    );
    assert_eq!(
        date!(2019 - 12 - 31).midnight() - date!(2020 - 01 - 01).midnight(),
        (-1).days()
    );
}

#[test]
fn ord() {
    use Ordering::*;
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 01).midnight()),
        Some(Equal)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2020 - 01 - 01).midnight()),
        Some(Less)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 02 - 01).midnight()),
        Some(Less)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 02).midnight()),
        Some(Less)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 01).with_time(time!(1:00))),
        Some(Less)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 01).with_time(time!(0:01))),
        Some(Less)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 01).with_time(time!(0:00:01))),
        Some(Less)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 01).with_time(time!(0:00:00.000_000_001))),
        Some(Less)
    );
    assert_eq!(
        date!(2020 - 01 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 01).midnight()),
        Some(Greater)
    );
    assert_eq!(
        date!(2019 - 02 - 01)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 01).midnight()),
        Some(Greater)
    );
    assert_eq!(
        date!(2019 - 01 - 02)
            .midnight()
            .partial_cmp(&date!(2019 - 01 - 01).midnight()),
        Some(Greater)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .with_time(time!(1:00))
            .partial_cmp(&date!(2019 - 01 - 01).midnight()),
        Some(Greater)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .with_time(time!(0:01))
            .partial_cmp(&date!(2019 - 01 - 01).midnight()),
        Some(Greater)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .with_time(time!(0:00:01))
            .partial_cmp(&date!(2019 - 01 - 01).midnight()),
        Some(Greater)
    );
    assert_eq!(
        date!(2019 - 01 - 01)
            .with_time(time!(0:00:00.000_000_001))
            .partial_cmp(&date!(2019 - 01 - 01).midnight()),
        Some(Greater)
    );
}

#[test]
#[cfg(feature = "std")]
fn display() {
    assert_eq!(
        date!(1970 - 01 - 01).midnight().to_string(),
        String::from("1970-01-01 0:00")
    );
    assert_eq!(
        date!(1970 - 01 - 01).with_time(time!(0:00:01)).to_string(),
        String::from("1970-01-01 0:00:01")
    );
}
