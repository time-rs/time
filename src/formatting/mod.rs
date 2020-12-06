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

impl FormatDescription<'_> {
    /// Using the format description provided, write the formatted value to the
    /// designated output. An `Err` will be returned if the format description
    /// requires information that the components do not provide or the value
    /// cannot be output to the stream.
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
                return Err(error::Format::InsufficientTypeInformation)
            }
        }

        Ok(())
    }
}
