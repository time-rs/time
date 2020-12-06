//! Format implementations for the `Date` struct.

use crate::{
    format_description::{
        component,
        modifier::{MonthRepr, Padding, WeekNumberRepr, WeekdayRepr, YearRepr},
    },
    formatting::format_value,
    Date, Weekday,
};
use core::fmt;

impl component::Date {
    /// Write the formatted value to the designated output. An `Err` will be
    /// returned if the value cannot be output to the stream.
    #[allow(clippy::too_many_lines)] // TODO remove this
    pub(super) fn format_into(
        self,
        output: &mut dyn fmt::Write,
        date: Date,
    ) -> Result<(), fmt::Error> {
        match self {
            Self::Day { padding } => format_value(output, date.day(), padding, 2)?,
            Self::Month { padding, repr } => {
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
            Self::Ordinal { padding } => format_value(output, date.ordinal(), padding, 3)?,
            Self::Weekday { repr, one_indexed } => match repr {
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
            Self::WeekNumber { padding, repr } => format_value(
                output,
                match repr {
                    WeekNumberRepr::Iso => date.iso_week(),
                    WeekNumberRepr::Sunday => date.sunday_based_week(),
                    WeekNumberRepr::Monday => date.monday_based_week(),
                },
                padding,
                2,
            )?,
            Self::Year {
                padding,
                repr,
                iso_week_based,
                sign_is_mandatory,
            } => {
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
        }

        Ok(())
    }
}
