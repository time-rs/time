//! Part of a format description.

use crate::format_description::modifier;
#[cfg(feature = "alloc")]
use crate::format_description::{modifier::Modifiers, InvalidFormatDescription};
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;

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

/// A component with no modifiers present.
#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
pub(crate) enum NakedComponent {
    /// Day of the month.
    Day,
    /// Hour of the day.
    Hour,
    /// Minute within the hour.
    Minute,
    /// Month of the year.
    Month,
    /// Hour of the UTC offset.
    OffsetHour,
    /// Minute within the hour of the UTC offset.
    OffsetMinute,
    /// Second within the minute of the UTC offset.
    OffsetSecond,
    /// Ordinal day of the year.
    Ordinal,
    /// AM/PM part of the time.
    Period,
    /// Second within the minute.
    Second,
    /// Subsecond within the second.
    Subsecond,
    /// Day of the week.
    Weekday,
    /// Week within the year.
    WeekNumber,
    /// Year of the date.
    Year,
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl NakedComponent {
    // We can't use `FromStr` here because we need the component index as well.
    /// Parse a component (without its modifiers) from the provided name.
    pub(crate) fn parse(
        component_name: &str,
        component_index: usize,
    ) -> Result<Self, InvalidFormatDescription> {
        match component_name {
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "minute" => Ok(Self::Minute),
            "month" => Ok(Self::Month),
            "offset_hour" => Ok(Self::OffsetHour),
            "offset_minute" => Ok(Self::OffsetMinute),
            "offset_second" => Ok(Self::OffsetSecond),
            "ordinal" => Ok(Self::Ordinal),
            "period" => Ok(Self::Period),
            "second" => Ok(Self::Second),
            "subsecond" => Ok(Self::Subsecond),
            "weekday" => Ok(Self::Weekday),
            "week_number" => Ok(Self::WeekNumber),
            "year" => Ok(Self::Year),
            name => Err(InvalidFormatDescription::InvalidComponentName {
                name: name.to_owned(),
                index: component_index,
            }),
        }
    }

    /// Attach the necessary modifiers to the component.
    pub(crate) fn attach_modifiers(self, modifiers: &Modifiers) -> Component {
        match self {
            Self::Day => Component::Day {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Hour => Component::Hour {
                padding: modifiers.padding.unwrap_or_default(),
                is_12_hour_clock: modifiers.hour_is_12_hour_clock.unwrap_or_default(),
            },
            Self::Minute => Component::Minute {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Month => Component::Month {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.month_repr.unwrap_or_default(),
            },
            Self::OffsetHour => Component::OffsetHour {
                sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::OffsetMinute => Component::OffsetMinute {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::OffsetSecond => Component::OffsetSecond {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Ordinal => Component::Ordinal {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Period => Component::Period {
                is_uppercase: modifiers.period_is_uppercase.unwrap_or(true),
            },
            Self::Second => Component::Second {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Subsecond => Component::Subsecond {
                digits: modifiers.subsecond_digits.unwrap_or_default(),
            },
            Self::Weekday => Component::Weekday {
                repr: modifiers.weekday_repr.unwrap_or_default(),
                one_indexed: modifiers.weekday_is_one_indexed.unwrap_or(true),
            },
            Self::WeekNumber => Component::WeekNumber {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.week_number_repr.unwrap_or_default(),
            },
            Self::Year => Component::Year {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.year_repr.unwrap_or_default(),
                iso_week_based: modifiers.year_is_iso_week_based.unwrap_or_default(),
                sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
            },
        }
    }
}
