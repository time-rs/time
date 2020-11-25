//! Implementation of the formatter for all types in the time crate.

use crate::{
    error,
    format_description::{
        modifier::{MonthRepr, Padding, SubsecondDigits, WeekNumberRepr, WeekdayRepr, YearRepr},
        Component, FormatDescription,
    },
    Weekday,
};
use core::fmt;

/// Using the format description provided, write the formatted value to the
/// designated output. An `Err` will be returned if the format description
/// requires information that the components do not provide or the value cannot
/// be output to the stream.
#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub(crate) fn format_into(
    output: &mut dyn fmt::Write,
    description: FormatDescription<'_>,
    date: Option<crate::Date>,
    time: Option<crate::Time>,
    offset: Option<crate::UtcOffset>,
) -> Result<(), error::Format> {
    match description {
        FormatDescription::Literal(literal) => output.write_str(literal)?,
        FormatDescription::Component(component) => match (date, time, offset, component) {
            (Some(date), _, _, Component::Day { padding }) => {
                format_value(output, date.day(), padding, 2)?
            }
            (
                _,
                Some(time),
                _,
                Component::Hour {
                    padding,
                    is_12_hour_clock,
                },
            ) => {
                let value = match (time.hour, is_12_hour_clock) {
                    (hour, false) => hour,
                    (0, true) | (12, true) => 12,
                    (hour, true) if hour < 12 => hour,
                    (hour, true) => hour - 12,
                };
                format_value(output, value, padding, 2)?
            }
            (_, Some(time), _, Component::Minute { padding }) => {
                format_value(output, time.minute, padding, 2)?
            }
            (Some(date), _, _, Component::Month { padding, repr }) => {
                #[allow(clippy::clippy::missing_docs_in_private_items)]
                const MONTH_NAMES: [&str; 12] = [
                    "January",
                    "February",
                    "March",
                    "April",
                    "May",
                    "June",
                    "July",
                    "August",
                    "September",
                    "October",
                    "November",
                    "December",
                ];
                match repr {
                    MonthRepr::Numerical => format_value(output, date.month(), padding, 2)?,
                    MonthRepr::Long => output.write_str(MONTH_NAMES[date.month() as usize - 1])?,
                    MonthRepr::Short => {
                        output.write_str(&MONTH_NAMES[date.month() as usize - 1][..3])?
                    }
                }
            }
            (
                _,
                _,
                Some(offset),
                Component::OffsetHour {
                    padding,
                    sign_is_mandatory,
                },
            ) => {
                if offset.seconds.is_negative() {
                    output.write_char('-')?;
                } else if sign_is_mandatory {
                    output.write_char('+')?;
                }
                format_value(output, (offset.seconds / 3_600).abs(), padding, 2)?
            }
            (_, _, Some(offset), Component::OffsetMinute { padding }) => {
                format_value(output, ((offset.seconds % 3_600) / 60).abs(), padding, 2)?
            }
            (_, _, Some(offset), Component::OffsetSecond { padding }) => {
                format_value(output, (offset.seconds % 60).abs(), padding, 2)?
            }
            (Some(date), _, _, Component::Ordinal { padding }) => {
                format_value(output, date.ordinal(), padding, 3)?
            }
            (_, Some(time), _, Component::Period { is_uppercase }) => {
                match (time.hour >= 12, is_uppercase) {
                    (false, false) => output.write_str("am"),
                    (false, true) => output.write_str("AM"),
                    (true, false) => output.write_str("pm"),
                    (true, true) => output.write_str("PM"),
                }?
            }
            (_, Some(time), _, Component::Second { padding }) => {
                format_value(output, time.second, padding, 2)?
            }
            (_, Some(time), _, Component::Subsecond { digits }) => {
                let (value, width) = match digits {
                    SubsecondDigits::One => (time.nanosecond / 100_000_000, 1),
                    SubsecondDigits::Two => (time.nanosecond / 10_000_000, 2),
                    SubsecondDigits::Three => (time.nanosecond / 1_000_000, 3),
                    SubsecondDigits::Four => (time.nanosecond / 100_000, 4),
                    SubsecondDigits::Five => (time.nanosecond / 10_000, 5),
                    SubsecondDigits::Six => (time.nanosecond / 1_000, 6),
                    SubsecondDigits::Seven => (time.nanosecond / 100, 7),
                    SubsecondDigits::Eight => (time.nanosecond / 10, 8),
                    SubsecondDigits::Nine => (time.nanosecond, 9),
                    SubsecondDigits::OneOrMore => match time.nanosecond {
                        nanos if nanos % 10 != 0 => (nanos, 9),
                        nanos if (nanos / 10) % 10 != 0 => (nanos / 10, 8),
                        nanos if (nanos / 100) % 10 != 0 => (nanos / 100, 7),
                        nanos if (nanos / 1_000) % 10 != 0 => (nanos / 1_000, 6),
                        nanos if (nanos / 10_000) % 10 != 0 => (nanos / 10_000, 5),
                        nanos if (nanos / 100_000) % 10 != 0 => (nanos / 100_000, 4),
                        nanos if (nanos / 1_000_000) % 10 != 0 => (nanos / 1_000_000, 3),
                        nanos if (nanos / 10_000_000) % 10 != 0 => (nanos / 10_000_000, 2),
                        nanos => (nanos / 100_000_000, 1),
                    },
                };
                format_value(output, value, Padding::Zero, width)?
            }
            (Some(date), _, _, Component::Weekday { repr, one_indexed }) => match repr {
                WeekdayRepr::Short => match date.weekday() {
                    Weekday::Monday => output.write_str("Mon"),
                    Weekday::Tuesday => output.write_str("Tue"),
                    Weekday::Wednesday => output.write_str("Wed"),
                    Weekday::Thursday => output.write_str("Thu"),
                    Weekday::Friday => output.write_str("Fri"),
                    Weekday::Saturday => output.write_str("Sat"),
                    Weekday::Sunday => output.write_str("Sun"),
                }?,
                WeekdayRepr::Long => match date.weekday() {
                    Weekday::Monday => output.write_str("Monday"),
                    Weekday::Tuesday => output.write_str("Tuesday"),
                    Weekday::Wednesday => output.write_str("Wednesday"),
                    Weekday::Thursday => output.write_str("Thursday"),
                    Weekday::Friday => output.write_str("Friday"),
                    Weekday::Saturday => output.write_str("Saturday"),
                    Weekday::Sunday => output.write_str("Sunday"),
                }?,
                WeekdayRepr::Sunday => format_value(
                    output,
                    date.weekday().number_days_from_sunday() + one_indexed as u8,
                    Padding::None,
                    1,
                )?,
                WeekdayRepr::Monday => format_value(
                    output,
                    date.weekday().number_days_from_monday() + one_indexed as u8,
                    Padding::None,
                    1,
                )?,
            },
            (Some(date), _, _, Component::WeekNumber { padding, repr }) => format_value(
                output,
                match repr {
                    WeekNumberRepr::Iso => date.week(),
                    WeekNumberRepr::Sunday => date.sunday_based_week(),
                    WeekNumberRepr::Monday => date.monday_based_week(),
                },
                padding,
                2,
            )?,
            (
                Some(date),
                _,
                _,
                Component::Year {
                    padding,
                    repr,
                    iso_week_based,
                    sign_is_mandatory,
                },
            ) => {
                let full_year = if iso_week_based {
                    date.iso_year_week().0
                } else {
                    date.year()
                };

                let value = match repr {
                    YearRepr::Full => full_year,
                    YearRepr::Century => full_year / 100,
                    YearRepr::LastTwo => (full_year % 100).abs(),
                };

                let width = match repr {
                    YearRepr::Full if value.abs() >= 100_000 => 6,
                    YearRepr::Full if value.abs() >= 10_000 => 5,
                    YearRepr::Full => 4,
                    YearRepr::Century if value.abs() >= 1_000 => 4,
                    YearRepr::Century if value.abs() >= 100 => 3,
                    YearRepr::Century | YearRepr::LastTwo => 2,
                };

                // Don't emit a sign when only displaying the last two digits.
                if repr != YearRepr::LastTwo && (sign_is_mandatory || full_year >= 10_000) {
                    output.write_char('+')?;
                }

                format_value(output, value, padding, width)?
            }
            _ => return Err(error::Format::InsufficientTypeInformation),
        },
        FormatDescription::Compound(descriptions) => {
            for &description in descriptions {
                format_into(output, description, date, time, offset)?;
            }
        }
    }

    Ok(())
}

/// Format a value with the provided padding and width.
fn format_value<T: fmt::Display>(
    output: &mut dyn fmt::Write,
    value: T,
    padding: Padding,
    width: usize,
) -> Result<(), fmt::Error> {
    match padding {
        Padding::Space => write!(output, "{: >width$}", value, width = width),
        Padding::Zero => write!(output, "{:0>width$}", value, width = width),
        Padding::None => write!(output, "{}", value),
    }
}
