use standback::convert::TryFrom;
use std::time::Duration as StdDuration;
use time::{error, Date, Duration, Error};

fn conversion_range() -> error::ConversionRange {
    StdDuration::try_from(Duration::seconds(-1)).unwrap_err()
}

fn component_range() -> error::ComponentRange {
    Date::try_from_yo(0, 367).unwrap_err()
}

fn parse() -> error::Parse {
    Date::parse("", " ").unwrap_err()
}

fn format_insufficient() -> error::Format {
    error::Format::InsufficientTypeInformation
}

fn format_std() -> error::Format {
    std::fmt::Error.into()
}

#[test]
fn display() {
    assert_eq!(
        conversion_range().to_string(),
        Error::ConversionRange(conversion_range()).to_string()
    );
    assert_eq!(
        component_range().to_string(),
        Error::ComponentRange(component_range().into()).to_string()
    );
    assert_eq!(parse().to_string(), Error::Parse(parse()).to_string());
    assert_eq!(
        format_insufficient().to_string(),
        Error::Format(format_insufficient()).to_string()
    );
    assert_eq!(
        format_std().to_string(),
        Error::Format(format_std()).to_string()
    );
}

#[test]
#[cfg(feature = "std")]
fn source() {
    use std::error::Error as StdError;

    assert!(Error::from(conversion_range())
        .source()
        .unwrap()
        .is::<error::ConversionRange>());
    assert!(Error::from(component_range())
        .source()
        .unwrap()
        .is::<error::ComponentRange>());
    assert!(Error::from(parse()).source().unwrap().is::<error::Parse>());
    assert!(Error::from(format_insufficient())
        .source()
        .unwrap()
        .is::<error::Format>());
    assert!(Error::from(format_insufficient())
        .source()
        .unwrap()
        .is::<error::Format>());
    assert!(format_insufficient().source().is_none());
    assert!(format_std().source().unwrap().is::<std::fmt::Error>());
}
