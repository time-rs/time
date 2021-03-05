use time::macros::offset;
#[cfg(feature = "local-offset")]
use time::OffsetDateTime;
use time::{Result, UtcOffset};

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
fn is_positive() {
    assert!(offset!("UTC").is_positive());
    assert!(offset!("+0:00:01").is_positive());
    assert!(!offset!("-0:00:01").is_positive());
    assert!(offset!("+1").is_positive());
    assert!(!offset!("-1").is_positive());
    assert!(offset!("+23:59").is_positive());
    assert!(!offset!("-23:59").is_positive());
    assert!(offset!("+23:59:59").is_positive());
    assert!(!offset!("-23:59:59").is_positive());
}

#[test]
fn is_negative() {
    assert!(!offset!("UTC").is_negative());
    assert!(!offset!("+0:00:01").is_negative());
    assert!(offset!("-0:00:01").is_negative());
    assert!(!offset!("+1").is_negative());
    assert!(offset!("-1").is_negative());
    assert!(!offset!("+23:59").is_negative());
    assert!(offset!("-23:59").is_negative());
    assert!(!offset!("+23:59:59").is_negative());
    assert!(offset!("-23:59:59").is_negative());
}

#[test]
#[cfg(feature = "local-offset")]
fn local_offset_at() {
    #[cfg(not(target_family = "unix"))]
    assert!(UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH).is_ok());

    #[cfg(target_family = "unix")]
    let _ = UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH);
}

#[test]
#[cfg(feature = "local-offset")]
fn current_local_offset() {
    #[cfg(not(target_family = "unix"))]
    assert!(UtcOffset::current_local_offset().is_ok());

    #[cfg(target_family = "unix")]
    let _ = UtcOffset::current_local_offset();
}
