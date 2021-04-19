//! A trait that can be used to format an item from its components.

use std::io;

#[allow(unused_imports)]
use standback::prelude::*;

use crate::format_description::modifier::Padding;
use crate::format_description::well_known::Rfc3339;
use crate::format_description::FormatItem;
use crate::formatting::{format_component, format_number};
use crate::{error, Date, Time, UtcOffset};

/// A type that can be formatted.
pub trait Formattable: sealed::Sealed {}
impl Formattable for FormatItem<'_> {}
impl Formattable for &[FormatItem<'_>] {}
#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl Formattable for Vec<FormatItem<'_>> {}
impl Formattable for Rfc3339 {}

/// Seal the trait to prevent downstream users from implementing it.
mod sealed {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Format the item using a format description, the intended output, and the various components.
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
    pub trait Sealed {
        /// Format the item into the provided output, returning the number of bytes written.
        fn format_into(
            &self,
            output: &mut impl io::Write,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<usize, error::Format>;

        /// Format the item directly to a `String`.
        fn format(
            &self,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<String, error::Format> {
            let mut buf = Vec::new();
            self.format_into(&mut buf, date, time, offset)?;
            io::Write::flush(&mut buf)?;
            Ok(String::from_utf8_lossy(&buf).into_owned())
        }
    }
}

// region: custom formats
impl<'a> sealed::Sealed for FormatItem<'a> {
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        Ok(match *self {
            Self::Literal(literal) => output.write(literal)?,
            Self::Component(component) => format_component(output, component, date, time, offset)?,
            Self::Compound(items) => items.format_into(output, date, time, offset)?,
        })
    }
}

impl<'a> sealed::Sealed for &[FormatItem<'a>] {
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        let mut bytes = 0;
        for item in self.iter() {
            bytes += item.format_into(output, date, time, offset)?;
        }
        Ok(bytes)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl<'a> sealed::Sealed for Vec<FormatItem<'a>> {
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        self.as_slice().format_into(output, date, time, offset)
    }
}
// endregion custom formats

// region: well-known formats
impl sealed::Sealed for Rfc3339 {
    fn format_into(
        &self,
        output: &mut impl io::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<usize, error::Format> {
        let date = date.ok_or(error::Format::InsufficientTypeInformation)?;
        let time = time.ok_or(error::Format::InsufficientTypeInformation)?;
        let offset = offset.ok_or(error::Format::InsufficientTypeInformation)?;

        let mut bytes = 0;

        let year = date.year();

        if !(0..10_000).contains(&year) {
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset.seconds_past_minute() != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        bytes += format_number(output, year as u32, Padding::Zero, 4)?;
        bytes += output.write(&[b'-'])?;
        bytes += format_number(output, date.month(), Padding::Zero, 2)?;
        bytes += output.write(&[b'-'])?;
        bytes += format_number(output, date.day(), Padding::Zero, 2)?;
        bytes += output.write(&[b'T'])?;
        bytes += format_number(output, time.hour(), Padding::Zero, 2)?;
        bytes += output.write(&[b':'])?;
        bytes += format_number(output, time.minute(), Padding::Zero, 2)?;
        bytes += output.write(&[b':'])?;
        bytes += format_number(output, time.second(), Padding::Zero, 2)?;

        if time.nanosecond() != 0 {
            bytes += output.write(&[b'.'])?;

            let (value, width) = match time.nanosecond() {
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
            bytes += format_number(output, value, Padding::Zero, width)?;
        }

        if offset == UtcOffset::UTC {
            bytes += output.write(&[b'Z'])?;
            return Ok(bytes);
        }

        bytes += output.write(if offset.is_negative() {
            &[b'-']
        } else {
            &[b'+']
        })?;
        bytes += format_number(
            output,
            offset.whole_hours().unsigned_abs(),
            Padding::Zero,
            2,
        )?;
        bytes += output.write(&[b':'])?;
        bytes += format_number(
            output,
            offset.minutes_past_hour().unsigned_abs(),
            Padding::Zero,
            2,
        )?;

        Ok(bytes)
    }
}
// endregion well-known formats
