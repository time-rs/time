//! Information parsed from an input and format description.

use crate::{parsing::Error, Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};
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
    pub sunday_week: Option<u8>,
    /// Week of the year, where week one begins on the first Monday of the calendar year.
    pub monday_week: Option<u8>,
    /// Week of the year, where week one is the Monday-to-Sunday period containing January 4.
    pub iso_week: Option<NonZeroU8>,
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
    pub nanosecond: Option<u32>,
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
            sunday_week: None,
            monday_week: None,
            iso_week: None,
            weekday: None,
            ordinal: None,
            day: None,
            hour_24: None,
            hour_12: None,
            hour_12_is_pm: None,
            minute: None,
            second: None,
            nanosecond: None,
            offset_hour: None,
            offset_minute: None,
            offset_second: None,
        }
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
