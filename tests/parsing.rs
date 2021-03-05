#![cfg(feature = "parsing")]

use core::convert::{TryFrom, TryInto};

use time::format_description::well_known::Rfc3339;
use time::format_description::{modifier, Component};
use time::macros::{date, datetime, time};
use time::parsing::Parsed;
use time::{format_description as fd, Date, OffsetDateTime, Time, UtcOffset, Weekday};

#[test]
fn rfc_3339() -> time::Result<()> {
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.1 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.12 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1234Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123_4 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12345Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123_45 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123_456 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1234567Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123_456_7 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12345678Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123_456_78 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789Z", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123_456_789 UTC"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789-01:02", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123_456_789 -01:02"),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789+01:02", &Rfc3339)?,
        datetime!("2021-01-02 03:04:05.123_456_789 +01:02"),
    );

    assert_eq!(
        Date::parse("2021-01-02T03:04:05Z", &Rfc3339)?,
        date!("2021-01-02"),
    );
    assert_eq!(
        Date::parse("2021-01-02T03:04:05.123+01:02", &Rfc3339)?,
        date!("2021-01-02"),
    );
    assert_eq!(
        Date::parse("2021-01-02T03:04:05.123-01:02", &Rfc3339)?,
        date!("2021-01-02"),
    );
    assert!(matches!(
        OffsetDateTime::parse("2021-01-02T03:04:05Z ", &Rfc3339),
        Err(time::error::Parse::UnexpectedTrailingCharacters { .. })
    ));

    Ok(())
}

#[test]
fn parse_time() -> time::Result<()> {
    let format_input_output = [
        (
            fd::parse("[hour]:[minute]:[second]")?,
            "13:02:03",
            time!("13:02:03"),
        ),
        (
            fd::parse("[hour repr:12]:[minute] [period]")?,
            "01:02 PM",
            time!("1:02 PM"),
        ),
        (fd::parse("[hour]:[minute]")?, "01:02", time!("1:02")),
        (
            fd::parse("[hour repr:12]:[minute] [period]")?,
            "01:02 AM",
            time!("1:02 AM"),
        ),
    ];

    for (format_description, input, output) in &format_input_output {
        assert_eq!(&Time::parse(input, format_description)?, output);
    }

    assert!(matches!(
        Time::try_from(Parsed::new()),
        Err(time::error::TryFromParsed::InsufficientInformation { .. })
    ));
    assert!(matches!(
        Time::parse("12", &fd::parse("[hour]")?),
        Err(time::error::Parse::TryFromParsed(
            time::error::TryFromParsed::InsufficientInformation { .. }
        ))
    ));
    assert!(matches!(
        Time::parse(" ", &fd::parse("")?),
        Err(time::error::Parse::UnexpectedTrailingCharacters { .. })
    ));

    Ok(())
}

#[test]
fn parse_date() -> time::Result<()> {
    let format_input_output = [
        (
            fd::parse("[year]-[month]-[day]")?,
            "2021-01-02",
            date!("2021-01-02"),
        ),
        (
            fd::parse("[year]-[ordinal]")?,
            "2021-002",
            date!("2021-002"),
        ),
        (
            fd::parse("[year base:iso_week]-W[week_number]-[weekday repr:monday]")?,
            "2020-W53-6",
            date!("2021-01-02"),
        ),
        (
            fd::parse("[year]-W[week_number repr:monday]-[weekday repr:monday]")?,
            "2021-W00-6",
            date!("2021-01-02"),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2021-W00-6",
            date!("2021-01-02"),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2023-W01-1",
            date!("2023-01-02"),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2022-W00-7",
            date!("2022-01-02"),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2026-W00-5",
            date!("2026-01-02"),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2025-W00-4",
            date!("2025-01-02"),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2019-W00-3",
            date!("2019-01-02"),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2018-W01-2",
            date!("2018-01-02"),
        ),
    ];

    for (format_description, input, output) in &format_input_output {
        assert_eq!(&Date::parse(input, format_description)?, output);
    }

    assert!(matches!(
        Date::try_from(Parsed::new()),
        Err(time::error::TryFromParsed::InsufficientInformation { .. })
    ));

    Ok(())
}

#[test]
fn parse_offset() -> time::Result<()> {
    assert_eq!(
        UtcOffset::parse("01", &fd::parse("[offset_hour sign:mandatory]")?),
        Err(time::error::Parse::ParseFromDescription(
            time::error::ParseFromDescription::InvalidComponent("offset hour")
        ))
    );

    Ok(())
}

#[test]
fn parse_components() -> time::Result<()> {
    macro_rules! parse_component {
        ($component:expr, $input:expr,_. $property:ident == $expected:expr) => {
            let mut parsed = Parsed::new();
            parsed.parse_component($input.as_bytes(), $component)?;
            assert_eq!(parsed.$property, $expected);
        };
    }

    parse_component!(
        Component::Year(modifier::Year {
            padding: modifier::Padding::Zero,
            repr: modifier::YearRepr::Full,
            iso_week_based: false,
            sign_is_mandatory: false,
        }),
        "2021",
        _.year == Some(2021)
    );
    parse_component!(
        Component::Year(modifier::Year {
            padding: modifier::Padding::Zero,
            repr: modifier::YearRepr::LastTwo,
            iso_week_based: false,
            sign_is_mandatory: false,
        }),
        "21",
        _.year_last_two == Some(21)
    );
    parse_component!(
        Component::Year(modifier::Year {
            padding: modifier::Padding::Zero,
            repr: modifier::YearRepr::Full,
            iso_week_based: true,
            sign_is_mandatory: false,
        }),
        "2021",
        _.iso_year == Some(2021)
    );
    parse_component!(
        Component::Year(modifier::Year {
            padding: modifier::Padding::Zero,
            repr: modifier::YearRepr::LastTwo,
            iso_week_based: true,
            sign_is_mandatory: false,
        }),
        "21",
        _.iso_year_last_two == Some(21)
    );
    parse_component!(
        Component::Month(modifier::Month {
            padding: modifier::Padding::Space,
            repr: modifier::MonthRepr::Numerical,
        }),
        " 1",
        _.month == 1.try_into().ok()
    );
    parse_component!(
        Component::Month(modifier::Month {
            padding: modifier::Padding::None,
            repr: modifier::MonthRepr::Short,
        }),
        "Jan",
        _.month == 1.try_into().ok()
    );
    parse_component!(
        Component::Month(modifier::Month {
            padding: modifier::Padding::None,
            repr: modifier::MonthRepr::Long,
        }),
        "January",
        _.month == 1.try_into().ok()
    );
    parse_component!(
        Component::Ordinal(modifier::Ordinal {
            padding: modifier::Padding::Zero,
        }),
        "012",
        _.ordinal == 12.try_into().ok()
    );
    parse_component!(
        Component::Weekday(modifier::Weekday {
            repr: modifier::WeekdayRepr::Short,
            one_indexed: false,
        }),
        "Sun",
        _.weekday == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier::Weekday {
            repr: modifier::WeekdayRepr::Long,
            one_indexed: false,
        }),
        "Sunday",
        _.weekday == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier::Weekday {
            repr: modifier::WeekdayRepr::Sunday,
            one_indexed: false,
        }),
        "0",
        _.weekday == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier::Weekday {
            repr: modifier::WeekdayRepr::Sunday,
            one_indexed: true,
        }),
        "1",
        _.weekday == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier::Weekday {
            repr: modifier::WeekdayRepr::Monday,
            one_indexed: false,
        }),
        "6",
        _.weekday == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier::Weekday {
            repr: modifier::WeekdayRepr::Monday,
            one_indexed: true,
        }),
        "7",
        _.weekday == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::WeekNumber(modifier::WeekNumber {
            padding: modifier::Padding::None,
            repr: modifier::WeekNumberRepr::Sunday,
        }),
        "2",
        _.sunday_week_number == Some(2)
    );
    parse_component!(
        Component::WeekNumber(modifier::WeekNumber {
            padding: modifier::Padding::None,
            repr: modifier::WeekNumberRepr::Monday,
        }),
        "2",
        _.monday_week_number == Some(2)
    );
    parse_component!(
        Component::WeekNumber(modifier::WeekNumber {
            padding: modifier::Padding::None,
            repr: modifier::WeekNumberRepr::Iso,
        }),
        "2",
        _.iso_week_number == 2.try_into().ok()
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::One
        }),
        "1",
        _.subsecond == Some(100_000_000)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::Two
        }),
        "12",
        _.subsecond == Some(120_000_000)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::Three
        }),
        "123",
        _.subsecond == Some(123_000_000)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::Four
        }),
        "1234",
        _.subsecond == Some(123_400_000)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::Five
        }),
        "12345",
        _.subsecond == Some(123_450_000)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::Six
        }),
        "123456",
        _.subsecond == Some(123_456_000)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::Seven
        }),
        "1234567",
        _.subsecond == Some(123_456_700)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::Eight
        }),
        "12345678",
        _.subsecond == Some(123_456_780)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::Nine
        }),
        "123456789",
        _.subsecond == Some(123_456_789)
    );
    parse_component!(
        Component::Subsecond(modifier::Subsecond {
            digits: modifier::SubsecondDigits::OneOrMore
        }),
        "123456789",
        _.subsecond == Some(123_456_789)
    );

    Ok(())
}
