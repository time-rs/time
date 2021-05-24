//! Information parsed from an input and format description.

use core::convert::{TryFrom, TryInto};
use core::num::{NonZeroU16, NonZeroU8};

use crate::error::TryFromParsed::InsufficientInformation;
use crate::format_description::modifier::{WeekNumberRepr, YearRepr};
use crate::format_description::Component;
use crate::parsing::component::{
    parse_day, parse_hour, parse_minute, parse_month, parse_offset_hour, parse_offset_minute,
    parse_offset_second, parse_ordinal, parse_period, parse_second, parse_subsecond,
    parse_week_number, parse_weekday, parse_year, Period,
};
use crate::parsing::ParsedItem;
use crate::{error, Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

/// All information parsed.
///
/// This information is directly used to construct the final values.
///
/// Most users will not need think about this struct in any way. It is public to allow for manual
/// control over values, in the instance that the default parser is insufficient.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct Parsed {
    /// Calendar year.
    pub year: Option<i32>,
    /// The last two digits of the calendar year.
    pub year_last_two: Option<u8>,
    /// Year of the [ISO week date](https://en.wikipedia.org/wiki/ISO_week_date).
    pub iso_year: Option<i32>,
    /// The last two digits of the ISO week year.
    pub iso_year_last_two: Option<u8>,
    /// Month of the year.
    pub month: Option<Month>,
    /// Week of the year, where week one begins on the first Sunday of the calendar year.
    pub sunday_week_number: Option<u8>,
    /// Week of the year, where week one begins on the first Monday of the calendar year.
    pub monday_week_number: Option<u8>,
    /// Week of the year, where week one is the Monday-to-Sunday period containing January 4.
    pub iso_week_number: Option<NonZeroU8>,
    /// Day of the week.
    pub weekday: Option<Weekday>,
    /// Day of the year.
    pub ordinal: Option<NonZeroU16>,
    /// Day of the month.
    pub day: Option<NonZeroU8>,
    /// Hour within the day.
    pub hour_24: Option<u8>,
    /// Hour within the 12-hour period (midnight to noon or vice versa). This is typically used in
    /// conjunction with AM/PM, which is indicated by the `hour_12_is_pm` field.
    pub hour_12: Option<NonZeroU8>,
    /// Whether the `hour_12` field indicates a time that "PM".
    pub hour_12_is_pm: Option<bool>,
    /// Minute within the hour.
    pub minute: Option<u8>,
    /// Second within the minute.
    pub second: Option<u8>,
    /// Nanosecond within the second.
    pub subsecond: Option<u32>,
    /// Whole hours of the UTC offset.
    pub offset_hour: Option<i8>,
    /// Minutes within the hour of the UTC offset.
    pub offset_minute: Option<u8>,
    /// Seconds within the minute of the UTC offset.
    pub offset_second: Option<u8>,
}

impl Parsed {
    /// Create a new instance of `Parsed` with no information known.
    pub const fn new() -> Self {
        Self {
            year: None,
            year_last_two: None,
            iso_year: None,
            iso_year_last_two: None,
            month: None,
            sunday_week_number: None,
            monday_week_number: None,
            iso_week_number: None,
            weekday: None,
            ordinal: None,
            day: None,
            hour_24: None,
            hour_12: None,
            hour_12_is_pm: None,
            minute: None,
            second: None,
            subsecond: None,
            offset_hour: None,
            offset_minute: None,
            offset_second: None,
        }
    }

    /// Parse a single component, mutating the struct. The remaining input is returned as the `Ok`
    /// value.
    pub fn parse_component<'a>(
        &mut self,
        input: &'a [u8],
        component: Component,
    ) -> Result<&'a [u8], error::ParseFromDescription> {
        use error::ParseFromDescription::InvalidComponent;

        match component {
            Component::Day(modifiers) => Ok(parse_day(input, modifiers)
                .ok_or(InvalidComponent("day"))?
                .assign_value_to(&mut self.day)),
            Component::Month(modifiers) => Ok(parse_month(input, modifiers)
                .ok_or(InvalidComponent("month"))?
                .assign_value_to(&mut self.month)),
            Component::Ordinal(modifiers) => Ok(parse_ordinal(input, modifiers)
                .ok_or(InvalidComponent("ordinal"))?
                .assign_value_to(&mut self.ordinal)),
            Component::Weekday(modifiers) => Ok(parse_weekday(input, modifiers)
                .ok_or(InvalidComponent("weekday"))?
                .assign_value_to(&mut self.weekday)),
            Component::WeekNumber(modifiers) => {
                let ParsedItem(remaining, value) =
                    parse_week_number(input, modifiers).ok_or(InvalidComponent("week number"))?;
                match modifiers.repr {
                    WeekNumberRepr::Iso => {
                        self.iso_week_number =
                            Some(NonZeroU8::new(value).ok_or(InvalidComponent("week number"))?)
                    }
                    WeekNumberRepr::Sunday => self.sunday_week_number = Some(value),
                    WeekNumberRepr::Monday => self.monday_week_number = Some(value),
                }
                Ok(remaining)
            }
            Component::Year(modifiers) => {
                let ParsedItem(remaining, value) =
                    parse_year(input, modifiers).ok_or(InvalidComponent("year"))?;
                match (modifiers.iso_week_based, modifiers.repr) {
                    (false, YearRepr::Full) => self.year = Some(value),
                    (false, YearRepr::LastTwo) => self.year_last_two = Some(value as u8),
                    (true, YearRepr::Full) => self.iso_year = Some(value),
                    (true, YearRepr::LastTwo) => self.iso_year_last_two = Some(value as u8),
                }
                Ok(remaining)
            }
            Component::Hour(modifiers) => {
                let ParsedItem(remaining, value) =
                    parse_hour(input, modifiers).ok_or(InvalidComponent("hour"))?;
                if modifiers.is_12_hour_clock {
                    self.hour_12 = Some(NonZeroU8::new(value).ok_or(InvalidComponent("hour"))?);
                } else {
                    self.hour_24 = Some(value);
                }
                Ok(remaining)
            }
            Component::Minute(modifiers) => Ok(parse_minute(input, modifiers)
                .ok_or(InvalidComponent("minute"))?
                .assign_value_to(&mut self.minute)),
            Component::Period(modifiers) => Ok(parse_period(input, modifiers)
                .ok_or(InvalidComponent("period"))?
                .map(|period| period == Period::Pm)
                .assign_value_to(&mut self.hour_12_is_pm)),
            Component::Second(modifiers) => Ok(parse_second(input, modifiers)
                .ok_or(InvalidComponent("second"))?
                .assign_value_to(&mut self.second)),
            Component::Subsecond(modifiers) => Ok(parse_subsecond(input, modifiers)
                .ok_or(InvalidComponent("subsecond"))?
                .assign_value_to(&mut self.subsecond)),
            Component::OffsetHour(modifiers) => Ok(parse_offset_hour(input, modifiers)
                .ok_or(InvalidComponent("offset hour"))?
                .assign_value_to(&mut self.offset_hour)),
            Component::OffsetMinute(modifiers) => Ok(parse_offset_minute(input, modifiers)
                .ok_or(InvalidComponent("offset minute"))?
                .assign_value_to(&mut self.offset_minute)),
            Component::OffsetSecond(modifiers) => Ok(parse_offset_second(input, modifiers)
                .ok_or(InvalidComponent("offset second"))?
                .assign_value_to(&mut self.offset_second)),
        }
    }
}

impl TryFrom<Parsed> for Date {
    type Error = error::TryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        macro_rules! items {
            ($($item:ident),+ $(,)?) => {
                Parsed { $($item: Some($item)),*, .. }
            };
        }

        /// Get the value needed to adjust the ordinal day for Sunday and Monday-based week
        /// numbering.
        const fn adjustment(year: i32) -> i16 {
            match Date::__from_ordinal_date_unchecked(year, 1).weekday() {
                Weekday::Monday => 7,
                Weekday::Tuesday => 1,
                Weekday::Wednesday => 2,
                Weekday::Thursday => 3,
                Weekday::Friday => 4,
                Weekday::Saturday => 5,
                Weekday::Sunday => 6,
            }
        }

        // TODO Only the basics have been covered. There are many other valid values that are not
        // currently constructed from the information known.

        match parsed {
            items!(year, ordinal) => Ok(Self::from_ordinal_date(year, ordinal.get())?),
            items!(year, month, day) => Ok(Self::from_calendar_date(year, month, day.get())?),
            items!(iso_year, iso_week_number, weekday) => Ok(Self::from_iso_week_date(
                iso_year,
                iso_week_number.get(),
                weekday,
            )?),
            items!(year, sunday_week_number, weekday) => Ok(Self::from_ordinal_date(
                year,
                (sunday_week_number as i16 * 7 + weekday.number_days_from_sunday() as i16
                    - adjustment(year)
                    + 1) as u16,
            )?),
            items!(year, monday_week_number, weekday) => Ok(Self::from_ordinal_date(
                year,
                (monday_week_number as i16 * 7 + weekday.number_days_from_monday() as i16
                    - adjustment(year)
                    + 1) as u16,
            )?),
            _ => Err(InsufficientInformation),
        }
    }
}

impl TryFrom<Parsed> for Time {
    type Error = error::TryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        let hour = match (parsed.hour_24, parsed.hour_12, parsed.hour_12_is_pm) {
            (Some(hour), _, _) => hour,
            (_, Some(hour), Some(false)) if hour.get() == 12 => 0,
            (_, Some(hour), Some(true)) if hour.get() == 12 => 12,
            (_, Some(hour), Some(false)) => hour.get(),
            (_, Some(hour), Some(true)) => hour.get() + 12,
            _ => return Err(InsufficientInformation),
        };
        let minute = parsed.minute.ok_or(InsufficientInformation)?;
        let second = parsed.second.unwrap_or(0);
        let subsecond = parsed.subsecond.unwrap_or(0);
        Ok(Self::from_hms_nano(hour, minute, second, subsecond)?)
    }
}

impl TryFrom<Parsed> for UtcOffset {
    type Error = error::TryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        let hour = parsed.offset_hour.ok_or(InsufficientInformation)?;
        let minute = parsed.offset_minute.unwrap_or(0);
        let second = parsed.offset_second.unwrap_or(0);
        Ok(Self::from_hms(hour, minute as i8, second as i8)?)
    }
}

impl TryFrom<Parsed> for PrimitiveDateTime {
    type Error = error::TryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        Ok(Self::new(parsed.try_into()?, parsed.try_into()?))
    }
}

impl TryFrom<Parsed> for OffsetDateTime {
    type Error = error::TryFromParsed;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        Ok(PrimitiveDateTime::try_from(parsed)?.assume_offset(parsed.try_into()?))
    }
}
