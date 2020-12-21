//! Format implementations for the `UtcOffset` struct.

use crate::{format_description::component, formatting::format_value, UtcOffset};
use core::fmt;

impl component::UtcOffset {
    /// Write the formatted value to the designated output. An `Err` will be returned if the value
    /// cannot be output to the stream.
    pub(super) fn format_into(
        self,
        output: &mut dyn fmt::Write,
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
                format_value(output, offset.hours.abs(), padding, 2)?;
            }
            Self::Minute { padding } => {
                format_value(output, offset.minutes.abs(), padding, 2)?;
            }
            Self::Second { padding } => {
                format_value(output, offset.seconds.abs(), padding, 2)?;
            }
        }

        Ok(())
    }
}
