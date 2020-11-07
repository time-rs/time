#[cfg(feature = "alloc")]
use time::error;
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    Result, Time,
};
use time_macros::time;

#[test]
fn from_hms_nanos_unchecked() {
    assert_eq!(
        Ok(Time::from_hms_nanos_unchecked(0, 1, 2, 3)),
        Time::from_hms_nano(0, 1, 2, 3)
    );
}

#[test]
fn midnight() {
    assert_eq!(Time::midnight(), time!("0:00"));
}

#[test]
fn from_hms() -> Result<()> {
    let time = Time::from_hms(1, 2, 3)?;
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
    assert_eq!(time.nanosecond(), 0);

    assert!(Time::from_hms(24, 0, 0).is_err());
    assert!(Time::from_hms(0, 60, 0).is_err());
    assert!(Time::from_hms(0, 0, 60).is_err());
    Ok(())
}

#[test]
fn from_hms_milli() -> Result<()> {
    let time = Time::from_hms_milli(1, 2, 3, 4)?;
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
    assert_eq!(time.millisecond(), 4);
    assert_eq!(time.nanosecond(), 4_000_000);

    assert!(Time::from_hms_milli(24, 0, 0, 0).is_err());
    assert!(Time::from_hms_milli(0, 60, 0, 0).is_err());
    assert!(Time::from_hms_milli(0, 0, 60, 0).is_err());
    assert!(Time::from_hms_milli(0, 0, 0, 1_000).is_err());
    Ok(())
}

#[test]
fn from_hms_micro() -> Result<()> {
    let time = Time::from_hms_micro(1, 2, 3, 4)?;
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
    assert_eq!(time.microsecond(), 4);
    assert_eq!(time.nanosecond(), 4_000);

    assert!(Time::from_hms_micro(24, 0, 0, 0).is_err());
    assert!(Time::from_hms_micro(0, 60, 0, 0).is_err());
    assert!(Time::from_hms_micro(0, 0, 60, 0).is_err());
    assert!(Time::from_hms_micro(0, 0, 0, 1_000_000).is_err());
    Ok(())
}

#[test]
fn from_hms_nano() -> Result<()> {
    let time = Time::from_hms_nano(1, 2, 3, 4)?;
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
    assert_eq!(time.nanosecond(), 4);

    assert!(Time::from_hms_nano(24, 0, 0, 0).is_err());
    assert!(Time::from_hms_nano(0, 60, 0, 0).is_err());
    assert!(Time::from_hms_nano(0, 0, 60, 0).is_err());
    assert!(Time::from_hms_nano(0, 0, 0, 1_000_000_000).is_err());
    Ok(())
}

#[test]
fn hour() -> Result<()> {
    for hour in 0..24 {
        assert_eq!(Time::from_hms(hour, 0, 0)?.hour(), hour);
        assert_eq!(Time::from_hms(hour, 59, 59)?.hour(), hour);
    }
    Ok(())
}

#[test]
fn minute() -> Result<()> {
    for minute in 0..60 {
        assert_eq!(Time::from_hms(0, minute, 0)?.minute(), minute);
        assert_eq!(Time::from_hms(23, minute, 59)?.minute(), minute);
    }
    Ok(())
}

#[test]
fn second() -> Result<()> {
    for second in 0..60 {
        assert_eq!(Time::from_hms(0, 0, second)?.second(), second);
        assert_eq!(Time::from_hms(23, 59, second)?.second(), second);
    }
    Ok(())
}

#[test]
fn millisecond() -> Result<()> {
    for milli in 0..1_000 {
        assert_eq!(Time::from_hms_milli(0, 0, 0, milli)?.millisecond(), milli);
        assert_eq!(
            Time::from_hms_milli(23, 59, 59, milli)?.millisecond(),
            milli
        );
    }
    Ok(())
}

#[test]
fn microsecond() -> Result<()> {
    for micro in (0..1_000_000).step_by(1_000) {
        assert_eq!(Time::from_hms_micro(0, 0, 0, micro)?.microsecond(), micro);
        assert_eq!(
            Time::from_hms_micro(23, 59, 59, micro)?.microsecond(),
            micro
        );
    }
    Ok(())
}

#[test]
fn nanosecond() -> Result<()> {
    for nano in (0..1_000_000_000).step_by(1_000_000) {
        assert_eq!(Time::from_hms_nano(0, 0, 0, nano)?.nanosecond(), nano);
        assert_eq!(Time::from_hms_nano(23, 59, 59, nano)?.nanosecond(), nano);
    }
    Ok(())
}

#[test]
#[cfg(feature = "alloc")]
fn format() {
    // Check all specifiers for time objects.
    let time = time!("0:01:02.345_678_901");
    assert_eq!(time.format("%H"), "00");
    assert_eq!(time.format("%I"), "12");
    assert_eq!(time.format("%M"), "01");
    assert_eq!(time.format("%N"), "345678901");
    assert_eq!(time.format("%p"), "am");
    assert_eq!(time.format("%P"), "AM");
    assert_eq!(time.format("%r"), "12:01:02 am");
    assert_eq!(time.format("%R"), "0:01");
    assert_eq!(time.format("%S"), "02");
    assert_eq!(time.format("%T"), "0:01:02");

    // Ensure all variants of AM/PM are covered.
    let time = time!("12:01:02");
    assert_eq!(time.format("%p"), "pm");
    assert_eq!(time.format("%P"), "PM");
}

#[test]
#[should_panic]
#[cfg(feature = "alloc")]
fn invalid_format_specifier() {
    Time::midnight().format("%$"); // `$` isn't a valid specifier
}

#[test]
#[cfg(feature = "alloc")]
fn parse() {
    let time = time!("0:01:02.345_678_901");
    assert_eq!(Time::parse("0:01:02.345678901 00", "%T.%N %H"), Ok(time));
    assert_eq!(Time::parse("0:01:02.345678901 12", "%T.%N %I"), Ok(time));
    assert_eq!(Time::parse("0:01:02.345678901 01", "%T.%N %M"), Ok(time));
    assert_eq!(
        Time::parse("0:01:02.345678901 345678901", "%T.%N %N"),
        Ok(time)
    );
    assert_eq!(Time::parse("0:01:02.345678901 am", "%T.%N %p"), Ok(time));
    assert_eq!(Time::parse("0:01:02.345678901 AM", "%T.%N %P"), Ok(time));
    assert_eq!(
        Time::parse("0:01:02.345678901 12:01:02 am", "%T.%N %r"),
        Ok(time)
    );
    assert_eq!(Time::parse("0:01:02.345678901 0:01", "%T.%N %R"), Ok(time));
    assert_eq!(Time::parse("0:01:02.345678901 02", "%T.%N %S"), Ok(time));
    assert_eq!(
        Time::parse("0:01:02.345678901 0:01:02", "%T.%N %T"),
        Ok(time)
    );

    // Times that aren't 12 AM or 12 PM.
    // For additional coverage, use various modifiers on the hour.
    assert_eq!(Time::parse("1:00 am", "%-I:%M %p"), Ok(time!("1:00")));
    assert_eq!(Time::parse("1:00 pm", "%-I:%M %p"), Ok(time!("13:00")));
    assert_eq!(Time::parse(" 1:00 am", "%_I:%M %p"), Ok(time!("1:00")));
    assert_eq!(Time::parse(" 1:00 pm", "%_I:%M %p"), Ok(time!("13:00")));
    assert_eq!(Time::parse("01:00 am", "%0I:%M %p"), Ok(time!("1:00")));
    assert_eq!(Time::parse("01:00 pm", "%0I:%M %p"), Ok(time!("13:00")));

    // Additional coverage
    assert_eq!(
        Time::parse("1:02:03.456789012 pm", "%-I:%M:%S.%N %p"),
        Ok(time!("13:02:03.456_789_012"))
    );
    assert_eq!(
        Time::parse("", ""),
        Err(error::Parse::InsufficientInformation)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn parse_missing_seconds() {
    // Missing seconds defaults to zero.
    assert_eq!(Time::parse("0:00", "%-H:%M"), Ok(time!("0:00")));
    assert_eq!(Time::parse("23:59", "%H:%M"), Ok(time!("23:59")));
    assert_eq!(Time::parse("12:00 am", "%I:%M %p"), Ok(time!("0:00")));
    assert_eq!(Time::parse("12:00 pm", "%I:%M %p"), Ok(time!("12:00")));
}

#[test]
#[cfg(feature = "alloc")]
fn parse_missing_minutes() {
    // Missing minutes defaults to zero.
    assert_eq!(Time::parse("0", "%-H"), Ok(time!("0:00")));
    assert_eq!(Time::parse("23", "%H"), Ok(time!("23:00")));
    assert_eq!(Time::parse("12am", "%I%p"), Ok(time!("0:00")));
    assert_eq!(Time::parse("12pm", "%I%p"), Ok(time!("12:00")));
}

#[test]
#[cfg(feature = "alloc")]
fn display() {
    assert_eq!(time!("0:00").to_string(), "0:00");
    assert_eq!(time!("23:59").to_string(), "23:59");
    assert_eq!(time!("23:59:59").to_string(), "23:59:59");
    assert_eq!(time!("0:00:01").to_string(), "0:00:01");
    assert_eq!(time!("0:00:00.001").to_string(), "0:00:00.001");
    assert_eq!(time!("0:00:00.000_001").to_string(), "0:00:00.000001");
    assert_eq!(
        time!("0:00:00.000_000_001").to_string(),
        "0:00:00.000000001"
    );
}

#[test]
fn add_duration() {
    assert_eq!(time!("0:00") + 1.seconds(), time!("0:00:01"));
    assert_eq!(time!("0:00") + 1.minutes(), time!("0:01"));
    assert_eq!(time!("0:00") + 1.hours(), time!("1:00"));
    assert_eq!(time!("0:00") + 1.days(), time!("0:00"));
}

#[test]
fn add_assign_duration() {
    let mut time = time!("0:00");

    time += 1.seconds();
    assert_eq!(time, time!("0:00:01"));

    time += 1.minutes();
    assert_eq!(time, time!("0:01:01"));

    time += 1.hours();
    assert_eq!(time, time!("1:01:01"));

    time += 1.days();
    assert_eq!(time, time!("1:01:01"));
}

#[test]
fn sub_duration() {
    assert_eq!(time!("12:00") - 1.hours(), time!("11:00"));

    // Underflow
    assert_eq!(time!("0:00") - 1.seconds(), time!("23:59:59"));
    assert_eq!(time!("0:00") - 1.minutes(), time!("23:59"));
    assert_eq!(time!("0:00") - 1.hours(), time!("23:00"));
    assert_eq!(time!("0:00") - 1.days(), time!("0:00"));
}

#[test]
fn sub_assign_duration() {
    let mut time = time!("0:00");

    time -= 1.seconds();
    assert_eq!(time, time!("23:59:59"));

    time -= 1.minutes();
    assert_eq!(time, time!("23:58:59"));

    time -= 1.hours();
    assert_eq!(time, time!("22:58:59"));

    time -= 1.days();
    assert_eq!(time, time!("22:58:59"));
}

#[test]
fn add_std_duration() {
    assert_eq!(time!("0:00") + 1.std_milliseconds(), time!("0:00:00.001"));
    assert_eq!(time!("0:00") + 1.std_seconds(), time!("0:00:01"));
    assert_eq!(time!("0:00") + 1.std_minutes(), time!("0:01"));
    assert_eq!(time!("0:00") + 1.std_hours(), time!("1:00"));
    assert_eq!(time!("0:00") + 1.std_days(), time!("0:00"));
}

#[test]
fn add_assign_std_duration() {
    let mut time = time!("0:00");

    time += 1.std_seconds();
    assert_eq!(time, time!("0:00:01"));

    time += 1.std_minutes();
    assert_eq!(time, time!("0:01:01"));

    time += 1.std_hours();
    assert_eq!(time, time!("1:01:01"));

    time += 1.std_days();
    assert_eq!(time, time!("1:01:01"));
}

#[test]
fn sub_std_duration() {
    assert_eq!(time!("12:00") - 1.std_hours(), time!("11:00"));

    // Underflow
    assert_eq!(time!("0:00") - 1.std_milliseconds(), time!("23:59:59.999"));
    assert_eq!(time!("0:00") - 1.std_seconds(), time!("23:59:59"));
    assert_eq!(time!("0:00") - 1.std_minutes(), time!("23:59"));
    assert_eq!(time!("0:00") - 1.std_hours(), time!("23:00"));
    assert_eq!(time!("0:00") - 1.std_days(), time!("0:00"));
}

#[test]
fn sub_assign_std_duration() {
    let mut time = time!("0:00");

    time -= 1.std_seconds();
    assert_eq!(time, time!("23:59:59"));

    time -= 1.std_minutes();
    assert_eq!(time, time!("23:58:59"));

    time -= 1.std_hours();
    assert_eq!(time, time!("22:58:59"));

    time -= 1.std_days();
    assert_eq!(time, time!("22:58:59"));
}

#[test]
fn sub_time() {
    assert_eq!(time!("0:00") - time!("0:00"), 0.seconds());
    assert_eq!(time!("1:00") - time!("0:00"), 1.hours());
    assert_eq!(
        time!("1:00") - time!("0:00:01"),
        59.minutes() + 59.seconds()
    );
}

#[test]
fn ordering() {
    assert!(time!("0:00") < time!("0:00:00.000_000_001"));
    assert!(time!("0:00") < time!("0:00:01"));
    assert!(time!("12:00") > time!("11:00"));
    assert_eq!(time!("0:00"), time!("0:00"));
}
