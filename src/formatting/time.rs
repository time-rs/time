//! Format implementations for the `Time` struct.

use crate::{
    format_description::{
        component,
        modifier::{Padding, SubsecondDigits},
    },
    formatting::format_number,
    Time,
};
use core::fmt;

impl component::Time {
    /// Write the formatted value to the designated output. An `Err` will be returned if the value
    /// cannot be output to the stream.
    pub(super) fn format_into(
        self,
        output: &mut impl fmt::Write,
        time: Time,
    ) -> Result<(), fmt::Error> {
        match self {
            Self::Hour {
                padding,
                is_12_hour_clock,
            } => {
                let value = match (time.hour, is_12_hour_clock) {
                    (hour, false) => hour,
                    (0, true) | (12, true) => 12,
                    (hour, true) if hour < 12 => hour,
                    (hour, true) => hour - 12,
                };
                format_number(output, value, padding, 2)?
            }
            Self::Minute { padding } => format_number(output, time.minute, padding, 2)?,
            Self::Period { is_uppercase } => match (time.hour >= 12, is_uppercase) {
                (false, false) => output.write_str("am"),
                (false, true) => output.write_str("AM"),
                (true, false) => output.write_str("pm"),
                (true, true) => output.write_str("PM"),
            }?,
            Self::Second { padding } => format_number(output, time.second, padding, 2)?,
            Self::Subsecond { digits } => {
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
                format_number(output, value, Padding::Zero, width)?
            }
        }

        Ok(())
    }
}
