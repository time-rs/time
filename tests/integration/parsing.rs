use core::convert::{TryFrom, TryInto};
use std::num::NonZeroU8;

use time::format_description::well_known::Rfc3339;
use time::format_description::{modifier, Component, FormatItem};
use time::macros::{date, datetime, time};
use time::parsing::Parsed;
use time::{
    error, format_description as fd, Date, Month, OffsetDateTime, PrimitiveDateTime, Time,
    UtcOffset, Weekday,
};

#[test]
fn rfc_3339() -> time::Result<()> {
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:60Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:59 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.1 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.12 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1234Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_4 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12345Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_45 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.1234567Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_7 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.12345678Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_78 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789Z", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_789 UTC),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789-01:02", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_789 -01:02),
    );
    assert_eq!(
        OffsetDateTime::parse("2021-01-02T03:04:05.123456789+01:02", &Rfc3339)?,
        datetime!(2021-01-02 03:04:05.123_456_789 +01:02),
    );

    assert_eq!(
        Date::parse("2021-01-02T03:04:05Z", &Rfc3339)?,
        date!(2021 - 01 - 02),
    );
    assert_eq!(
        Date::parse("2021-01-02T03:04:05.123+01:02", &Rfc3339)?,
        date!(2021 - 01 - 02),
    );
    assert_eq!(
        Date::parse("2021-01-02T03:04:05.123-01:02", &Rfc3339)?,
        date!(2021 - 01 - 02),
    );
    assert_eq!(
        Time::parse("2021-01-02T03:04:60Z", &Rfc3339)?,
        time!(03:04:59)
    );

    Ok(())
}

#[test]
fn rfc_3339_err() {
    macro_rules! invalid_literal {
        () => {
            Err(error::Parse::ParseFromDescription(
                error::ParseFromDescription::InvalidLiteral { .. },
            ))
        };
    }
    macro_rules! invalid_component {
        ($name:literal) => {
            Err(error::Parse::ParseFromDescription(
                error::ParseFromDescription::InvalidComponent($name),
            ))
        };
    }

    assert!(matches!(
        PrimitiveDateTime::parse("x", &Rfc3339),
        invalid_component!("year")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-x", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-0", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-0", &Rfc3339),
        invalid_component!("day")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T0", &Rfc3339),
        invalid_component!("hour")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:0", &Rfc3339),
        invalid_component!("minute")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:0", &Rfc3339),
        invalid_component!("second")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00.x", &Rfc3339),
        invalid_component!("subsecond")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00x", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00+0", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00+00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-01-01T00:00:00+00:0", &Rfc3339),
        invalid_component!("offset minute")
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-13-01T00:00:00Z", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "month"
    ));

    assert!(matches!(
        OffsetDateTime::parse("2021-01-02T03:04:05Z ", &Rfc3339),
        Err(error::Parse::UnexpectedTrailingCharacters { .. })
    ));
    assert!(matches!(
        OffsetDateTime::parse("x", &Rfc3339),
        invalid_component!("year")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-x", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-0", &Rfc3339),
        invalid_component!("month")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-0", &Rfc3339),
        invalid_component!("day")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T0", &Rfc3339),
        invalid_component!("hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:0", &Rfc3339),
        invalid_component!("minute")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:0", &Rfc3339),
        invalid_component!("second")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00.x", &Rfc3339),
        invalid_component!("subsecond")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00x", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+0", &Rfc3339),
        invalid_component!("offset hour")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+00x", &Rfc3339),
        invalid_literal!()
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+00:0", &Rfc3339),
        invalid_component!("offset minute")
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+24:00", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "offset hour"
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-01-01T00:00:00+00:60", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "offset minute"
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-13-01T00:00:00Z", &Rfc3339),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "month"
    ));
}

#[test]
fn parse_time() -> time::Result<()> {
    let format_input_output = [
        (fd::parse("[hour repr:12] [period]")?, "01 PM", time!(1 PM)),
        (
            fd::parse("[hour]:[minute]:[second]")?,
            "13:02:03",
            time!(13:02:03),
        ),
        (
            fd::parse("[hour repr:12]:[minute] [period]")?,
            "01:02 PM",
            time!(1:02 PM),
        ),
        (fd::parse("[hour]:[minute]")?, "01:02", time!(1:02)),
        (
            fd::parse("[hour repr:12]:[minute] [period]")?,
            "01:02 AM",
            time!(1:02 AM),
        ),
        (fd::parse("[hour]:[minute]")?, "01:02", time!(1:02)),
        (fd::parse("[hour repr:12] [period]")?, "12 AM", time!(12 AM)),
        (fd::parse("[hour repr:12] [period]")?, "12 PM", time!(12 PM)),
    ];

    for (format_description, input, output) in &format_input_output {
        assert_eq!(&Time::parse(input, format_description)?, output);
    }

    Ok(())
}

#[test]
fn parse_time_err() -> time::Result<()> {
    assert!(matches!(
        Time::try_from(Parsed::new()),
        Err(error::TryFromParsed::InsufficientInformation { .. })
    ));
    assert!(matches!(
        Time::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation { .. }
        ))
    ));
    assert!(matches!(
        Time::parse("12", &fd::parse("[hour]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation { .. }
        ))
    ));
    assert!(matches!(
        Time::parse("13 PM", &fd::parse("[hour repr:12] [period]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "hour"
    ));
    assert!(matches!(
        Time::parse(" ", &fd::parse("")?),
        Err(error::Parse::UnexpectedTrailingCharacters { .. })
    ));
    assert!(matches!(
        Time::parse("a", &fd::parse("[subsecond digits:1]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("1a", &fd::parse("[subsecond digits:2]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("12a", &fd::parse("[subsecond digits:3]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("123a", &fd::parse("[subsecond digits:4]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("1234a", &fd::parse("[subsecond digits:5]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("12345a", &fd::parse("[subsecond digits:6]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("123456a", &fd::parse("[subsecond digits:7]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("1234567a", &fd::parse("[subsecond digits:8]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));
    assert!(matches!(
        Time::parse("12345678a", &fd::parse("[subsecond digits:9]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("subsecond")
        ))
    ));

    Ok(())
}

#[test]
fn parse_date() -> time::Result<()> {
    let format_input_output = [
        (
            fd::parse("[year]-[month]-[day]")?,
            "2021-01-02",
            date!(2021 - 01 - 02),
        ),
        (
            fd::parse("[year]-[ordinal]")?,
            "2021-002",
            date!(2021 - 002),
        ),
        (
            fd::parse("[year base:iso_week]-W[week_number]-[weekday repr:monday]")?,
            "2020-W53-6",
            date!(2021 - 01 - 02),
        ),
        (
            fd::parse("[year]-W[week_number repr:monday]-[weekday repr:monday]")?,
            "2021-W00-6",
            date!(2021 - 01 - 02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2021-W00-6",
            date!(2021 - 01 - 02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2023-W01-1",
            date!(2023 - 01 - 02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2022-W00-7",
            date!(2022 - 01 - 02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2026-W00-5",
            date!(2026 - 01 - 02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2025-W00-4",
            date!(2025 - 01 - 02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2019-W00-3",
            date!(2019 - 01 - 02),
        ),
        (
            fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:sunday]")?,
            "2018-W01-2",
            date!(2018 - 01 - 02),
        ),
        (
            fd::parse(
                "[year padding:space]-W[week_number repr:sunday padding:none]-[weekday \
                 repr:sunday]",
            )?,
            " 2018-W01-2",
            date!(2018 - 01 - 02),
        ),
    ];

    for (format_description, input, output) in &format_input_output {
        assert_eq!(&Date::parse(input, format_description)?, output);
    }

    Ok(())
}

#[test]
fn parse_date_err() -> time::Result<()> {
    assert!(matches!(
        Date::try_from(Parsed::new()),
        Err(error::TryFromParsed::InsufficientInformation { .. })
    ));
    assert!(matches!(
        Date::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation { .. }
        ))
    ));
    assert!(matches!(
        Date::parse("a", &fd::parse("[year]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        Date::parse("0001", &fd::parse("[year sign:mandatory]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        Date::parse("0a", &fd::parse("[year repr:last_two]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        Date::parse("2021-366", &fd::parse("[year]-[ordinal]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "ordinal"
    ));
    assert!(matches!(
        Date::parse("2021-12-32", &fd::parse("[year]-[month]-[day]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "day"
    ));
    assert!(matches!(
        Date::parse("2021-W54-1", &fd::parse("[year base:iso_week]-W[week_number]-[weekday repr:monday]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "week"
    ));
    assert!(matches!(
        Date::parse("2021-W54-1", &fd::parse("[year]-W[week_number repr:sunday]-[weekday repr:monday]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "ordinal"
    ));
    assert!(matches!(
        Date::parse("2021-W54-1", &fd::parse("[year]-W[week_number repr:monday]-[weekday repr:monday]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "ordinal"
    ));
    assert!(matches!(
        Date::parse("Ja", &fd::parse("[month repr:short]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("month")
        ))
    ));
    assert!(matches!(
        Date::parse("  2a21", &fd::parse("[year padding:space]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));

    Ok(())
}

#[test]
fn parse_offset_err() -> time::Result<()> {
    assert!(matches!(
        UtcOffset::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation { .. }
        ))
    ));
    assert_eq!(
        UtcOffset::parse("01", &fd::parse("[offset_hour sign:mandatory]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("offset hour")
        ))
    );
    assert!(matches!(
        UtcOffset::parse("24", &fd::parse("[offset_hour]")?),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "offset hour"
    ));
    assert!(matches!(
        UtcOffset::parse("00:60", &fd::parse("[offset_hour]:[offset_minute]")?),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "offset minute"
    ));
    assert!(matches!(
        UtcOffset::parse("00:00:60", &fd::parse("[offset_hour]:[offset_minute]:[offset_second]")?),
        Err(error::Parse::TryFromParsed(error::TryFromParsed::ComponentRange(component))) if component.name() == "offset second"
    ));

    Ok(())
}

#[test]
fn parse_primitive_date_time_err() -> time::Result<()> {
    assert!(matches!(
        PrimitiveDateTime::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation { .. }
        ))
    ));
    assert!(matches!(
        PrimitiveDateTime::parse("2021-001 13 PM", &fd::parse("[year]-[ordinal] [hour repr:12] [period]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "hour"
    ));

    Ok(())
}

#[test]
fn parse_offset_date_time_err() -> time::Result<()> {
    assert!(matches!(
        OffsetDateTime::parse("", &fd::parse("")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::InsufficientInformation { .. }
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("x", &fd::parse("[year]")?),
        Err(error::Parse::ParseFromDescription(
            error::ParseFromDescription::InvalidComponent("year")
        ))
    ));
    assert!(matches!(
        OffsetDateTime::parse("2021-001 12 PM +25", &fd::parse("[year]-[ordinal] [hour repr:12] [period] [offset_hour sign:mandatory]")?),
        Err(error::Parse::TryFromParsed(
            error::TryFromParsed::ComponentRange(component)
        )) if component.name() == "offset hour"
    ));

    Ok(())
}

#[test]
fn parse_components() -> time::Result<()> {
    macro_rules! parse_component {
        ($component:expr, $input:expr,_. $property:ident() == $expected:expr) => {
            let mut parsed = Parsed::new();
            parsed.parse_component($input, $component)?;
            assert_eq!(parsed.$property(), $expected);
        };
    }

    parse_component!(
        Component::Year(modifier!(Year {
            padding: modifier::Padding::Zero,
            repr: modifier::YearRepr::Full,
            iso_week_based: false,
            sign_is_mandatory: false,
        })),
        b"2021",
        _.year() == Some(2021)
    );
    parse_component!(
        Component::Year(modifier!(Year {
            padding: modifier::Padding::Zero,
            repr: modifier::YearRepr::LastTwo,
            iso_week_based: false,
            sign_is_mandatory: false,
        })),
        b"21",
        _.year_last_two() == Some(21)
    );
    parse_component!(
        Component::Year(modifier!(Year {
            padding: modifier::Padding::Zero,
            repr: modifier::YearRepr::Full,
            iso_week_based: true,
            sign_is_mandatory: false,
        })),
        b"2021",
        _.iso_year() == Some(2021)
    );
    parse_component!(
        Component::Year(modifier!(Year {
            padding: modifier::Padding::Zero,
            repr: modifier::YearRepr::LastTwo,
            iso_week_based: true,
            sign_is_mandatory: false,
        })),
        b"21",
        _.iso_year_last_two() == Some(21)
    );
    parse_component!(
        Component::Month(modifier!(Month {
            padding: modifier::Padding::Space,
            repr: modifier::MonthRepr::Numerical,
        })),
        b" 1",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Month(modifier!(Month {
            padding: modifier::Padding::None,
            repr: modifier::MonthRepr::Short,
            case_sensitive: true,
        })),
        b"Jan",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Month(modifier!(Month {
            padding: modifier::Padding::None,
            repr: modifier::MonthRepr::Short,
            case_sensitive: false,
        })),
        b"jAn",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Month(modifier!(Month {
            padding: modifier::Padding::None,
            repr: modifier::MonthRepr::Long,
            case_sensitive: true,
        })),
        b"January",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Month(modifier!(Month {
            padding: modifier::Padding::None,
            repr: modifier::MonthRepr::Long,
            case_sensitive: false,
        })),
        b"jAnUaRy",
        _.month() == Some(Month::January)
    );
    parse_component!(
        Component::Ordinal(modifier!(Ordinal {
            padding: modifier::Padding::Zero,
        })),
        b"012",
        _.ordinal() == 12.try_into().ok()
    );
    parse_component!(
        Component::Weekday(modifier!(Weekday {
            repr: modifier::WeekdayRepr::Short,
            one_indexed: false,
            case_sensitive: true,
        })),
        b"Sun",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier!(Weekday {
            repr: modifier::WeekdayRepr::Short,
            one_indexed: false,
            case_sensitive: false,
        })),
        b"sUn",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier!(Weekday {
            repr: modifier::WeekdayRepr::Long,
            one_indexed: false,
            case_sensitive: true,
        })),
        b"Sunday",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier!(Weekday {
            repr: modifier::WeekdayRepr::Long,
            one_indexed: false,
            case_sensitive: false,
        })),
        b"sUnDaY",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier!(Weekday {
            repr: modifier::WeekdayRepr::Sunday,
            one_indexed: false,
        })),
        b"0",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier!(Weekday {
            repr: modifier::WeekdayRepr::Sunday,
            one_indexed: true,
        })),
        b"1",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier!(Weekday {
            repr: modifier::WeekdayRepr::Monday,
            one_indexed: false,
        })),
        b"6",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::Weekday(modifier!(Weekday {
            repr: modifier::WeekdayRepr::Monday,
            one_indexed: true,
        })),
        b"7",
        _.weekday() == Some(Weekday::Sunday)
    );
    parse_component!(
        Component::WeekNumber(modifier!(WeekNumber {
            padding: modifier::Padding::None,
            repr: modifier::WeekNumberRepr::Sunday,
        })),
        b"2",
        _.sunday_week_number() == Some(2)
    );
    parse_component!(
        Component::WeekNumber(modifier!(WeekNumber {
            padding: modifier::Padding::None,
            repr: modifier::WeekNumberRepr::Monday,
        })),
        b"2",
        _.monday_week_number() == Some(2)
    );
    parse_component!(
        Component::WeekNumber(modifier!(WeekNumber {
            padding: modifier::Padding::None,
            repr: modifier::WeekNumberRepr::Iso,
        })),
        b"2",
        _.iso_week_number() == 2.try_into().ok()
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::One
        })),
        b"1",
        _.subsecond() == Some(100_000_000)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::Two
        })),
        b"12",
        _.subsecond() == Some(120_000_000)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::Three
        })),
        b"123",
        _.subsecond() == Some(123_000_000)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::Four
        })),
        b"1234",
        _.subsecond() == Some(123_400_000)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::Five
        })),
        b"12345",
        _.subsecond() == Some(123_450_000)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::Six
        })),
        b"123456",
        _.subsecond() == Some(123_456_000)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::Seven
        })),
        b"1234567",
        _.subsecond() == Some(123_456_700)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::Eight
        })),
        b"12345678",
        _.subsecond() == Some(123_456_780)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::Nine
        })),
        b"123456789",
        _.subsecond() == Some(123_456_789)
    );
    parse_component!(
        Component::Subsecond(modifier!(Subsecond {
            digits: modifier::SubsecondDigits::OneOrMore
        })),
        b"123456789",
        _.subsecond() == Some(123_456_789)
    );
    parse_component!(
        Component::Period(modifier!(Period {
            is_uppercase: false,
            case_sensitive: true,
        })),
        b"am",
        _.hour_12_is_pm() == Some(false)
    );
    parse_component!(
        Component::Period(modifier!(Period {
            is_uppercase: false,
            case_sensitive: false,
        })),
        b"aM",
        _.hour_12_is_pm() == Some(false)
    );

    Ok(())
}

#[test]
fn parse_optional() -> time::Result<()> {
    // Ensure full parsing works as expected.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &FormatItem::Optional(&FormatItem::Compound(&fd::parse("[year]-[month]-[day]")?)),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZeroU8::get), Some(2));

    // Ensure a successful partial parse *does not* mutate `parsed`.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01",
        &FormatItem::Optional(&FormatItem::Compound(&fd::parse("[year]-[month]-[day]")?)),
    )?;
    assert_eq!(remaining_input, b"2021-01");
    assert!(parsed.year().is_none());
    assert!(parsed.month().is_none());
    assert!(parsed.day().is_none());

    Ok(())
}

#[test]
fn parse_first() -> time::Result<()> {
    // Ensure the first item is parsed correctly.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &FormatItem::First(&[FormatItem::Compound(&fd::parse("[year]-[month]-[day]")?)]),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZeroU8::get), Some(2));

    // Ensure an empty slice is a no-op success.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(b"2021-01-02", &FormatItem::First(&[]))?;
    assert_eq!(remaining_input, b"2021-01-02");
    assert!(parsed.year().is_none());
    assert!(parsed.month().is_none());
    assert!(parsed.day().is_none());

    // Ensure success when the first item fails.
    let mut parsed = Parsed::new();
    let remaining_input = parsed.parse_item(
        b"2021-01-02",
        &FormatItem::First(&[
            FormatItem::Compound(&fd::parse("[period]")?),
            FormatItem::Compound(&fd::parse("x")?),
            FormatItem::Compound(&fd::parse("[year]-[month]-[day]")?),
        ]),
    )?;
    assert!(remaining_input.is_empty());
    assert_eq!(parsed.year(), Some(2021));
    assert_eq!(parsed.month(), Some(Month::January));
    assert_eq!(parsed.day().map(NonZeroU8::get), Some(2));

    // Ensure the first error is returned.
    let mut parsed = Parsed::new();
    let err = parsed
        .parse_item(
            b"2021-01-02",
            &FormatItem::First(&[
                FormatItem::Compound(&fd::parse("[period]")?),
                FormatItem::Compound(&fd::parse("x")?),
            ]),
        )
        .unwrap_err();
    assert_eq!(err, error::ParseFromDescription::InvalidComponent("period"));

    Ok(())
}
