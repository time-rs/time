//! Information parsed from an input and format description.

use crate::{
    format_description::{
        modifier::{WeekNumberRepr, YearRepr},
        Component, FormatDescription,
    },
    parsing::{
        combinator,
        date::{
            parse_day, parse_month, parse_ordinal, parse_week_number, parse_weekday, parse_year,
        },
        offset::{parse_offset_hour, parse_offset_minute, parse_offset_second},
        time::{parse_hour, parse_minute, parse_period, parse_second, parse_subsecond, Period},
        Error,
    },
    Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
};
use core::{
    convert::{TryFrom, TryInto},
    num::{NonZeroU16, NonZeroU8},
};

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
    pub month: Option<NonZeroU8>,
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

    /// Parse a given string into its components from the provided format description.
    pub fn parse_from_description<'a>(
        mut input: &'a str,
        format_description: &FormatDescription<'a>,
    ) -> Result<Self, Error> {
        let mut parsed = Self::new();
        parsed._parse_from_description(&mut input, format_description)?;
        Ok(parsed)
    }

    /// Parse a given string into its components from the provided format description.
    fn _parse_from_description<'a>(
        &mut self,
        input: &mut &'a str,
        format_description: &FormatDescription<'a>,
    ) -> Result<(), Error> {
        match format_description {
            FormatDescription::Literal(literal) => {
                combinator::string(literal)(input).ok_or(Error::InvalidLiteral)?;
            }
            FormatDescription::Component(component) => {
                self.parse_component(input, *component)?;
            }
            FormatDescription::BorrowedCompound(compound) => {
                for format_description in *compound {
                    self._parse_from_description(input, format_description)?;
                }
            }
            #[cfg(feature = "alloc")]
            FormatDescription::OwnedCompound(compound) => {
                for format_description in compound {
                    self._parse_from_description(input, format_description)?;
                }
            }
        }

        Ok(())
    }

    /// Parse a single component, mutating the provided `Parsed` struct.
    fn parse_component<'a>(
        &mut self,
        input: &mut &'a str,
        component: Component,
    ) -> Result<(), Error> {
        match component {
            Component::Day(modifiers) => {
                self.day = Some(parse_day(input, modifiers).ok_or(Error::InvalidComponent("day"))?);
            }
            Component::Month(modifiers) => {
                self.month =
                    Some(parse_month(input, modifiers).ok_or(Error::InvalidComponent("month"))?);
            }
            Component::Ordinal(modifiers) => {
                self.ordinal = Some(
                    parse_ordinal(input, modifiers).ok_or(Error::InvalidComponent("ordinal"))?,
                );
            }
            Component::Weekday(modifiers) => {
                self.weekday = Some(
                    parse_weekday(input, modifiers).ok_or(Error::InvalidComponent("weekday"))?,
                );
            }
            Component::WeekNumber(modifiers) => {
                let value = parse_week_number(input, modifiers)
                    .ok_or(Error::InvalidComponent("week number"))?;
                match modifiers.repr {
                    WeekNumberRepr::Iso => {
                        self.iso_week_number = Some(
                            NonZeroU8::new(value).ok_or(Error::InvalidComponent("week number"))?,
                        )
                    }
                    WeekNumberRepr::Sunday => self.sunday_week_number = Some(value),
                    WeekNumberRepr::Monday => self.monday_week_number = Some(value),
                }
            }
            Component::Year(modifiers) => {
                let value = parse_year(input, modifiers).ok_or(Error::InvalidComponent("year"))?;
                match (modifiers.iso_week_based, modifiers.repr) {
                    (false, YearRepr::Full) => self.iso_year = Some(value),
                    (false, YearRepr::LastTwo) => self.iso_year_last_two = Some(value as u8),
                    (true, YearRepr::Full) => self.year = Some(value),
                    (true, YearRepr::LastTwo) => self.year_last_two = Some(value as u8),
                }
            }
            Component::Hour(modifiers) => {
                let value = parse_hour(input, modifiers).ok_or(Error::InvalidComponent("hour"))?;
                if modifiers.is_12_hour_clock {
                    self.hour_12 =
                        Some(NonZeroU8::new(value).ok_or(Error::InvalidComponent("hour"))?);
                } else {
                    self.hour_24 = Some(value);
                }
            }
            Component::Minute(modifiers) => {
                self.minute =
                    Some(parse_minute(input, modifiers).ok_or(Error::InvalidComponent("minute"))?);
            }
            Component::Period(modifiers) => {
                self.hour_12_is_pm = Some(
                    parse_period(input, modifiers).ok_or(Error::InvalidComponent("period"))?
                        == Period::Pm,
                );
            }
            Component::Second(modifiers) => {
                self.second =
                    Some(parse_second(input, modifiers).ok_or(Error::InvalidComponent("second"))?);
            }
            Component::Subsecond(modifiers) => {
                self.subsecond = Some(
                    parse_subsecond(input, modifiers)
                        .ok_or(Error::InvalidComponent("subsecond"))?,
                );
            }
            Component::OffsetHour(modifiers) => {
                self.offset_hour = Some(
                    parse_offset_hour(input, modifiers)
                        .ok_or(Error::InvalidComponent("offset hour"))?,
                );
            }
            Component::OffsetMinute(modifiers) => {
                self.offset_minute = Some(
                    parse_offset_minute(input, modifiers)
                        .ok_or(Error::InvalidComponent("offset minute"))?,
                );
            }
            Component::OffsetSecond(modifiers) => {
                self.offset_second = Some(
                    parse_offset_second(input, modifiers)
                        .ok_or(Error::InvalidComponent("offset second"))?,
                );
            }
        }

        Ok(())
    }
}

impl TryFrom<Parsed> for Date {
    type Error = Error;

    fn try_from(_parsed: Parsed) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Parsed> for Time {
    type Error = Error;

    fn try_from(_parsed: Parsed) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Parsed> for UtcOffset {
    type Error = Error;

    fn try_from(_parsed: Parsed) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Parsed> for PrimitiveDateTime {
    type Error = Error;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        Ok(Self::new(parsed.try_into()?, parsed.try_into()?))
    }
}

impl TryFrom<Parsed> for OffsetDateTime {
    type Error = Error;

    fn try_from(parsed: Parsed) -> Result<Self, Self::Error> {
        Ok(PrimitiveDateTime::try_from(parsed)?.assume_offset(parsed.try_into()?))
    }
}
