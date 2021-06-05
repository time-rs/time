use std::error::Error as _;
use std::io;

use time::error::{
    ComponentRange, ConversionRange, Error, Format, IndeterminateOffset, InvalidFormatDescription,
    Parse, ParseFromDescription, TryFromParsed,
};
use time::format_description::modifier::{self, Padding};
use time::format_description::{Component, FormatItem};
use time::macros::format_description;
use time::{Date, Time};

macro_rules! assert_display_eq {
    ($a:expr, $b:expr $(,)?) => {
        assert_eq!($a.to_string(), $b.to_string())
    };
}

macro_rules! assert_source {
    ($err:expr,None $(,)?) => {
        assert!($err.source().is_none())
    };
    ($err:expr, $source:ty $(,)?) => {
        assert!($err.source().unwrap().is::<$source>());
    };
}

fn component_range() -> ComponentRange {
    Date::from_ordinal_date(0, 367).unwrap_err()
}

fn insufficient_type_information() -> Format {
    Time::MIDNIGHT
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_err()
}

fn unexpected_trailing_characters() -> Parse {
    Time::parse("a", &format_description!("")).unwrap_err()
}

fn io_error() -> io::Error {
    io::Error::last_os_error()
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", FormatItem::Literal(b"abcdef")), "abcdef");
    assert_eq!(
        format!(
            "{:?}",
            FormatItem::Compound(&[FormatItem::Component(Component::Day(modifier::Day {
                padding: Padding::Zero
            }))])
        ),
        format!(
            "{:?}",
            FormatItem::Compound(&[FormatItem::Component(Component::Day(modifier::Day {
                padding: Padding::Zero
            }))])
        )
    );
}

#[test]
fn display() {
    assert_display_eq!(ConversionRange, Error::from(ConversionRange));
    assert_display_eq!(component_range(), Error::from(component_range()));
    assert_display_eq!(component_range(), TryFromParsed::from(component_range()));
    assert_display_eq!(IndeterminateOffset, Error::IndeterminateOffset);
    assert_display_eq!(
        TryFromParsed::InsufficientInformation,
        Error::from(TryFromParsed::InsufficientInformation)
    );
    assert_display_eq!(
        insufficient_type_information(),
        Error::from(insufficient_type_information())
    );
    assert_display_eq!(
        Format::InvalidComponent("a"),
        Error::from(Format::InvalidComponent("a"))
    );
    assert_display_eq!(
        ParseFromDescription::InvalidComponent("a"),
        Error::from(Parse::from(ParseFromDescription::InvalidComponent("a")))
    );
    assert_display_eq!(
        component_range(),
        Error::from(Parse::from(TryFromParsed::from(component_range())))
    );
    assert_display_eq!(
        ParseFromDescription::InvalidComponent("a"),
        Parse::from(ParseFromDescription::InvalidComponent("a"))
    );
    assert_display_eq!(
        component_range(),
        Parse::from(TryFromParsed::from(component_range()))
    );
    assert_display_eq!(
        unexpected_trailing_characters(),
        Error::from(unexpected_trailing_characters()),
    );
    assert_display_eq!(
        InvalidFormatDescription::UnclosedOpeningBracket { index: 0 },
        Error::from(InvalidFormatDescription::UnclosedOpeningBracket { index: 0 })
    );
    assert_display_eq!(io_error(), Format::from(io_error()));
}

#[test]
fn source() {
    assert_source!(Error::from(ConversionRange), ConversionRange);
    assert_source!(Error::from(component_range()), ComponentRange);
    assert_source!(TryFromParsed::from(component_range()), ComponentRange);
    assert_source!(TryFromParsed::InsufficientInformation, None);
    assert_source!(insufficient_type_information(), None);
    assert_source!(Format::InvalidComponent("a"), None);
    assert_source!(Error::from(insufficient_type_information()), Format);
    assert_source!(Error::from(IndeterminateOffset), IndeterminateOffset);
    assert_source!(
        Parse::from(TryFromParsed::InsufficientInformation),
        TryFromParsed
    );
    assert_source!(
        Error::from(TryFromParsed::InsufficientInformation),
        TryFromParsed
    );
    assert_source!(
        Parse::from(ParseFromDescription::InvalidComponent("a")),
        ParseFromDescription
    );
    assert_source!(
        Error::from(ParseFromDescription::InvalidComponent("a")),
        ParseFromDescription
    );
    assert_source!(unexpected_trailing_characters(), None);
    assert_source!(Error::from(unexpected_trailing_characters()), None);
    assert_source!(
        Error::from(InvalidFormatDescription::UnclosedOpeningBracket { index: 0 }),
        InvalidFormatDescription
    );
    assert_source!(Format::from(io_error()), io::Error);
}
