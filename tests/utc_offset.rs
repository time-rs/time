#[cfg(feature = "alloc")]
use time::format_description;
#[cfg(all(feature = "local-offset", not(target_family = "unix")))]
use time::OffsetDateTime;
use time::{Result, UtcOffset};
use time_macros::offset;

#[test]
fn hours() -> Result<()> {
    assert_eq!(UtcOffset::hours(1)?.as_seconds(), 3_600);
    assert_eq!(UtcOffset::hours(-1)?.as_seconds(), -3_600);
    assert_eq!(UtcOffset::hours(23)?.as_seconds(), 82_800);
    assert_eq!(UtcOffset::hours(-23)?.as_seconds(), -82_800);
    Ok(())
}

#[test]
fn directional_hours() {
    assert_eq!(UtcOffset::east_hours(1), Ok(offset!("+1")));
    assert_eq!(UtcOffset::west_hours(1), Ok(offset!("-1")));
}

#[test]
fn minutes() -> Result<()> {
    assert_eq!(UtcOffset::minutes(1)?.as_seconds(), 60);
    assert_eq!(UtcOffset::minutes(-1)?.as_seconds(), -60);
    assert_eq!(UtcOffset::minutes(1_439)?.as_seconds(), 86_340);
    assert_eq!(UtcOffset::minutes(-1_439)?.as_seconds(), -86_340);
    Ok(())
}

#[test]
fn directional_minutes() {
    assert_eq!(UtcOffset::east_minutes(1), Ok(offset!("+0:01")));
    assert_eq!(UtcOffset::west_minutes(1), Ok(offset!("-0:01")));
}

#[test]
fn seconds() -> Result<()> {
    assert_eq!(UtcOffset::seconds(1)?.as_seconds(), 1);
    assert_eq!(UtcOffset::seconds(-1)?.as_seconds(), -1);
    assert_eq!(UtcOffset::seconds(86_399)?.as_seconds(), 86_399);
    assert_eq!(UtcOffset::seconds(-86_399)?.as_seconds(), -86_399);
    Ok(())
}

#[test]
fn directional_seconds() {
    assert_eq!(UtcOffset::east_seconds(1), Ok(offset!("+0:00:01")));
    assert_eq!(UtcOffset::west_seconds(1), Ok(offset!("-0:00:01")));
}

#[test]
fn as_hours() {
    assert_eq!(offset!("+1").as_hours(), 1);
    assert_eq!(offset!("+0:59").as_hours(), 0);
    assert_eq!(offset!("-1").as_hours(), -1);
    assert_eq!(offset!("-0:59").as_hours(), -0);
}

#[test]
fn as_minutes() {
    assert_eq!(offset!("+1").as_minutes(), 60);
    assert_eq!(offset!("+0:01").as_minutes(), 1);
    assert_eq!(offset!("+0:00:59").as_minutes(), 0);
    assert_eq!(offset!("-1").as_minutes(), -60);
    assert_eq!(offset!("-0:01").as_minutes(), -1);
    assert_eq!(offset!("-0:00:59").as_minutes(), 0);
}

#[test]
fn as_seconds() {
    assert_eq!(offset!("+1").as_seconds(), 3_600);
    assert_eq!(offset!("+0:01").as_seconds(), 60);
    assert_eq!(offset!("+0:00:01").as_seconds(), 1);
    assert_eq!(offset!("-1").as_seconds(), -3_600);
    assert_eq!(offset!("-0:01").as_seconds(), -60);
    assert_eq!(offset!("-0:00:01").as_seconds(), -1);
}

#[test]
fn utc_is_zero() {
    assert_eq!(offset!("UTC"), offset!("+0"));
}

#[test]
#[cfg(feature = "alloc")]
fn format() -> time::Result<()> {
    assert_eq!(
        offset!("+01:02:03").format(&format_description::parse("[offset_hour sign:automatic]")?)?,
        "01"
    );
    assert_eq!(
        offset!("+01:02:03").format(&format_description::parse("[offset_hour sign:mandatory]")?)?,
        "+01"
    );
    assert_eq!(
        offset!("-01:02:03").format(&format_description::parse("[offset_hour sign:automatic]")?)?,
        "-01"
    );
    assert_eq!(
        offset!("-01:02:03").format(&format_description::parse("[offset_hour sign:mandatory]")?)?,
        "-01"
    );
    assert_eq!(
        offset!("+01:02:03").format(&format_description::parse("[offset_minute]")?)?,
        "02"
    );
    assert_eq!(
        offset!("+01:02:03").format(&format_description::parse("[offset_second]")?)?,
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
