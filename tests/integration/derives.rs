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
}

#[test]
fn modifier_clone() {
    use time::format_description::modifier::*;

    assert_cloned_eq!(Day {
        padding: Default::default()
    });
    assert_cloned_eq!(MonthRepr::Numerical);
    assert_cloned_eq!(Month {
        padding: Default::default(),
        repr: Default::default(),
    });
    assert_cloned_eq!(Ordinal {
        padding: Default::default(),
    });
    assert_cloned_eq!(WeekdayRepr::Short);
    assert_cloned_eq!(Weekday {
        repr: Default::default(),
        one_indexed: Default::default(),
    });
    assert_cloned_eq!(WeekNumberRepr::Iso);
    assert_cloned_eq!(WeekNumber {
        padding: Default::default(),
        repr: Default::default(),
    });
    assert_cloned_eq!(YearRepr::Full);
    assert_cloned_eq!(Year {
        padding: Default::default(),
        repr: Default::default(),
        iso_week_based: Default::default(),
        sign_is_mandatory: Default::default(),
    });
    assert_cloned_eq!(Hour {
        padding: Default::default(),
        is_12_hour_clock: Default::default(),
    });
    assert_cloned_eq!(Minute {
        padding: Default::default(),
    });
    assert_cloned_eq!(Period {
        is_uppercase: Default::default(),
    });
    assert_cloned_eq!(Second {
        padding: Default::default(),
    });
    assert_cloned_eq!(SubsecondDigits::One);
    assert_cloned_eq!(Subsecond {
        digits: Default::default(),
    });
    assert_cloned_eq!(OffsetHour {
        sign_is_mandatory: Default::default(),
        padding: Default::default(),
    });
    assert_cloned_eq!(OffsetMinute {
        padding: Default::default(),
    });
    assert_cloned_eq!(OffsetSecond {
        padding: Default::default(),
    });
    assert_cloned_eq!(Padding::Zero);
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
}

#[test]
fn modifier_debug() {
    use time::format_description::modifier::*;

    let _ = format!(
        "{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
        Day {
            padding: Default::default()
        },
        MonthRepr::Numerical,
        Month {
            padding: Default::default(),
            repr: Default::default(),
        },
        Ordinal {
            padding: Default::default(),
        },
        WeekdayRepr::Short,
        Weekday {
            repr: Default::default(),
            one_indexed: Default::default(),
        },
        WeekNumberRepr::Iso,
        WeekNumber {
            padding: Default::default(),
            repr: Default::default(),
        },
        YearRepr::Full,
        Year {
            padding: Default::default(),
            repr: Default::default(),
            iso_week_based: Default::default(),
            sign_is_mandatory: Default::default(),
        },
        Hour {
            padding: Default::default(),
            is_12_hour_clock: Default::default(),
        },
        Minute {
            padding: Default::default(),
        },
        Period {
            is_uppercase: Default::default(),
        },
        Second {
            padding: Default::default(),
        },
        SubsecondDigits::One,
        Subsecond {
            digits: Default::default(),
        },
        OffsetHour {
            sign_is_mandatory: Default::default(),
            padding: Default::default(),
        },
        OffsetMinute {
            padding: Default::default(),
        },
        OffsetSecond {
            padding: Default::default(),
        },
        Padding::Zero,
    );
}
