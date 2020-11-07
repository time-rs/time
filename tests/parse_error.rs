#[cfg(feature = "alloc")]
use time::error;

#[cfg(feature = "alloc")]
fn component_range() -> error::ComponentRange {
    time::Date::from_yo(0, 367).unwrap_err()
}

// The exact message doesn't matter; just make sure it doesn't panic.
#[test]
#[cfg(feature = "alloc")]
fn display() {
    let _ = error::Parse::InvalidNanosecond.to_string();
    let _ = error::Parse::InvalidSecond.to_string();
    let _ = error::Parse::InvalidMinute.to_string();
    let _ = error::Parse::InvalidHour.to_string();
    let _ = error::Parse::InvalidAmPm.to_string();
    let _ = error::Parse::InvalidMonth.to_string();
    let _ = error::Parse::InvalidYear.to_string();
    let _ = error::Parse::InvalidWeek.to_string();
    let _ = error::Parse::InvalidDayOfWeek.to_string();
    let _ = error::Parse::InvalidDayOfMonth.to_string();
    let _ = error::Parse::InvalidDayOfYear.to_string();
    let _ = error::Parse::InvalidOffset.to_string();
    let _ = error::Parse::MissingFormatSpecifier.to_string();
    let _ = error::Parse::InvalidFormatSpecifier('!').to_string();
    let _ = error::Parse::UnexpectedCharacter {
        expected: 'a',
        actual: 'b',
    }
    .to_string();
    let _ = error::Parse::UnexpectedEndOfString.to_string();
    let _ = error::Parse::InsufficientInformation.to_string();
    let _ = error::Parse::ComponentOutOfRange(component_range()).to_string();
}

#[test]
#[cfg(feature = "std")]
fn source() {
    use std::error::Error as StdError;

    assert!(error::Parse::from(component_range())
        .source()
        .unwrap()
        .is::<error::ComponentRange>());
    assert!(error::Parse::InvalidNanosecond.source().is_none());
}
