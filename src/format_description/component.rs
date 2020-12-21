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
    /// A component that requires a [`Date`](crate::Date) to be present.
    Date(Date),
    /// A component that requires a [`Time`](crate::Time) to be present.
    Time(Time),
    /// A component that requires a [`UtcOffset`](crate::UtcOffset) to be present.
    UtcOffset(UtcOffset),
}

/// A component of a larger format description.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Date {
    /// Day of the month.
    Day {
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
    /// Ordinal day of the year.
    Ordinal {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Day of the week.
    Weekday {
        /// What form of representation should be used?
        repr: modifier::WeekdayRepr,
        /// When using a numerical representation, should it be zero or one-indexed?
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
        /// Whether the `+` sign is present when a positive year contains fewer than five digits.
        sign_is_mandatory: bool,
    },
}

/// A component of a larger format description.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Time {
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
}

/// A component of a larger format description.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtcOffset {
    /// Hour of the UTC offset.
    Hour {
        /// Whether the `+` sign is present on positive values.
        sign_is_mandatory: bool,
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Minute within the hour of the UTC offset.
    Minute {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
    /// Second within the minute of the UTC offset.
    Second {
        /// The padding to obtain the minimum width.
        padding: modifier::Padding,
    },
}

/// A component of a date with no modifiers present.
#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
pub(crate) enum NakedDateComponent {
    /// Day of the month.
    Day,
    /// Month of the year.
    Month,
    /// Ordinal day of the year.
    Ordinal,
    /// Day of the week.
    Weekday,
    /// Week within the year.
    WeekNumber,
    /// Year of the date.
    Year,
}

/// A component of a time with no modifiers present.
#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
pub(crate) enum NakedTimeComponent {
    /// Hour of the day.
    Hour,
    /// Minute within the hour.
    Minute,
    /// AM/PM part of the time.
    Period,
    /// Second within the minute.
    Second,
    /// Subsecond within the second.
    Subsecond,
}

/// A component of an offset with no modifiers present.
#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
#[allow(clippy::enum_variant_names)]
pub(crate) enum NakedUtcOffsetComponent {
    /// Hour of the UTC offset.
    OffsetHour,
    /// Minute within the hour of the UTC offset.
    OffsetMinute,
    /// Second within the minute of the UTC offset.
    OffsetSecond,
}

/// A component with no modifiers present.
#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
pub(crate) enum NakedComponent {
    /// A component that requires a [`Date`](crate::Date) to be present.
    Date(NakedDateComponent),
    /// A component that requires a [`Time`](crate::Time) to be present.
    Time(NakedTimeComponent),
    /// A component that requires a [`UtcOffset`](crate::UtcOffset) to be present.
    UtcOffset(NakedUtcOffsetComponent),
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl NakedDateComponent {
    /// Parse a component (without its modifiers) from the provided name.
    fn parse(component_name: &str) -> Option<Self> {
        match component_name {
            "day" => Some(Self::Day),
            "month" => Some(Self::Month),
            "ordinal" => Some(Self::Ordinal),
            "weekday" => Some(Self::Weekday),
            "week_number" => Some(Self::WeekNumber),
            "year" => Some(Self::Year),
            _ => None,
        }
    }

    /// Attach the necessary modifiers to the component.
    fn attach_modifiers(self, modifiers: &Modifiers) -> Date {
        match self {
            Self::Day => Date::Day {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Month => Date::Month {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.month_repr.unwrap_or_default(),
            },
            Self::Ordinal => Date::Ordinal {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Weekday => Date::Weekday {
                repr: modifiers.weekday_repr.unwrap_or_default(),
                one_indexed: modifiers.weekday_is_one_indexed.unwrap_or(true),
            },
            Self::WeekNumber => Date::WeekNumber {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.week_number_repr.unwrap_or_default(),
            },
            Self::Year => Date::Year {
                padding: modifiers.padding.unwrap_or_default(),
                repr: modifiers.year_repr.unwrap_or_default(),
                iso_week_based: modifiers.year_is_iso_week_based.unwrap_or_default(),
                sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
            },
        }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl NakedTimeComponent {
    /// Parse a component (without its modifiers) from the provided name.
    fn parse(component_name: &str) -> Option<Self> {
        match component_name {
            "hour" => Some(Self::Hour),
            "minute" => Some(Self::Minute),
            "period" => Some(Self::Period),
            "second" => Some(Self::Second),
            "subsecond" => Some(Self::Subsecond),
            _ => None,
        }
    }

    /// Attach the necessary modifiers to the component.
    fn attach_modifiers(self, modifiers: &Modifiers) -> Time {
        match self {
            Self::Hour => Time::Hour {
                padding: modifiers.padding.unwrap_or_default(),
                is_12_hour_clock: modifiers.hour_is_12_hour_clock.unwrap_or_default(),
            },
            Self::Minute => Time::Minute {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Period => Time::Period {
                is_uppercase: modifiers.period_is_uppercase.unwrap_or(true),
            },
            Self::Second => Time::Second {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::Subsecond => Time::Subsecond {
                digits: modifiers.subsecond_digits.unwrap_or_default(),
            },
        }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(__time_03_docs, doc(cfg(feature = "alloc")))]
impl NakedUtcOffsetComponent {
    /// Parse a component (without its modifiers) from the provided name.
    fn parse(component_name: &str) -> Option<Self> {
        match component_name {
            "offset_hour" => Some(Self::OffsetHour),
            "offset_minute" => Some(Self::OffsetMinute),
            "offset_second" => Some(Self::OffsetSecond),
            _ => None,
        }
    }

    /// Attach the necessary modifiers to the component.
    fn attach_modifiers(self, modifiers: &Modifiers) -> UtcOffset {
        match self {
            Self::OffsetHour => UtcOffset::Hour {
                sign_is_mandatory: modifiers.sign_is_mandatory.unwrap_or_default(),
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::OffsetMinute => UtcOffset::Minute {
                padding: modifiers.padding.unwrap_or_default(),
            },
            Self::OffsetSecond => UtcOffset::Second {
                padding: modifiers.padding.unwrap_or_default(),
            },
        }
    }
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
        NakedDateComponent::parse(component_name)
            .map(Self::Date)
            .or_else(|| NakedTimeComponent::parse(component_name).map(Self::Time))
            .or_else(|| NakedUtcOffsetComponent::parse(component_name).map(Self::UtcOffset))
            .ok_or_else(|| InvalidFormatDescription::InvalidComponentName {
                name: component_name.to_owned(),
                index: component_index,
            })
    }

    /// Attach the necessary modifiers to the component.
    pub(crate) fn attach_modifiers(self, modifiers: &Modifiers) -> Component {
        match self {
            Self::Date(naked_component) => {
                Component::Date(naked_component.attach_modifiers(modifiers))
            }
            Self::Time(naked_component) => {
                Component::Time(naked_component.attach_modifiers(modifiers))
            }
            Self::UtcOffset(naked_component) => {
                Component::UtcOffset(naked_component.attach_modifiers(modifiers))
            }
        }
    }
}
