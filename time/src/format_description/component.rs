//! Part of a format description.

use crate::format_description::modifier;

/// Indicate whether the hour is "am" or "pm".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Period {
    #[allow(clippy::missing_docs_in_private_items)]
    Am,
    #[allow(clippy::missing_docs_in_private_items)]
    Pm,
}

/// A component of a larger format description.
#[non_exhaustive]
#[allow(deprecated)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    /// Day of the month.
    Day(modifier::Day),
    /// Month of the year in the abbreviated form (e.g. "Jan").
    MonthShort(modifier::MonthShort),
    /// Month of the year in the full form (e.g. "January").
    MonthLong(modifier::MonthLong),
    /// Month of the year in the numerical form (e.g. "1" for January).
    MonthNumerical(modifier::MonthNumerical),
    /// Ordinal day of the year.
    Ordinal(modifier::Ordinal),
    /// Weekday in the abbreviated form (e.g. "Mon").
    WeekdayShort(modifier::WeekdayShort),
    /// Weekday in the full form (e.g. "Monday").
    WeekdayLong(modifier::WeekdayLong),
    /// Weekday number where Sunday is either 0 or 1 depending on the modifier.
    WeekdaySunday(modifier::WeekdaySunday),
    /// Weekday number where Monday is either 0 or 1 depending on the modifier.
    WeekdayMonday(modifier::WeekdayMonday),
    /// Week number of the year, where week 1 starts is the week beginning on Monday that contains
    /// January 4.
    WeekNumberIso(modifier::WeekNumberIso),
    /// Week number of the year, where week 1 starts on the first Sunday of the calendar year.
    WeekNumberSunday(modifier::WeekNumberSunday),
    /// Week number of the year, where week 1 starts on the first Monday of the calendar year.
    WeekNumberMonday(modifier::WeekNumberMonday),
    /// The calendar year. Supports the extended range.
    CalendarYearFullExtendedRange(modifier::CalendarYearFullExtendedRange),
    /// The calendar year. Does not support the extended range.
    CalendarYearFullStandardRange(modifier::CalendarYearFullStandardRange),
    /// The ISO week-based year. Supports the extended range.
    IsoYearFullExtendedRange(modifier::IsoYearFullExtendedRange),
    /// The ISO week-based year. Does not support the extended range.
    IsoYearFullStandardRange(modifier::IsoYearFullStandardRange),
    /// The century of the calendar year. Supports the extended range.
    CalendarYearCenturyExtendedRange(modifier::CalendarYearCenturyExtendedRange),
    /// The century of the calendar year. Does not support the extended range.
    CalendarYearCenturyStandardRange(modifier::CalendarYearCenturyStandardRange),
    /// The century of the ISO week-based year. Supports the extended range.
    IsoYearCenturyExtendedRange(modifier::IsoYearCenturyExtendedRange),
    /// The century of the ISO week-based year. Does not support the extended range.
    IsoYearCenturyStandardRange(modifier::IsoYearCenturyStandardRange),
    /// The last two digits of the calendar year.
    CalendarYearLastTwo(modifier::CalendarYearLastTwo),
    /// The last two digits of the ISO week-based year.
    IsoYearLastTwo(modifier::IsoYearLastTwo),
    /// Hour of the day using the 12-hour clock.
    Hour12(modifier::Hour12),
    /// Hour of the day using the 24-hour clock.
    Hour24(modifier::Hour24),
    /// Minute within the hour.
    Minute(modifier::Minute),
    /// AM/PM part of the time.
    Period(modifier::Period),
    /// Second within the minute.
    Second(modifier::Second),
    /// Subsecond within the second.
    Subsecond(modifier::Subsecond),
    /// Hour of the UTC offset.
    OffsetHour(modifier::OffsetHour),
    /// Minute within the hour of the UTC offset.
    OffsetMinute(modifier::OffsetMinute),
    /// Second within the minute of the UTC offset.
    OffsetSecond(modifier::OffsetSecond),
    /// A number of bytes to ignore when parsing. This has no effect on formatting.
    Ignore(modifier::Ignore),
    /// A Unix timestamp in seconds.
    UnixTimestampSecond(modifier::UnixTimestampSecond),
    /// A Unix timestamp in milliseconds.
    UnixTimestampMillisecond(modifier::UnixTimestampMillisecond),
    /// A Unix timestamp in microseconds.
    UnixTimestampMicrosecond(modifier::UnixTimestampMicrosecond),
    /// A Unix timestamp in nanoseconds.
    UnixTimestampNanosecond(modifier::UnixTimestampNanosecond),
    /// The end of input. Parsing this component will fail if there is any input remaining. This
    /// component neither affects formatting nor consumes any input when parsing.
    End(modifier::End),

    // Start of deprecated components that are no longer emitted by macros or parsers. They must
    // be maintained for backward compatibility, as downstream users could have constructed them
    // manually.
    /// Month of the year.
    #[deprecated(
        since = "0.3.48",
        note = "use `MonthShort`, `MonthLong`, or `MonthNumeric` instead"
    )]
    Month(modifier::Month),
    /// Day of the week.
    #[deprecated(
        since = "0.3.48",
        note = "use `WeekdayShort`, `WeekdayLong`, or `WeekdaySunday`, or `WeekdayMonday` instead"
    )]
    Weekday(modifier::Weekday),
    /// Week within the year.
    #[deprecated(
        since = "0.3.48",
        note = "use `WeekNumberIso`, `WeekNumberSunday`, or `WeekNumberMonday` instead"
    )]
    WeekNumber(modifier::WeekNumber),
    /// Hour of the day.
    #[deprecated(since = "0.3.48", note = "use `Hour12` or `Hour24` instead")]
    Hour(modifier::Hour),
    /// A Unix timestamp.
    #[deprecated(
        since = "0.3.48",
        note = "use `UnixTimestampSeconds`, `UnixTimestampMilliseconds`, \
                `UnixTimestampMicroseconds`, or `UnixTimestampNanoseconds` instead"
    )]
    UnixTimestamp(modifier::UnixTimestamp),
    /// Year of the date.
    #[deprecated(
        since = "0.3.48",
        note = "use one of the various `Year*` components instead"
    )]
    Year(modifier::Year),
}
