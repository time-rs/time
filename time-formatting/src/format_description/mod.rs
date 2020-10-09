//! Description of how types should be formatted and parsed.

pub mod error;
pub mod modifier;
#[cfg(feature = "alloc")]
pub(crate) mod parse;

/// Helper methods.
#[cfg(feature = "alloc")]
mod helper {
    /// Consume all leading whitespace, advancing `index` as appropriate.
    #[must_use = "This does not modify the original string."]
    pub(crate) fn consume_whitespace<'a>(s: &'a str, index: &mut usize) -> &'a str {
        *index += s.len();
        let s = s.trim_start();
        *index -= s.len();
        s
    }
}

/// A component of a larger format description.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    /// Day of the month.
    Day {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Hour of the day.
    Hour {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
        /// Is the hour displayed using a 12 or 24-hour clock?
        is_12_hour_clock: bool,
    },
    /// Minute within the hour.
    Minute {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Month of the year.
    Month {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
        /// What form of representation should be used?
        repr: modifier::MonthRepr,
    },
    /// Hour of the UTC offset.
    OffsetHour {
        /// Whether the `+` sign is present on positive values.
        sign_is_mandatory: bool,
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Minute within the hour of the UTC offset.
    OffsetMinute {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Second within the minute of the UTC offset.
    OffsetSecond {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Ordinal day of the year.
    Ordinal {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// AM/PM part of the time.
    Period {
        /// Is the period uppercase or lowercase?
        is_uppercase: bool,
    },
    /// Second within the minute.
    Second {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Subsecond within the second.
    Subsecond {
        /// How many digits are present in the component?
        digits: modifier::SubsecondDigits,
    },
    /// Day of the week.
    Weekday {
        /// What form of representation should be used?
        repr: modifier::WeekdayRepr,
        /// When using a numerical representation, should it be zero or
        /// one-indexed?
        ///
        /// This setting has no effect on textual representations.
        one_indexed: bool,
    },
    /// Week within the year.
    WeekNumber {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
        /// What kind of representation should be used?
        repr: modifier::WeekNumberRepr,
    },
    /// Year of the date.
    Year {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
        /// What kind of representation should be used?
        repr: modifier::YearRepr,
        /// Whether the value based on the ISO week number.
        iso_week_based: bool,
        /// Whether the `+` sign is present when a positive year contains fewer
        /// than five digits.
        sign_is_mandatory: bool,
    },
}

/// A complete description of how to format and parse a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Description<'a> {
    /// A string that is formatted as-is.
    Literal(&'a str),
    /// A minimal representation of a single non-literal item.
    Component(Component),
    /// A series of literals or components that collectively form a partial or
    /// complete description.
    ///
    /// Note that this is a reference to a slice, such that either a [`Vec`] or
    /// statically known list can be provided.
    Compound(&'a [Self]),
}
