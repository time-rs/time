#[cfg(feature = "alloc")]
use time::{error, Date, Error};

#[cfg(feature = "alloc")]
fn component_range() -> error::ComponentRange {
    Date::from_yo(0, 367).unwrap_err()
}

#[test]
#[cfg(feature = "alloc")]
fn display() {
    assert_eq!(
        error::ConversionRange.to_string(),
        Error::ConversionRange.to_string()
    );
    assert_eq!(
        component_range().to_string(),
        Error::ComponentRange(component_range()).to_string()
    );
}

#[test]
#[cfg(feature = "std")]
fn source() {
    use std::error::Error as StdError;

    assert!(Error::from(error::ConversionRange)
        .source()
        .unwrap()
        .is::<error::ConversionRange>());
    assert!(Error::from(component_range())
        .source()
        .unwrap()
        .is::<error::ComponentRange>());
}
