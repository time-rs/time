//! The `DeferredFormat` struct, acting as an intermediary between a request to
//! format and the final output.

use crate::{
    format::{format_specifier, parse_fmt_string, well_known, Format, FormatItem},
    internal_prelude::*,
};
use core::fmt::{self, Display, Formatter};

/// A struct containing all the necessary information to display the inner type.
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

    /// Provide the `UtcOffset` component.
    #[inline]
    pub(crate) fn with_offset(&mut self, offset: UtcOffset) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    /// Obtain the `Date` component.
    #[inline(always)]
    pub(crate) const fn date(&self) -> Option<Date> {
        self.date
    }

    /// Obtain the `Time` component.
    #[inline(always)]
    pub(crate) const fn time(&self) -> Option<Time> {
        self.time
    }

    /// Obtain the `UtcOffset` component.
    #[inline(always)]
    pub(crate) const fn offset(&self) -> Option<UtcOffset> {
        self.offset
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
            Format::Rfc3339 => well_known::rfc3339::fmt(self, f),
            #[cfg(not(supports_non_exhaustive))]
            Format::__NonExhaustive => unreachable!(),
        }
    }
}
