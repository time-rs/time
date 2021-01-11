//! Formatting for the time crate.

mod date;
mod offset;
mod time;

use crate::{
    error,
    format_description::{modifier::Padding, Component, FormatDescription},
    Date, Time, UtcOffset,
};
use core::fmt;

/// A trait that indicates the formatted width of the value can be determined.
///
/// Note that this should not be implemented for any signed integers. This forces the caller to
/// write the sign if desired.
trait DigitCount {
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
fn format_number(
    output: &mut dyn fmt::Write,
    value: impl itoa::Integer + DigitCount + Copy,
    padding: Padding,
    width: u8,
) -> Result<(), fmt::Error> {
    match padding {
        Padding::Space => {
            for _ in 0..(width.saturating_sub(value.num_digits())) {
                output.write_char(' ')?;
            }
            itoa::fmt(output, value)
        }
        Padding::Zero => {
            for _ in 0..(width.saturating_sub(value.num_digits())) {
                output.write_char('0')?;
            }
            itoa::fmt(output, value)
        }
        Padding::None => itoa::fmt(output, value),
    }
}

impl FormatDescription<'_> {
    /// Using the format description provided, write the formatted value to the designated output.
    /// An `Err` will be returned if the format description requires information that the components
    /// do not provide or the value cannot be output to the stream.
    pub(crate) fn format_into(
        &self,
        output: &mut dyn fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), error::Format> {
        match (self, date, time, offset) {
            (&FormatDescription::Literal(literal), _, _, _) => output.write_str(literal)?,
            (&FormatDescription::Component(Component::Date(component)), Some(date), _, _) => {
                component.format_into(output, date)?
            }
            (&FormatDescription::Component(Component::Time(component)), _, Some(time), _) => {
                component.format_into(output, time)?
            }
            (
                &FormatDescription::Component(Component::UtcOffset(component)),
                _,
                _,
                Some(offset),
            ) => component.format_into(output, offset)?,
            (&FormatDescription::BorrowedCompound(descriptions), _, _, _) => {
                for description in descriptions {
                    description.format_into(output, date, time, offset)?;
                }
            }
            #[cfg(feature = "alloc")]
            (&FormatDescription::OwnedCompound(ref descriptions), _, _, _) => {
                for description in descriptions {
                    description.format_into(output, date, time, offset)?;
                }
            }
            (&FormatDescription::Component(Component::Date(_)), None, _, _)
            | (&FormatDescription::Component(Component::Time(_)), _, None, _)
            | (&FormatDescription::Component(Component::UtcOffset(_)), _, _, None) => {
                return Err(error::Format::InsufficientTypeInformation);
            }
        }

        Ok(())
    }
}
