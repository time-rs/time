//! The `DeferredFormat` struct, acting as an intermediary between a request to
//! format and the final output.

use crate::{
    format::{date, format_specifier, parse_fmt_string, time, Format, FormatItem, Padding},
    internal_prelude::*,
};
use core::fmt::{self, Display, Formatter};

/// A struct containing all the necessary information to display the inner type.
#[allow(single_use_lifetimes)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct DeferredFormat {
    /// The `Date` to use for formatting.
    date: Option<Date>,
    /// The `Time` to use for formatting.
    time: Option<Time>,
    /// The `UtcOffset` to use for formatting.
    offset: Option<UtcOffset>,
    /// The list of items used to display the item.
    format: Format,
}

impl DeferredFormat {
    /// Create a new `DeferredFormat` with the provided formatting string.
    #[inline]
    pub(crate) fn new(format: impl Into<Format>) -> Self {
        Self {
            date: None,
            time: None,
            offset: None,
            format: format.into(),
        }
    }

    /// Provide the `Date` component.
    #[inline]
    pub(crate) fn with_date(&mut self, date: Date) -> &mut Self {
        self.date = Some(date);
        self
    }

    /// Provide the `Time` component.
    #[inline]
    pub(crate) fn with_time(&mut self, time: Time) -> &mut Self {
        self.time = Some(time);
        self
    }

    /// Provide the `UtCOffset` component.
    #[inline]
    pub(crate) fn with_offset(&mut self, offset: UtcOffset) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}

impl Display for DeferredFormat {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.format {
            Format::Custom(s) => {
                for item in parse_fmt_string(s) {
                    match item {
                        FormatItem::Literal(value) => f.write_str(value)?,
                        FormatItem::Specifier(specifier) => {
                            format_specifier(f, self.date, self.time, self.offset, specifier)?
                        }
                    }
                }

                Ok(())
            }
            Format::Rfc3339 => rfc3339(self, f),
            #[cfg(not(supports_non_exhaustive))]
            Format::__NonExhaustive => unreachable!(),
        }
    }
}

/// Format `df` according to the RFC3339 specification.
#[inline]
fn rfc3339(df: &DeferredFormat, f: &mut Formatter<'_>) -> fmt::Result {
    // If we're using RFC3339, all three components must be present.
    // This will be enforced with typestate when Rust gains sufficient
    // capabilities (namely proper sealed traits and/or function overloading).
    #[allow(clippy::option_unwrap_used)]
    let date = df.date.unwrap();
    #[allow(clippy::option_unwrap_used)]
    let time = df.time.unwrap();
    #[allow(clippy::option_unwrap_used)]
    let offset = df.offset.unwrap();

    date::fmt_Y(f, date, Padding::Zero)?;
    f.write_str("-")?;
    date::fmt_m(f, date, Padding::Zero)?;
    f.write_str("-")?;
    date::fmt_d(f, date, Padding::Zero)?;
    f.write_str("T")?;
    time::fmt_H(f, time, Padding::Zero)?;
    f.write_str(":")?;
    time::fmt_M(f, time, Padding::Zero)?;
    f.write_str(":")?;
    time::fmt_S(f, time, Padding::Zero)?;
    write!(
        f,
        "{:+03}:{:02}",
        offset.as_hours(),
        offset.as_minutes().rem_euclid(60)
    )?;

    Ok(())
}
