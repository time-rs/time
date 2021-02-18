//! Formatting for the time crate.

pub(crate) mod formattable;

use core::fmt;

use crate::format_description::{modifier, Component};
use crate::{error, Date, Time, UtcOffset};

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

#[allow(clippy::missing_docs_in_private_items)]
const WEEKDAY_NAMES: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

/// A trait that indicates the formatted width of the value can be determined.
///
/// Note that this should not be implemented for any signed integers. This forces the caller to
/// write the sign if desired.
pub(crate) trait DigitCount {
    /// The number of digits in the stringified value.
    fn num_digits(self) -> u8;
}
impl DigitCount for u8 {
    fn num_digits(self) -> u8 {
        if self < 10 {
            1
        } else if self < 100 {
            2
        } else {
            3
        }
    }
}
impl DigitCount for u16 {
    fn num_digits(self) -> u8 {
        if self < 10 {
            1
        } else if self < 100 {
            2
        } else if self < 1_000 {
            3
        } else if self < 10_000 {
            4
        } else {
            5
        }
    }
}
impl DigitCount for u32 {
    fn num_digits(self) -> u8 {
        if self < 10 {
            1
        } else if self < 100 {
            2
        } else if self < 1_000 {
            3
        } else if self < 10_000 {
            4
        } else if self < 100_000 {
            5
        } else if self < 1_000_000 {
            6
        } else if self < 10_000_000 {
            7
        } else if self < 100_000_000 {
            8
        } else if self < 1_000_000_000 {
            9
        } else {
            10
        }
    }
}

/// Format a number with the provided padding and width.
///
/// The sign must be written by the caller.
pub(crate) fn format_number(
    output: &mut impl fmt::Write,
    value: impl itoa::Integer + DigitCount + Copy,
    padding: modifier::Padding,
    width: u8,
) -> Result<(), fmt::Error> {
    match padding {
        modifier::Padding::Space => {
            for _ in 0..(width.saturating_sub(value.num_digits())) {
                output.write_char(' ')?;
            }
            itoa::fmt(output, value)
        }
        modifier::Padding::Zero => {
            for _ in 0..(width.saturating_sub(value.num_digits())) {
                output.write_char('0')?;
            }
            itoa::fmt(output, value)
        }
        modifier::Padding::None => itoa::fmt(output, value),
    }
}

/// Format the provided component into the designated output. An `Err` will be returned if the
/// component requires information that it does not provide or if the value cannot be output to the
/// stream.
#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub(crate) fn format_component(
    output: &mut impl fmt::Write,
    component: Component,
    date: Option<Date>,
    time: Option<Time>,
    offset: Option<UtcOffset>,
) -> Result<(), error::Format> {
    match (component, date, time, offset) {
        (Component::Day(modifier::Day { padding }), Some(date), ..) => {
            format_number(output, date.day(), padding, 2)?
        }
        (Component::Month(modifier::Month { padding, repr }), Some(date), ..) => match repr {
            modifier::MonthRepr::Numerical => format_number(output, date.month(), padding, 2)?,
            modifier::MonthRepr::Long => {
                output.write_str(MONTH_NAMES[date.month() as usize - 1])?
            }
            modifier::MonthRepr::Short => {
                output.write_str(&MONTH_NAMES[date.month() as usize - 1][..3])?
            }
        },
        (Component::Ordinal(modifier::Ordinal { padding }), Some(date), ..) => {
            format_number(output, date.ordinal(), padding, 3)?
        }
        (Component::Weekday(modifier::Weekday { repr, one_indexed }), Some(date), ..) => match repr
        {
            modifier::WeekdayRepr::Short => output.write_str(
                &WEEKDAY_NAMES[date.weekday().number_days_from_monday() as usize][..3],
            )?,
            modifier::WeekdayRepr::Long => output
                .write_str(WEEKDAY_NAMES[date.weekday().number_days_from_monday() as usize])?,
            modifier::WeekdayRepr::Sunday => format_number(
                output,
                date.weekday().number_days_from_sunday() + one_indexed as u8,
                modifier::Padding::None,
                1,
            )?,
            modifier::WeekdayRepr::Monday => format_number(
                output,
                date.weekday().number_days_from_monday() + one_indexed as u8,
                modifier::Padding::None,
                1,
            )?,
        },
        (Component::WeekNumber(modifier::WeekNumber { padding, repr }), Some(date), ..) => {
            format_number(
                output,
                match repr {
                    modifier::WeekNumberRepr::Iso => date.iso_week(),
                    modifier::WeekNumberRepr::Sunday => date.sunday_based_week(),
                    modifier::WeekNumberRepr::Monday => date.monday_based_week(),
                },
                padding,
                2,
            )?
        }
        (
            Component::Year(modifier::Year {
                padding,
                repr,
                iso_week_based,
                sign_is_mandatory,
            }),
            Some(date),
            ..,
        ) => {
            let full_year = if iso_week_based {
                date.iso_year_week().0
            } else {
                date.year()
            };

            let value = match repr {
                modifier::YearRepr::Full => full_year,
                modifier::YearRepr::LastTwo => (full_year % 100).abs(),
            };

            let width = match repr {
                #[cfg(feature = "large-dates")]
                modifier::YearRepr::Full if value.abs() >= 100_000 => 6,
                #[cfg(feature = "large-dates")]
                modifier::YearRepr::Full if value.abs() >= 10_000 => 5,
                modifier::YearRepr::Full => 4,
                modifier::YearRepr::LastTwo => 2,
            };

            // Don't emit a sign when only displaying the last two digits.
            if repr != modifier::YearRepr::LastTwo {
                if full_year < 0 {
                    output.write_char('-')?;
                } else if sign_is_mandatory || cfg!(feature = "large-dates") && full_year >= 10_000
                {
                    output.write_char('+')?;
                }
            }

            format_number(output, value.abs() as u32, padding, width)?
        }
        (
            Component::Hour(modifier::Hour {
                padding,
                is_12_hour_clock,
            }),
            _,
            Some(time),
            _,
        ) => {
            let value = match (time.hour, is_12_hour_clock) {
                (hour, false) => hour,
                (0, true) | (12, true) => 12,
                (hour, true) if hour < 12 => hour,
                (hour, true) => hour - 12,
            };
            format_number(output, value, padding, 2)?
        }
        (Component::Minute(modifier::Minute { padding }), _, Some(time), _) => {
            format_number(output, time.minute, padding, 2)?
        }
        (Component::Period(modifier::Period { is_uppercase }), _, Some(time), _) => {
            match (time.hour >= 12, is_uppercase) {
                (false, false) => output.write_str("am"),
                (false, true) => output.write_str("AM"),
                (true, false) => output.write_str("pm"),
                (true, true) => output.write_str("PM"),
            }?
        }
        (Component::Second(modifier::Second { padding }), _, Some(time), _) => {
            format_number(output, time.second, padding, 2)?
        }
        (Component::Subsecond(modifier::Subsecond { digits }), _, Some(time), _) => {
            let (value, width) = match digits {
                modifier::SubsecondDigits::One => (time.nanosecond / 100_000_000, 1),
                modifier::SubsecondDigits::Two => (time.nanosecond / 10_000_000, 2),
                modifier::SubsecondDigits::Three => (time.nanosecond / 1_000_000, 3),
                modifier::SubsecondDigits::Four => (time.nanosecond / 100_000, 4),
                modifier::SubsecondDigits::Five => (time.nanosecond / 10_000, 5),
                modifier::SubsecondDigits::Six => (time.nanosecond / 1_000, 6),
                modifier::SubsecondDigits::Seven => (time.nanosecond / 100, 7),
                modifier::SubsecondDigits::Eight => (time.nanosecond / 10, 8),
                modifier::SubsecondDigits::Nine => (time.nanosecond, 9),
                modifier::SubsecondDigits::OneOrMore => match time.nanosecond {
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
            format_number(output, value, modifier::Padding::Zero, width)?
        }
        (
            Component::OffsetHour(modifier::OffsetHour {
                padding,
                sign_is_mandatory,
            }),
            _,
            _,
            Some(offset),
        ) => {
            if offset.hours < 0 || offset.minutes < 0 || offset.seconds < 0 {
                output.write_char('-')?;
            } else if sign_is_mandatory {
                output.write_char('+')?;
            }
            format_number(output, offset.hours.abs() as u8, padding, 2)?;
        }
        (Component::OffsetMinute(modifier::OffsetMinute { padding }), _, _, Some(offset)) => {
            format_number(output, offset.minutes.abs() as u8, padding, 2)?;
        }
        (Component::OffsetSecond(modifier::OffsetSecond { padding }), _, _, Some(offset)) => {
            format_number(output, offset.seconds.abs() as u8, padding, 2)?;
        }
        _ => return Err(error::Format::InsufficientTypeInformation),
    }

    Ok(())
}
