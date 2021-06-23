use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

use time::error::{
    self, ConversionRange, IndeterminateOffset, InvalidFormatDescription, TryFromParsed,
};
use time::ext::NumericalDuration;
use time::format_description::{modifier, well_known, Component};
use time::macros::{date, offset, time};
use time::parsing::Parsed;
use time::{Duration, Instant, Month, Time, Weekday};
use time_macros::datetime;

macro_rules! assert_cloned_eq {
    ($x:expr) => {
        assert_eq!($x.clone(), $x)
    };
}

fn component_range_error() -> error::ComponentRange {
    Time::from_hms(24, 0, 0).unwrap_err()
}

#[test]
fn clone() {
    let instant = Instant::now();
    assert_cloned_eq!(date!(2021 - 001));
    assert_cloned_eq!(time!(0:00));
    assert_cloned_eq!(offset!(UTC));
    assert_cloned_eq!(datetime!(2021-001 0:00));
    assert_cloned_eq!(datetime!(2021-001 0:00 UTC));
    assert_cloned_eq!(Weekday::Monday);
    assert_cloned_eq!(Month::January);
    assert_cloned_eq!(Duration::ZERO);
    assert_cloned_eq!(instant);
    assert_cloned_eq!(IndeterminateOffset);
    assert_cloned_eq!(ConversionRange);
    assert_cloned_eq!(InvalidFormatDescription::MissingComponentName { index: 0 });
    assert_cloned_eq!(TryFromParsed::InsufficientInformation);
    let _ = Parsed::new().clone();
    assert_cloned_eq!(error::Parse::ParseFromDescription(
        error::ParseFromDescription::InvalidComponent("foo")
    ));
    assert_cloned_eq!(error::ParseFromDescription::InvalidComponent("foo"));
    assert_cloned_eq!(Component::OffsetSecond(modifier::OffsetSecond {
        padding: Default::default(),
    }));
    assert_cloned_eq!(well_known::Rfc3339);
    assert_cloned_eq!(component_range_error());

    assert_cloned_eq!(modifier::Day::default());
    assert_cloned_eq!(modifier::MonthRepr::default());
    assert_cloned_eq!(modifier::Month::default());
    assert_cloned_eq!(modifier::Ordinal::default());
    assert_cloned_eq!(modifier::WeekdayRepr::default());
    assert_cloned_eq!(modifier::Weekday::default());
    assert_cloned_eq!(modifier::WeekNumberRepr::default());
    assert_cloned_eq!(modifier::WeekNumber::default());
    assert_cloned_eq!(modifier::YearRepr::default());
    assert_cloned_eq!(modifier::Year::default());
    assert_cloned_eq!(modifier::Hour::default());
    assert_cloned_eq!(modifier::Minute::default());
    assert_cloned_eq!(modifier::Period::default());
    assert_cloned_eq!(modifier::Second::default());
    assert_cloned_eq!(modifier::SubsecondDigits::default());
    assert_cloned_eq!(modifier::Subsecond::default());
    assert_cloned_eq!(modifier::OffsetHour::default());
    assert_cloned_eq!(modifier::OffsetMinute::default());
    assert_cloned_eq!(modifier::OffsetSecond::default());
    assert_cloned_eq!(modifier::Padding::default());
}

#[test]
fn hash() {
    let mut hasher = DefaultHasher::new();
    date!(2021 - 001).hash(&mut hasher);
    time!(0:00).hash(&mut hasher);
    offset!(UTC).hash(&mut hasher);
    datetime!(2021-001 0:00).hash(&mut hasher);
    datetime!(2021-001 0:00 UTC).hash(&mut hasher);
    Weekday::Monday.hash(&mut hasher);
    Month::January.hash(&mut hasher);
    Instant::now().hash(&mut hasher);
    Duration::ZERO.hash(&mut hasher);
    component_range_error().hash(&mut hasher);
}

#[test]
fn partial_ord() {
    let instant = Instant::now();
    assert_eq!(offset!(UTC).partial_cmp(&offset!(+1)), Some(Ordering::Less));
    assert_eq!(
        offset!(+1).partial_cmp(&offset!(UTC)),
        Some(Ordering::Greater)
    );
    assert_eq!(
        (instant - 1.seconds()).partial_cmp(&instant),
        Some(Ordering::Less)
    );
    assert_eq!(
        (instant + 1.seconds()).partial_cmp(&instant),
        Some(Ordering::Greater)
    );
}

#[test]
fn ord() {
    assert_eq!(offset!(UTC).cmp(&offset!(+1)), Ordering::Less);
    assert_eq!(offset!(+1).cmp(&offset!(UTC)), Ordering::Greater);
    assert_eq!(offset!(UTC).cmp(&offset!(UTC)), Ordering::Equal);
}

#[test]
fn debug() {
    let _ = format!("{:?}", Duration::ZERO);
    let _ = format!("{:?}", IndeterminateOffset);
    let _ = format!("{:?}", ConversionRange);
    let _ = format!("{:?}", TryFromParsed::InsufficientInformation);
    let _ = format!("{:?}", Parsed::new());
    let _ = format!("{:?}", Instant::now());
    let _ = format!("{:?}", error::ParseFromDescription::InvalidComponent("foo"));
    let _ = format!("{:?}", well_known::Rfc3339);
    let _ = format!("{:?}", component_range_error());

    let _ = format!("{:?}", modifier::Day::default());
    let _ = format!("{:?}", modifier::MonthRepr::default());
    let _ = format!("{:?}", modifier::Month::default());
    let _ = format!("{:?}", modifier::Ordinal::default());
    let _ = format!("{:?}", modifier::WeekdayRepr::default());
    let _ = format!("{:?}", modifier::Weekday::default());
    let _ = format!("{:?}", modifier::WeekNumberRepr::default());
    let _ = format!("{:?}", modifier::WeekNumber::default());
    let _ = format!("{:?}", modifier::YearRepr::default());
    let _ = format!("{:?}", modifier::Year::default());
    let _ = format!("{:?}", modifier::Hour::default());
    let _ = format!("{:?}", modifier::Minute::default());
    let _ = format!("{:?}", modifier::Period::default());
    let _ = format!("{:?}", modifier::Second::default());
    let _ = format!("{:?}", modifier::SubsecondDigits::default());
    let _ = format!("{:?}", modifier::Subsecond::default());
    let _ = format!("{:?}", modifier::OffsetHour::default());
    let _ = format!("{:?}", modifier::OffsetMinute::default());
    let _ = format!("{:?}", modifier::OffsetSecond::default());
    let _ = format!("{:?}", modifier::Padding::default());
}
