//! Format implementations for the `UtcOffset` struct.

use crate::{format_description::component, formatting::format_number, UtcOffset};
use core::fmt;

impl component::UtcOffset {
    /// Write the formatted value to the designated output. An `Err` will be returned if the value
    /// cannot be output to the stream.
    pub(super) fn format_into(
        self,
        output: &mut impl fmt::Write,
        offset: UtcOffset,
    ) -> Result<(), fmt::Error> {
        match self {
            Self::Hour {
                padding,
                sign_is_mandatory,
            } => {
                if offset.hours < 0 || offset.minutes < 0 || offset.seconds < 0 {
                    output.write_char('-')?;
                } else if sign_is_mandatory {
                    output.write_char('+')?;
                }
                format_number(output, offset.hours.abs() as u8, padding, 2)?;
            }
            Self::Minute { padding } => {
                format_number(output, offset.minutes.abs() as u8, padding, 2)?;
            }
            Self::Second { padding } => {
                format_number(output, offset.seconds.abs() as u8, padding, 2)?;
            }
        }

        Ok(())
    }
}
