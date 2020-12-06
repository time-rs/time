#[cfg(feature = "alloc")]
use time::format_description::FormatDescription;
#[cfg(all(feature = "local-offset", not(target_family = "unix")))]
use time::OffsetDateTime;
use time::{Result, UtcOffset};
use time_macros::offset;

#[test]
fn utc_is_zero() {
    assert_eq!(offset!("UTC"), offset!("+0"));
}

#[test]
fn from_hms() -> Result<()> {
    assert_eq!(UtcOffset::from_hms(0, 0, 0), Ok(offset!("UTC")));
    assert_eq!(UtcOffset::from_hms(0, 0, 1), Ok(offset!("+0:00:01")));
    assert_eq!(UtcOffset::from_hms(0, 0, -1), Ok(offset!("-0:00:01")));
    assert_eq!(UtcOffset::from_hms(1, 0, 0), Ok(offset!("+1")));
    assert_eq!(UtcOffset::from_hms(-1, 0, 0), Ok(offset!("-1")));
    assert_eq!(UtcOffset::from_hms(23, 59, 0), Ok(offset!("+23:59")));
    assert_eq!(UtcOffset::from_hms(-23, -59, 0), Ok(offset!("-23:59")));
    assert_eq!(UtcOffset::from_hms(23, 59, 59), Ok(offset!("+23:59:59")));
    assert_eq!(UtcOffset::from_hms(-23, -59, -59), Ok(offset!("-23:59:59")));
    assert_eq!(UtcOffset::from_hms(1, 2, 3)?.as_hms(), (1, 2, 3));
    assert_eq!(UtcOffset::from_hms(1, -2, -3)?.as_hms(), (1, 2, 3));
    assert_eq!(UtcOffset::from_hms(0, 2, -3)?.as_hms(), (0, 2, 3));
    Ok(())
}

#[test]
fn as_hms() {
    assert_eq!(offset!("UTC").as_hms(), (0, 0, 0));
    assert_eq!(offset!("+0:00:01").as_hms(), (0, 0, 1));
    assert_eq!(offset!("-0:00:01").as_hms(), (0, 0, -1));
    assert_eq!(offset!("+1").as_hms(), (1, 0, 0));
    assert_eq!(offset!("-1").as_hms(), (-1, 0, 0));
    assert_eq!(offset!("+23:59").as_hms(), (23, 59, 0));
    assert_eq!(offset!("-23:59").as_hms(), (-23, -59, 0));
    assert_eq!(offset!("+23:59:59").as_hms(), (23, 59, 59));
    assert_eq!(offset!("-23:59:59").as_hms(), (-23, -59, -59));
}

#[test]
fn to_seconds() {
    assert_eq!(offset!("UTC").to_seconds(), 0);
    assert_eq!(offset!("+0:00:01").to_seconds(), 1);
    assert_eq!(offset!("-0:00:01").to_seconds(), -1);
    assert_eq!(offset!("+1").to_seconds(), 3_600);
    assert_eq!(offset!("-1").to_seconds(), -3_600);
    assert_eq!(offset!("+23:59").to_seconds(), 86_340);
    assert_eq!(offset!("-23:59").to_seconds(), -86_340);
    assert_eq!(offset!("+23:59:59").to_seconds(), 86_399);
    assert_eq!(offset!("-23:59:59").to_seconds(), -86_399);
}

#[test]
#[cfg(feature = "alloc")]
fn format() -> time::Result<()> {
    assert_eq!(
        offset!("+01:02:03").format(&FormatDescription::parse("[offset_hour sign:automatic]")?)?,
        "01"
    );
    assert_eq!(
        offset!("+01:02:03").format(&FormatDescription::parse("[offset_hour sign:mandatory]")?)?,
        "+01"
    );
    assert_eq!(
        offset!("-01:02:03").format(&FormatDescription::parse("[offset_hour sign:automatic]")?)?,
        "-01"
    );
    assert_eq!(
        offset!("-01:02:03").format(&FormatDescription::parse("[offset_hour sign:mandatory]")?)?,
        "-01"
    );
    assert_eq!(
        offset!("+01:02:03").format(&FormatDescription::parse("[offset_minute]")?)?,
        "02"
    );
    assert_eq!(
        offset!("+01:02:03").format(&FormatDescription::parse("[offset_second]")?)?,
        "03"
    );

    Ok(())
}

#[test]
#[cfg(feature = "alloc")]
fn display() {
    assert_eq!(offset!("UTC").to_string(), "+00:00:00");
    assert_eq!(offset!("+0:00:01").to_string(), "+00:00:01");
    assert_eq!(offset!("-0:00:01").to_string(), "-00:00:01");
    assert_eq!(offset!("+1").to_string(), "+01:00:00");
    assert_eq!(offset!("-1").to_string(), "-01:00:00");
    assert_eq!(offset!("+23:59").to_string(), "+23:59:00");
    assert_eq!(offset!("-23:59").to_string(), "-23:59:00");
    assert_eq!(offset!("+23:59:59").to_string(), "+23:59:59");
    assert_eq!(offset!("-23:59:59").to_string(), "-23:59:59");
}

#[test]
#[cfg(all(feature = "local-offset", not(target_family = "unix")))]
fn local_offset_at() {
    assert!(UtcOffset::local_offset_at(OffsetDateTime::unix_epoch()).is_ok());
}

#[test]
#[cfg(all(feature = "local-offset", not(target_family = "unix")))]
fn current_local_offset() {
    assert!(UtcOffset::current_local_offset().is_ok());
}
