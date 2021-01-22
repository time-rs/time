#![cfg(feature = "serde")]

use time::{
    macros::{date, datetime, offset, time},
    Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
};

#[test]
fn time() -> serde_json::Result<()> {
    let original = [Time::midnight(), time!("23:59:59.999_999_999")];
    let serialized = "[[0,0,0,0],[23,59,59,999999999]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Time; 2]>(serialized)?, original);

    Ok(())
}

#[test]
fn date() -> serde_json::Result<()> {
    let original = [date!("-9999-001"), date!("+9999-365")];
    let serialized = "[[-9999,1],[9999,365]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Date; 2]>(serialized)?, original);

    Ok(())
}

#[test]
fn primitive_date_time() -> serde_json::Result<()> {
    let original = [
        datetime!("-9999-001 0:00"),
        datetime!("+9999-365 23:59:59.999_999_999"),
    ];
    let serialized = "[[-9999,1,0,0,0,0],[9999,365,23,59,59,999999999]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(
        serde_json::from_str::<[PrimitiveDateTime; 2]>(serialized)?,
        original
    );

    Ok(())
}

#[test]
fn offset_date_time() -> serde_json::Result<()> {
    let original = [
        datetime!("-9999-001 0:00 UTC").to_offset(offset!("+23:59:59")),
        datetime!("+9999-365 23:59:59.999_999_999 UTC").to_offset(offset!("-23:59:59")),
    ];
    let serialized = "[[-9999,1,23,59,59,0,23,59,59],[9999,365,0,0,0,999999999,-23,-59,-59]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(
        serde_json::from_str::<[OffsetDateTime; 2]>(serialized)?,
        original
    );

    Ok(())
}

#[test]
fn utc_offset() -> serde_json::Result<()> {
    let original = [offset!("-23:59:59"), offset!("+23:59:59")];
    let serialized = "[[-23,-59,-59],[23,59,59]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(
        serde_json::from_str::<[UtcOffset; 2]>(serialized)?,
        original
    );

    Ok(())
}

#[test]
fn duration() -> serde_json::Result<()> {
    let original = [Duration::MIN, Duration::MAX];
    let serialized = "[[-9223372036854775808,-999999999],[9223372036854775807,999999999]]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Duration; 2]>(serialized)?, original);

    Ok(())
}

#[test]
fn weekday() -> serde_json::Result<()> {
    let original = [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
        Weekday::Sunday,
    ];
    let serialized = "[1,2,3,4,5,6,7]";

    assert_eq!(serde_json::to_string(&original)?, serialized);
    assert_eq!(serde_json::from_str::<[Weekday; 7]>(serialized)?, original);

    Ok(())
}
