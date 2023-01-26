use time::macros::offset;
use time::{OffsetDateTime, Result, UtcOffset};

#[test]
fn utc_is_zero() {
    assert_eq!(offset!(UTC), offset!(+0));
}

#[test]
fn from_hms() -> Result<()> {
    assert_eq!(UtcOffset::from_hms(0, 0, 0), Ok(offset!(UTC)));
    assert_eq!(UtcOffset::from_hms(0, 0, 1), Ok(offset!(+0:00:01)));
    assert_eq!(UtcOffset::from_hms(0, 0, -1), Ok(offset!(-0:00:01)));
    assert_eq!(UtcOffset::from_hms(1, 0, 0), Ok(offset!(+1)));
    assert_eq!(UtcOffset::from_hms(-1, 0, 0), Ok(offset!(-1)));
    assert_eq!(UtcOffset::from_hms(23, 59, 0), Ok(offset!(+23:59)));
    assert_eq!(UtcOffset::from_hms(-23, -59, 0), Ok(offset!(-23:59)));
    assert_eq!(UtcOffset::from_hms(23, 59, 59), Ok(offset!(+23:59:59)));
    assert_eq!(UtcOffset::from_hms(-23, -59, -59), Ok(offset!(-23:59:59)));
    assert_eq!(UtcOffset::from_hms(1, 2, 3)?.as_hms(), (1, 2, 3));
    assert_eq!(UtcOffset::from_hms(1, -2, -3)?.as_hms(), (1, 2, 3));
    assert_eq!(UtcOffset::from_hms(0, 2, -3)?.as_hms(), (0, 2, 3));
    Ok(())
}

#[test]
fn from_whole_seconds() {
    assert_eq!(UtcOffset::from_whole_seconds(0), Ok(offset!(UTC)));
    assert_eq!(UtcOffset::from_whole_seconds(1), Ok(offset!(+0:00:01)));
    assert_eq!(UtcOffset::from_whole_seconds(-1), Ok(offset!(-0:00:01)));
    assert_eq!(UtcOffset::from_whole_seconds(3_600), Ok(offset!(+1)));
    assert_eq!(UtcOffset::from_whole_seconds(-3_600), Ok(offset!(-1)));
    assert_eq!(UtcOffset::from_whole_seconds(86_340), Ok(offset!(+23:59)));
    assert_eq!(UtcOffset::from_whole_seconds(-86_340), Ok(offset!(-23:59)));
    assert_eq!(
        UtcOffset::from_whole_seconds(86_399),
        Ok(offset!(+23:59:59))
    );
    assert_eq!(
        UtcOffset::from_whole_seconds(-86_399),
        Ok(offset!(-23:59:59))
    );
}

#[test]
fn as_hms() {
    assert_eq!(offset!(UTC).as_hms(), (0, 0, 0));
    assert_eq!(offset!(+0:00:01).as_hms(), (0, 0, 1));
    assert_eq!(offset!(-0:00:01).as_hms(), (0, 0, -1));
    assert_eq!(offset!(+1).as_hms(), (1, 0, 0));
    assert_eq!(offset!(-1).as_hms(), (-1, 0, 0));
    assert_eq!(offset!(+23:59).as_hms(), (23, 59, 0));
    assert_eq!(offset!(-23:59).as_hms(), (-23, -59, 0));
    assert_eq!(offset!(+23:59:59).as_hms(), (23, 59, 59));
    assert_eq!(offset!(-23:59:59).as_hms(), (-23, -59, -59));
}

#[test]
fn whole_hours() {
    assert_eq!(offset!(+1:02:03).whole_hours(), 1);
    assert_eq!(offset!(-1:02:03).whole_hours(), -1);
}

#[test]
fn whole_minutes() {
    assert_eq!(offset!(+1:02:03).whole_minutes(), 62);
    assert_eq!(offset!(-1:02:03).whole_minutes(), -62);
}

#[test]
fn minutes_past_hour() {
    assert_eq!(offset!(+1:02:03).minutes_past_hour(), 2);
    assert_eq!(offset!(-1:02:03).minutes_past_hour(), -2);
}

#[test]
fn whole_seconds() {
    assert_eq!(offset!(UTC).whole_seconds(), 0);
    assert_eq!(offset!(+0:00:01).whole_seconds(), 1);
    assert_eq!(offset!(-0:00:01).whole_seconds(), -1);
    assert_eq!(offset!(+1).whole_seconds(), 3_600);
    assert_eq!(offset!(-1).whole_seconds(), -3_600);
    assert_eq!(offset!(+23:59).whole_seconds(), 86_340);
    assert_eq!(offset!(-23:59).whole_seconds(), -86_340);
    assert_eq!(offset!(+23:59:59).whole_seconds(), 86_399);
    assert_eq!(offset!(-23:59:59).whole_seconds(), -86_399);
}

#[test]
fn seconds_past_minute() {
    assert_eq!(offset!(+1:02:03).seconds_past_minute(), 3);
    assert_eq!(offset!(-1:02:03).seconds_past_minute(), -3);
}

#[test]
fn is_utc() {
    assert!(offset!(UTC).is_utc());
    assert!(!offset!(+0:00:01).is_utc());
    assert!(!offset!(-0:00:01).is_utc());
    assert!(!offset!(+1).is_utc());
    assert!(!offset!(-1).is_utc());
    assert!(!offset!(+23:59).is_utc());
    assert!(!offset!(-23:59).is_utc());
    assert!(!offset!(+23:59:59).is_utc());
    assert!(!offset!(-23:59:59).is_utc());
}

#[test]
fn is_positive() {
    assert!(!offset!(UTC).is_positive());
    assert!(offset!(+0:00:01).is_positive());
    assert!(!offset!(-0:00:01).is_positive());
    assert!(offset!(+1).is_positive());
    assert!(!offset!(-1).is_positive());
    assert!(offset!(+23:59).is_positive());
    assert!(!offset!(-23:59).is_positive());
    assert!(offset!(+23:59:59).is_positive());
    assert!(!offset!(-23:59:59).is_positive());
}

#[test]
fn is_negative() {
    assert!(!offset!(UTC).is_negative());
    assert!(!offset!(+0:00:01).is_negative());
    assert!(offset!(-0:00:01).is_negative());
    assert!(!offset!(+1).is_negative());
    assert!(offset!(-1).is_negative());
    assert!(!offset!(+23:59).is_negative());
    assert!(offset!(-23:59).is_negative());
    assert!(!offset!(+23:59:59).is_negative());
    assert!(offset!(-23:59:59).is_negative());
}

#[test]
fn neg() {
    assert_eq!(-offset!(UTC), offset!(UTC));
    assert_eq!(-offset!(+0:00:01), offset!(-0:00:01));
    assert_eq!(-offset!(-0:00:01), offset!(+0:00:01));
    assert_eq!(-offset!(+1), offset!(-1));
    assert_eq!(-offset!(-1), offset!(+1));
    assert_eq!(-offset!(+23:59), offset!(-23:59));
    assert_eq!(-offset!(-23:59), offset!(+23:59));
    assert_eq!(-offset!(+23:59:59), offset!(-23:59:59));
    assert_eq!(-offset!(-23:59:59), offset!(+23:59:59));
}

#[test]
fn local_offset_at() {
    #[cfg(not(target_family = "unix"))]
    assert!(UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH).is_ok());

    #[cfg(target_family = "unix")]
    let _ = UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH);
}

#[test]
fn current_local_offset() {
    #[cfg(not(target_family = "unix"))]
    assert!(UtcOffset::current_local_offset().is_ok());

    #[cfg(target_family = "unix")]
    let _ = UtcOffset::current_local_offset();
}
