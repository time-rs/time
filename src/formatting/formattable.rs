//! A trait that can be used to format an item from its components.

use core::ops::Deref;
use std::{fmt, io};

use crate::format_description::well_known::{Rfc2822, Rfc3339};
use crate::format_description::FormatItem;
use crate::formatting::{
    format_component, format_number_pad_zero, write, MONTH_NAMES, WEEKDAY_NAMES,
};
use crate::{error, Date, Time, UtcOffset};

/// A type that can be formatted.
#[cfg_attr(__time_03_docs, doc(notable_trait))]
pub trait Formattable: sealed::Sealed {}
impl Formattable for FormatItem<'_> {}
impl Formattable for [FormatItem<'_>] {}
impl Formattable for Rfc3339 {}
impl Formattable for Rfc2822 {}
impl<T: Deref> Formattable for T where T::Target: Formattable {}

/// A compatibility layer to translate [`io::Write`] into [`fmt::Write`]
struct Compat<'a, W: io::Write> {
    /// The [`io::Write`]r to apply the translation on
    writer: &'a mut W,
    /// The total bytes written into the writer so far
    bytes_written: usize,
    /// The last error from the writer if it returned errors
    error: Option<io::Error>,
}

impl<'a, W> fmt::Write for Compat<'a, W>
where
    W: io::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer
            .write_all(s.as_bytes())
            .map(|_| self.bytes_written += s.len())
            .map_err(|error| {
                self.error = Some(error);
                fmt::Error
            })
    }
}

impl<'a, W> Compat<'a, W>
where
    W: io::Write,
{
    /// Create the compatibility layer from this [`io::Write`]
    fn from_io(writer: &'a mut W) -> Self {
        Self {
            writer,
            bytes_written: 0,
            error: None,
        }
    }

    /// Turn a `Result<(), error::Format::StdFmt>` into `Result<usize, error::Format::StdIo>`
    fn into_io_result(self, result: Result<(), error::Format>) -> Result<usize, error::Format> {
        result.map(|_| self.bytes_written).map_err(|fmt_error| {
            self.error
                .map(error::Format::from)
                .unwrap_or_else(|| fmt_error)
        })
    }
}

/// Seal the trait to prevent downstream users from implementing it.
mod sealed {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Format the item using a format description, the intended output, and the various components.
    #[cfg_attr(__time_03_docs, doc(cfg(feature = "formatting")))]
    pub trait Sealed {
        /// Format the item into the provided output.
        fn format_into(
            &self,
            output: &mut impl fmt::Write,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<(), error::Format>;

        /// Format the item into the provided output, returning the number of bytes written.
        fn format_into_old(
            &self,
            output: &mut impl io::Write,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<usize, error::Format> {
            let mut compat = Compat::from_io(output);
            let result = self.format_into(&mut compat, date, time, offset);
            compat.into_io_result(result)
        }

        /// Format the item directly to a `String`.
        fn format(
            &self,
            date: Option<Date>,
            time: Option<Time>,
            offset: Option<UtcOffset>,
        ) -> Result<String, error::Format> {
            let mut buf = String::new();
            self.format_into(&mut buf, date, time, offset)?;
            Ok(buf)
        }
    }
}

// region: custom formats
impl<'a> sealed::Sealed for FormatItem<'a> {
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), error::Format> {
        match *self {
            Self::Literal(literal) => {
                output.write_str(String::from_utf8_lossy(literal).as_ref())?
            }
            Self::Component(component) => format_component(output, component, date, time, offset)?,
            Self::Compound(items) => items.format_into(output, date, time, offset)?,
            Self::Optional(item) => item.format_into(output, date, time, offset)?,
            Self::First(items) => match items {
                [] => (),
                [item, ..] => item.format_into(output, date, time, offset)?,
            },
        };

        Ok(())
    }
}

impl<'a> sealed::Sealed for [FormatItem<'a>] {
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), error::Format> {
        for item in self.iter() {
            item.format_into(output, date, time, offset)?;
        }
        Ok(())
    }
}

impl<T: Deref> sealed::Sealed for T
where
    T::Target: sealed::Sealed,
{
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), error::Format> {
        self.deref().format_into(output, date, time, offset)
    }
}
// endregion custom formats

// region: well-known formats
impl sealed::Sealed for Rfc2822 {
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), error::Format> {
        let date = date.ok_or(error::Format::InsufficientTypeInformation)?;
        let time = time.ok_or(error::Format::InsufficientTypeInformation)?;
        let offset = offset.ok_or(error::Format::InsufficientTypeInformation)?;

        let (year, month, day) = date.to_calendar_date();

        if year < 1900 {
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset.seconds_past_minute() != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        write(
            output,
            &WEEKDAY_NAMES[date.weekday().number_days_from_monday() as usize][..3],
        )?;
        write(output, ", ")?;
        format_number_pad_zero::<_, _, 2>(output, day)?;
        write(output, " ")?;
        write(output, &MONTH_NAMES[month as usize - 1][..3])?;
        write(output, " ")?;
        format_number_pad_zero::<_, _, 4>(output, year as u32)?;
        write(output, " ")?;
        format_number_pad_zero::<_, _, 2>(output, time.hour())?;
        write(output, ":")?;
        format_number_pad_zero::<_, _, 2>(output, time.minute())?;
        write(output, ":")?;
        format_number_pad_zero::<_, _, 2>(output, time.second())?;
        write(output, " ")?;
        write(output, if offset.is_negative() { "-" } else { "+" })?;
        format_number_pad_zero::<_, _, 2>(output, offset.whole_hours().unsigned_abs())?;
        format_number_pad_zero::<_, _, 2>(output, offset.minutes_past_hour().unsigned_abs())?;

        Ok(())
    }
}

impl sealed::Sealed for Rfc3339 {
    fn format_into(
        &self,
        output: &mut impl fmt::Write,
        date: Option<Date>,
        time: Option<Time>,
        offset: Option<UtcOffset>,
    ) -> Result<(), error::Format> {
        let date = date.ok_or(error::Format::InsufficientTypeInformation)?;
        let time = time.ok_or(error::Format::InsufficientTypeInformation)?;
        let offset = offset.ok_or(error::Format::InsufficientTypeInformation)?;

        let year = date.year();

        if !(0..10_000).contains(&year) {
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset.seconds_past_minute() != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        format_number_pad_zero::<_, _, 4>(output, year as u32)?;
        write(output, "-")?;
        format_number_pad_zero::<_, _, 2>(output, date.month() as u8)?;
        write(output, "-")?;
        format_number_pad_zero::<_, _, 2>(output, date.day())?;
        write(output, "T")?;
        format_number_pad_zero::<_, _, 2>(output, time.hour())?;
        write(output, ":")?;
        format_number_pad_zero::<_, _, 2>(output, time.minute())?;
        write(output, ":")?;
        format_number_pad_zero::<_, _, 2>(output, time.second())?;

        #[allow(clippy::if_not_else)]
        if time.nanosecond() != 0 {
            let nanos = time.nanosecond();
            write(output, ".")?;
            if nanos % 10 != 0 {
                format_number_pad_zero::<_, _, 9>(output, nanos)
            } else if (nanos / 10) % 10 != 0 {
                format_number_pad_zero::<_, _, 8>(output, nanos / 10)
            } else if (nanos / 100) % 10 != 0 {
                format_number_pad_zero::<_, _, 7>(output, nanos / 100)
            } else if (nanos / 1_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 6>(output, nanos / 1_000)
            } else if (nanos / 10_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 5>(output, nanos / 10_000)
            } else if (nanos / 100_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 4>(output, nanos / 100_000)
            } else if (nanos / 1_000_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 3>(output, nanos / 1_000_000)
            } else if (nanos / 10_000_000) % 10 != 0 {
                format_number_pad_zero::<_, _, 2>(output, nanos / 10_000_000)
            } else {
                format_number_pad_zero::<_, _, 1>(output, nanos / 100_000_000)
            }?;
        }

        if offset == UtcOffset::UTC {
            write(output, "Z")?;
            return Ok(());
        }

        write(output, if offset.is_negative() { "-" } else { "+" })?;
        format_number_pad_zero::<_, _, 2>(output, offset.whole_hours().unsigned_abs())?;
        write(output, ":")?;
        format_number_pad_zero::<_, _, 2>(output, offset.minutes_past_hour().unsigned_abs())?;

        Ok(())
    }
}
// endregion well-known formats
