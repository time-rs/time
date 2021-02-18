//! A trait that can be used to format an item from its components.

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt;

use crate::format_description::modifier::Padding;
use crate::format_description::well_known::Rfc3339;
use crate::format_description::FormatItem;
use crate::formatting::{format_component, format_number};
use crate::{error, Date, Time, UtcOffset};

/// Seal the trait to prevent downstream users from implementing it, while still allowing it to
/// exist in generic bounds.
pub(crate) mod sealed {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Format the item using a format description, the intended output, and the various components.
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
    pub trait Formattable {
        /// An error that may be returned when formatting.
        type Error;

        /// Format the item into the provided output.
        fn format_into(
            &self,
            output: &mut impl fmt::Write,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<(), Self::Error>;

        /// Format the item directly to a `String`.
        #[cfg(feature = "alloc")]
        #[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
        fn format(
            &self,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<String, Self::Error> {
            let mut s = String::new();
            self.format_into(&mut s, date, time, offset)?;
            Ok(s)
        }
    }
}

impl<'a> sealed::Formattable for FormatItem<'a> {
    type Error = error::Format;

    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), Self::Error> {
        match *self {
            Self::Literal(literal) => output.write_str(literal)?,
            Self::Component(component) => format_component(output, component, date, time, offset)?,
            Self::Compound(items) => items.format_into(output, date, time, offset)?,
        }
        Ok(())
    }
}

impl<'a> sealed::Formattable for &[FormatItem<'a>] {
    type Error = error::Format;

    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), Self::Error> {
        for item in self.iter() {
            item.format_into(output, date, time, offset)?;
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl<'a> sealed::Formattable for Vec<FormatItem<'a>> {
    type Error = <&'a [FormatItem<'a>] as sealed::Formattable>::Error;

    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), Self::Error> {
        self.as_slice().format_into(output, date, time, offset)
    }
}

impl sealed::Formattable for Rfc3339 {
    type Error = error::Format;

    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), Self::Error> {
        let date = date.ok_or(error::Format::InsufficientTypeInformation)?;
        let time = time.ok_or(error::Format::InsufficientTypeInformation)?;
        let offset = offset.ok_or(error::Format::InsufficientTypeInformation)?;

        let year = date.year();

        if !(0..10_000).contains(&year) {
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset.seconds != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        format_number(output, year as u32, Padding::Zero, 4)?;
        output.write_char('-')?;
        format_number(output, date.month(), Padding::Zero, 2)?;
        output.write_char('-')?;
        format_number(output, date.day(), Padding::Zero, 2)?;
        output.write_char('T')?;
        format_number(output, time.hour, Padding::Zero, 2)?;
        output.write_char(':')?;
        format_number(output, time.minute, Padding::Zero, 2)?;
        output.write_char(':')?;
        format_number(output, time.second, Padding::Zero, 2)?;

        if time.nanosecond != 0 {
            output.write_char('.')?;

            let (value, width) = match time.nanosecond {
                nanos if nanos % 10 != 0 => (nanos, 9),
                nanos if (nanos / 10) % 10 != 0 => (nanos / 10, 8),
                nanos if (nanos / 100) % 10 != 0 => (nanos / 100, 7),
                nanos if (nanos / 1_000) % 10 != 0 => (nanos / 1_000, 6),
                nanos if (nanos / 10_000) % 10 != 0 => (nanos / 10_000, 5),
                nanos if (nanos / 100_000) % 10 != 0 => (nanos / 100_000, 4),
                nanos if (nanos / 1_000_000) % 10 != 0 => (nanos / 1_000_000, 3),
                nanos if (nanos / 10_000_000) % 10 != 0 => (nanos / 10_000_000, 2),
                nanos => (nanos / 100_000_000, 1),
            };
            format_number(output, value, Padding::Zero, width)?;
        }

        if offset == UtcOffset::UTC {
            output.write_char('Z')?;
            return Ok(());
        }

        output.write_char(if offset.hours < 0 || offset.minutes < 0 {
            '-'
        } else {
            '+'
        })?;
        format_number(output, offset.hours.abs() as u8, Padding::Zero, 2)?;
        output.write_char(':')?;
        format_number(output, offset.minutes.abs() as u8, Padding::Zero, 2)?;

        Ok(())
    }
}
