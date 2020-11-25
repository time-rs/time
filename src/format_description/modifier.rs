//! Various modifiers for components.

#[cfg(feature = "alloc")]
use crate::format_description::{error::InvalidFormatDescription, helper};
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;

/// Type of padding to ensure a minimum width.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Padding {
    /// A space character (` `) should be used as padding.
    Space,
    /// A zero character (`0`) should be used as padding.
    Zero,
    /// There is no padding. This can result in a width below the otherwise
    /// minimum number of characters.
    None,
}

/// The representation of a month.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthRepr {
    /// The number of the month (January is 1, December is 12).
    Numerical,
    /// The long form of the month name (e.g. "January").
    Long,
    /// The short form of the month name (e.g. "Jan").
    Short,
}

/// The number of digits present in a subsecond representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubsecondDigits {
    /// Exactly one digit.
    One,
    /// Exactly two digits.
    Two,
    /// Exactly three digits.
    Three,
    /// Exactly four digits.
    Four,
    /// Exactly five digits.
    Five,
    /// Exactly six digits.
    Six,
    /// Exactly seven digits.
    Seven,
    /// Exactly eight digits.
    Eight,
    /// Exactly nine digits.
    Nine,
    /// Any number of digits that is at least one. When formatting, the minimum
    /// digits necessary will be used.
    OneOrMore,
}

/// The representation used for the day of the week.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekdayRepr {
    /// The short form of the weekday (e.g. "Mon").
    Short,
    /// The long form of the weekday (e.g. "Monday").
    Long,
    /// A numerical representation using Sunday as the first day of the week.
    ///
    /// Sunday is either 0 or 1, depending on the other modifier's value.
    Sunday,
    /// A numerical representation using Monday as the first day of the week.
    ///
    /// Monday is either 0 or 1, depending on the other modifier's value.
    Monday,
}

/// The representation used for the week number.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekNumberRepr {
    /// Week 1 is the week that contains January 4.
    Iso,
    /// Week 1 begins on the first Sunday of the calendar year.
    Sunday,
    /// Week 1 begins on the first Monday of the calendar year.
    Monday,
}

/// The representation used for a year value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearRepr {
    /// The full value of the year.
    Full,
    /// Only the century portion of the year.
    Century,
    /// Only the last two digits of the year.
    LastTwo,
}

macro_rules! impl_default {
    ($($type:ty => $default:expr;)*) => {$(
        impl Default for $type {
            fn default() -> Self {
                $default
            }
        }
    )*};
}

impl_default! {
    Padding => Self::Zero;
    MonthRepr => Self::Long;
    SubsecondDigits => Self::OneOrMore;
    WeekdayRepr => Self::Long;
    WeekNumberRepr => Self::Iso;
    YearRepr => Self::Full;
}

/// The modifiers parsed for any given component. `None` indicates the modifier
/// was not present.
#[allow(clippy::missing_docs_in_private_items)] // fields
#[derive(Debug, Default)]
pub(crate) struct Modifiers {
    pub(crate) padding: Option<Padding>,
    pub(crate) hour_is_12_hour_clock: Option<bool>,
    pub(crate) period_is_uppercase: Option<bool>,
    pub(crate) month_repr: Option<MonthRepr>,
    pub(crate) subsecond_digits: Option<SubsecondDigits>,
    pub(crate) weekday_repr: Option<WeekdayRepr>,
    pub(crate) weekday_is_one_indexed: Option<bool>,
    pub(crate) week_number_repr: Option<WeekNumberRepr>,
    pub(crate) year_repr: Option<YearRepr>,
    pub(crate) year_is_iso_week_based: Option<bool>,
    pub(crate) sign_is_mandatory: Option<bool>,
}

impl Modifiers {
    /// Parse the modifiers of a given component.
    #[cfg(feature = "alloc")]
    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse(
        component_name: &str,
        mut s: &str,
        index: &mut usize,
    ) -> Result<Self, InvalidFormatDescription> {
        let mut modifiers = Self::default();

        while !s.is_empty() {
            // Trim any whitespace between modifiers.
            s = helper::consume_whitespace(s, index);

            let modifier;
            if let Some(whitespace_loc) = s.find(char::is_whitespace) {
                *index += whitespace_loc;
                modifier = &s[..whitespace_loc];
                s = &s[whitespace_loc..];
            } else {
                modifier = s;
                s = "";
            }

            if modifier.is_empty() {
                break;
            }

            match (component_name, modifier) {
                ("day", "padding:space")
                | ("hour", "padding:space")
                | ("minute", "padding:space")
                | ("month", "padding:space")
                | ("offset_hour", "padding:space")
                | ("offset_minute", "padding:space")
                | ("offset_second", "padding:space")
                | ("ordinal", "padding:space")
                | ("second", "padding:space")
                | ("week_number", "padding:space")
                | ("year", "padding:space") => modifiers.padding = Some(Padding::Space),
                ("day", "padding:zero")
                | ("hour", "padding:zero")
                | ("minute", "padding:zero")
                | ("month", "padding:zero")
                | ("offset_hour", "padding:zero")
                | ("offset_minute", "padding:zero")
                | ("offset_second", "padding:zero")
                | ("ordinal", "padding:zero")
                | ("second", "padding:zero")
                | ("week_number", "padding:zero")
                | ("year", "padding:zero") => modifiers.padding = Some(Padding::Zero),
                ("day", "padding:none")
                | ("hour", "padding:none")
                | ("minute", "padding:none")
                | ("month", "padding:none")
                | ("offset_hour", "padding:none")
                | ("offset_minute", "padding:none")
                | ("offset_second", "padding:none")
                | ("ordinal", "padding:none")
                | ("second", "padding:none")
                | ("week_number", "padding:none")
                | ("year", "padding:none") => modifiers.padding = Some(Padding::None),
                ("hour", "repr:24") => modifiers.hour_is_12_hour_clock = Some(false),
                ("hour", "repr:12") => modifiers.hour_is_12_hour_clock = Some(true),
                ("month", "repr:numerical") => modifiers.month_repr = Some(MonthRepr::Numerical),
                ("month", "repr:long") => modifiers.month_repr = Some(MonthRepr::Long),
                ("month", "repr:short") => modifiers.month_repr = Some(MonthRepr::Short),
                ("offset_hour", "sign:automatic") | ("year", "sign:automatic") => {
                    modifiers.sign_is_mandatory = Some(false)
                }
                ("offset_hour", "sign:mandatory") | ("year", "sign:mandatory") => {
                    modifiers.sign_is_mandatory = Some(true)
                }
                ("period", "case:upper") => modifiers.period_is_uppercase = Some(true),
                ("period", "case:lower") => modifiers.period_is_uppercase = Some(false),
                ("subsecond", "digits:1") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::One)
                }
                ("subsecond", "digits:2") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Two)
                }
                ("subsecond", "digits:3") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Three)
                }
                ("subsecond", "digits:4") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Four)
                }
                ("subsecond", "digits:5") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Five)
                }
                ("subsecond", "digits:6") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Six)
                }
                ("subsecond", "digits:7") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Seven)
                }
                ("subsecond", "digits:8") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Eight)
                }
                ("subsecond", "digits:9") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::Nine)
                }
                ("subsecond", "digits:1+") => {
                    modifiers.subsecond_digits = Some(SubsecondDigits::OneOrMore)
                }
                ("weekday", "repr:short") => modifiers.weekday_repr = Some(WeekdayRepr::Short),
                ("weekday", "repr:long") => modifiers.weekday_repr = Some(WeekdayRepr::Long),
                ("weekday", "repr:sunday") => modifiers.weekday_repr = Some(WeekdayRepr::Sunday),
                ("weekday", "repr:monday") => modifiers.weekday_repr = Some(WeekdayRepr::Monday),
                ("weekday", "one_indexed:true") => modifiers.weekday_is_one_indexed = Some(true),
                ("weekday", "one_indexed:false") => modifiers.weekday_is_one_indexed = Some(false),
                ("week_number", "repr:iso") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Iso)
                }
                ("week_number", "repr:sunday") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Sunday)
                }
                ("week_number", "repr:monday") => {
                    modifiers.week_number_repr = Some(WeekNumberRepr::Monday)
                }
                ("year", "repr:full") => modifiers.year_repr = Some(YearRepr::Full),
                ("year", "repr:century") => modifiers.year_repr = Some(YearRepr::Century),
                ("year", "repr:last_two") => modifiers.year_repr = Some(YearRepr::LastTwo),
                ("year", "base:calendar") => modifiers.year_is_iso_week_based = Some(false),
                ("year", "base:iso_week") => modifiers.year_is_iso_week_based = Some(true),
                _ => {
                    return Err(InvalidFormatDescription::InvalidModifier {
                        value: modifier.to_owned(),
                        index: *index,
                    })
                }
            }
        }

        Ok(modifiers)
    }
}
