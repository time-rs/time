//! A trait that can be used to format an item from its components.

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Deref;
use std::io;

use deranged::{ru8, ru16};
use num_conv::prelude::*;

use crate::format_description::format_description_v3::FormatDescriptionV3Inner;
use crate::format_description::modifier::Padding;
use crate::format_description::well_known::iso8601::EncodedConfig;
use crate::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use crate::format_description::{BorrowedFormatItem, FormatDescriptionV3, OwnedFormatItem};
use crate::formatting::{
    ComponentProvider, MONTH_NAMES, WEEKDAY_NAMES, format_four_digits_pad_zero, format_two_digits,
    iso8601, write, write_bytes, write_if_else,
};
use crate::internal_macros::try_likely_ok;
use crate::{error, num_fmt};

/// A type that describes a format.
///
/// Implementors of [`Formattable`] are [format descriptions](crate::format_description).
///
/// To format a value into a String, use the `format` method on the respective type.
#[cfg_attr(docsrs, doc(notable_trait))]
pub trait Formattable: sealed::Sealed {}
impl Formattable for FormatDescriptionV3<'_> {}
impl Formattable for BorrowedFormatItem<'_> {}
impl Formattable for [BorrowedFormatItem<'_>] {}
impl Formattable for OwnedFormatItem {}
impl Formattable for [OwnedFormatItem] {}
impl Formattable for Rfc3339 {}
impl Formattable for Rfc2822 {}
impl<const CONFIG: EncodedConfig> Formattable for Iso8601<CONFIG> {}
impl<T> Formattable for T where T: Deref<Target: Formattable> {}

/// Seal the trait to prevent downstream users from implementing it.
mod sealed {
    use super::*;
    use crate::formatting::ComponentProvider;
    use crate::formatting::metadata::ComputeMetadata;

    /// Format the item using a format description, the intended output, and the various components.
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    pub trait Sealed: ComputeMetadata {
        /// Format the item into the provided output, returning the number of bytes written.
        fn format_into<V>(
            &self,
            output: &mut (impl io::Write + ?Sized),
            value: &V,
            state: &mut V::State,
        ) -> Result<usize, error::Format>
        where
            V: ComponentProvider;

        /// Format the item directly to a `String`.
        #[inline]
        fn format<V>(&self, value: &V, state: &mut V::State) -> Result<String, error::Format>
        where
            V: ComponentProvider,
        {
            let crate::formatting::metadata::Metadata {
                max_bytes_needed,
                guaranteed_utf8,
            } = self.compute_metadata();

            let mut buf = Vec::with_capacity(max_bytes_needed);
            try_likely_ok!(self.format_into(&mut buf, value, state));
            Ok(if guaranteed_utf8 {
                // Safety: The output is guaranteed to be UTF-8.
                unsafe { String::from_utf8_unchecked(buf) }
            } else {
                String::from_utf8_lossy(&buf).into_owned()
            })
        }
    }
}

impl sealed::Sealed for FormatDescriptionV3<'_> {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    #[inline]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        self.inner.format_into(output, value, state)
    }
}

impl sealed::Sealed for FormatDescriptionV3Inner<'_> {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    #[inline]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        use FormatDescriptionV3Inner::*;

        use crate::formatting::*;

        match &self {
            Day(modifier) if V::SUPPLIES_DATE => {
                fmt_day(output, value.day(state), *modifier).map_err(Into::into)
            }
            MonthShort(modifier) if V::SUPPLIES_DATE => {
                fmt_month_short(output, value.month(state), *modifier).map_err(Into::into)
            }
            MonthLong(modifier) if V::SUPPLIES_DATE => {
                fmt_month_long(output, value.month(state), *modifier).map_err(Into::into)
            }
            MonthNumerical(modifier) if V::SUPPLIES_DATE => {
                fmt_month_numerical(output, value.month(state), *modifier).map_err(Into::into)
            }
            Ordinal(modifier) if V::SUPPLIES_DATE => {
                fmt_ordinal(output, value.ordinal(state), *modifier).map_err(Into::into)
            }
            WeekdayShort(modifier) if V::SUPPLIES_DATE => {
                fmt_weekday_short(output, value.weekday(state), *modifier).map_err(Into::into)
            }
            WeekdayLong(modifier) if V::SUPPLIES_DATE => {
                fmt_weekday_long(output, value.weekday(state), *modifier).map_err(Into::into)
            }
            WeekdaySunday(modifier) if V::SUPPLIES_DATE => {
                fmt_weekday_sunday(output, value.weekday(state), *modifier).map_err(Into::into)
            }
            WeekdayMonday(modifier) if V::SUPPLIES_DATE => {
                fmt_weekday_monday(output, value.weekday(state), *modifier).map_err(Into::into)
            }
            WeekNumberIso(modifier) if V::SUPPLIES_DATE => {
                fmt_week_number_iso(output, value.iso_week_number(state), *modifier)
                    .map_err(Into::into)
            }
            WeekNumberSunday(modifier) if V::SUPPLIES_DATE => {
                fmt_week_number_sunday(output, value.sunday_based_week(state), *modifier)
                    .map_err(Into::into)
            }
            WeekNumberMonday(modifier) if V::SUPPLIES_DATE => {
                fmt_week_number_monday(output, value.monday_based_week(state), *modifier)
                    .map_err(Into::into)
            }
            CalendarYearFullExtendedRange(modifier) if V::SUPPLIES_DATE => {
                fmt_calendar_year_full_extended_range(output, value.calendar_year(state), *modifier)
                    .map_err(Into::into)
            }
            CalendarYearFullStandardRange(modifier) if V::SUPPLIES_DATE => {
                fmt_calendar_year_full_standard_range(
                    output,
                    try_likely_ok!(
                        value
                            .calendar_year(state)
                            .narrow::<-9_999, 9_999>()
                            .ok_or_else(|| error::ComponentRange::conditional("year"))
                    )
                    .into(),
                    *modifier,
                )
                .map_err(Into::into)
            }
            IsoYearFullExtendedRange(modifier) if V::SUPPLIES_DATE => {
                fmt_iso_year_full_extended_range(output, value.iso_year(state), *modifier)
                    .map_err(Into::into)
            }
            IsoYearFullStandardRange(modifier) if V::SUPPLIES_DATE => {
                fmt_iso_year_full_standard_range(
                    output,
                    try_likely_ok!(
                        value
                            .iso_year(state)
                            .narrow::<-9_999, 9_999>()
                            .ok_or_else(|| error::ComponentRange::conditional("year"))
                    )
                    .into(),
                    *modifier,
                )
                .map_err(Into::into)
            }
            CalendarYearCenturyExtendedRange(modifier) if V::SUPPLIES_DATE => {
                let year = value.calendar_year(state);
                // Safety: Given the range of `year`, the range of the century is
                // `-9_999..=9_999`.
                let century = unsafe { ri16::new_unchecked((year.get() / 100).truncate()) };
                fmt_calendar_year_century_extended_range(
                    output,
                    century,
                    year.is_negative(),
                    *modifier,
                )
                .map_err(Into::into)
            }
            CalendarYearCenturyStandardRange(modifier) if V::SUPPLIES_DATE => {
                let year = value.calendar_year(state);
                let is_negative = year.is_negative();
                // Safety: Given the range of `year`, the range of the century is
                // `-9_999..=9_999`.
                let year =
                    unsafe { ri16::<-9_999, 9_999>::new_unchecked((year.get() / 100).truncate()) };
                fmt_calendar_year_century_standard_range(
                    output,
                    year.narrow::<-99, 99>()
                        .ok_or_else(|| error::ComponentRange::conditional("year"))?
                        .into(),
                    is_negative,
                    *modifier,
                )
                .map_err(Into::into)
            }
            IsoYearCenturyExtendedRange(modifier) if V::SUPPLIES_DATE => {
                let year = value.iso_year(state);
                let is_negative = year.is_negative();
                // Safety: Given the range of `year`, the range of the century is
                // `-9_999..=9_999`.
                let century = unsafe { ri16::new_unchecked((year.get() / 100).truncate()) };
                fmt_iso_year_century_extended_range(output, century, is_negative, *modifier)
                    .map_err(Into::into)
            }
            IsoYearCenturyStandardRange(modifier) if V::SUPPLIES_DATE => {
                let year = value.iso_year(state);
                let is_negative = year.is_negative();
                // Safety: Given the range of `year`, the range of the century is
                // `-9_999..=9_999`.
                let year =
                    unsafe { ri16::<-9_999, 9_999>::new_unchecked((year.get() / 100).truncate()) };
                fmt_iso_year_century_standard_range(
                    output,
                    year.narrow::<-99, 99>()
                        .ok_or_else(|| error::ComponentRange::conditional("year"))?
                        .into(),
                    is_negative,
                    *modifier,
                )
                .map_err(Into::into)
            }
            CalendarYearLastTwo(modifier) if V::SUPPLIES_DATE => {
                // Safety: Modulus of 100 followed by `.unsigned_abs()` guarantees that the
                // value is in the range `0..=99`.
                let last_two = unsafe {
                    ru8::new_unchecked(
                        (value.calendar_year(state).get().unsigned_abs() % 100).truncate(),
                    )
                };
                fmt_calendar_year_last_two(output, last_two, *modifier).map_err(Into::into)
            }
            IsoYearLastTwo(modifier) if V::SUPPLIES_DATE => {
                // Safety: Modulus of 100 followed by `.unsigned_abs()` guarantees that the
                // value is in the range `0..=99`.
                let last_two = unsafe {
                    ru8::new_unchecked(
                        (value.iso_year(state).get().unsigned_abs() % 100).truncate(),
                    )
                };
                fmt_iso_year_last_two(output, last_two, *modifier).map_err(Into::into)
            }
            Hour12(modifier) if V::SUPPLIES_TIME => {
                fmt_hour_12(output, value.hour(state), *modifier).map_err(Into::into)
            }
            Hour24(modifier) if V::SUPPLIES_TIME => {
                fmt_hour_24(output, value.hour(state), *modifier).map_err(Into::into)
            }
            Minute(modifier) if V::SUPPLIES_TIME => {
                fmt_minute(output, value.minute(state), *modifier).map_err(Into::into)
            }
            Period(modifier) if V::SUPPLIES_TIME => {
                fmt_period(output, value.period(state), *modifier).map_err(Into::into)
            }
            Second(modifier) if V::SUPPLIES_TIME => {
                fmt_second(output, value.second(state), *modifier).map_err(Into::into)
            }
            Subsecond(modifier) if V::SUPPLIES_TIME => {
                fmt_subsecond(output, value.nanosecond(state), *modifier).map_err(Into::into)
            }
            OffsetHour(modifier) if V::SUPPLIES_OFFSET => fmt_offset_hour(
                output,
                value.offset_is_negative(state),
                value.offset_hour(state),
                *modifier,
            )
            .map_err(Into::into),
            OffsetMinute(modifier) if V::SUPPLIES_OFFSET => {
                fmt_offset_minute(output, value.offset_minute(state), *modifier).map_err(Into::into)
            }
            OffsetSecond(modifier) if V::SUPPLIES_OFFSET => {
                fmt_offset_second(output, value.offset_second(state), *modifier).map_err(Into::into)
            }
            Ignore(_) => Ok(0),
            UnixTimestampSecond(modifier) if V::SUPPLIES_TIMESTAMP => {
                fmt_unix_timestamp_second(output, value.unix_timestamp_seconds(state), *modifier)
                    .map_err(Into::into)
            }
            UnixTimestampMillisecond(modifier) if V::SUPPLIES_TIMESTAMP => {
                fmt_unix_timestamp_millisecond(
                    output,
                    value.unix_timestamp_milliseconds(state),
                    *modifier,
                )
                .map_err(Into::into)
            }
            UnixTimestampMicrosecond(modifier) if V::SUPPLIES_TIMESTAMP => {
                fmt_unix_timestamp_microsecond(
                    output,
                    value.unix_timestamp_microseconds(state),
                    *modifier,
                )
                .map_err(Into::into)
            }
            UnixTimestampNanosecond(modifier) if V::SUPPLIES_TIMESTAMP => {
                fmt_unix_timestamp_nanosecond(
                    output,
                    value.unix_timestamp_nanoseconds(state),
                    *modifier,
                )
                .map_err(Into::into)
            }
            End(modifier::End { trailing_input: _ }) => Ok(0),
            Self::BorrowedLiteral(literal) => {
                write_bytes(output, literal.as_bytes()).map_err(Into::into)
            }
            Self::BorrowedCompound(items) => {
                let mut bytes = 0;
                for item in *items {
                    bytes += try_likely_ok!(item.format_into(output, value, state));
                }
                Ok(bytes)
            }
            Self::BorrowedOptional {
                format: should_format,
                item,
            } => {
                if *should_format {
                    item.format_into(output, value, state)
                } else {
                    Ok(0)
                }
            }
            Self::BorrowedFirst(items) => match items {
                [] => Ok(0),
                [item, ..] => item.format_into(output, value, state),
            },
            Self::OwnedLiteral(literal) => {
                write_bytes(output, literal.as_bytes()).map_err(Into::into)
            }
            Self::OwnedCompound(items) => {
                let mut bytes = 0;
                for item in &**items {
                    bytes += try_likely_ok!(item.format_into(output, value, state));
                }
                Ok(bytes)
            }
            Self::OwnedOptional {
                format: should_format,
                item,
            } => {
                if *should_format {
                    item.format_into(output, value, state)
                } else {
                    Ok(0)
                }
            }
            Self::OwnedFirst(items) => match &items[..] {
                [] => Ok(0),
                [item, ..] => item.format_into(output, value, state),
            },

            // This is functionally the same as a wildcard arm, but it will cause an error
            // if a new component is added. This is to avoid a bug where
            // a new component, the code compiles, and formatting fails.
            // Allow unreachable patterns because some branches may be fully matched above.
            #[allow(unreachable_patterns)]
            Day(_)
            | MonthShort(_)
            | MonthLong(_)
            | MonthNumerical(_)
            | Ordinal(_)
            | WeekdayShort(_)
            | WeekdayLong(_)
            | WeekdaySunday(_)
            | WeekdayMonday(_)
            | WeekNumberIso(_)
            | WeekNumberSunday(_)
            | WeekNumberMonday(_)
            | CalendarYearFullExtendedRange(_)
            | CalendarYearFullStandardRange(_)
            | IsoYearFullExtendedRange(_)
            | IsoYearFullStandardRange(_)
            | CalendarYearCenturyExtendedRange(_)
            | CalendarYearCenturyStandardRange(_)
            | IsoYearCenturyExtendedRange(_)
            | IsoYearCenturyStandardRange(_)
            | CalendarYearLastTwo(_)
            | IsoYearLastTwo(_)
            | Hour12(_)
            | Hour24(_)
            | Minute(_)
            | Period(_)
            | Second(_)
            | Subsecond(_)
            | OffsetHour(_)
            | OffsetMinute(_)
            | OffsetSecond(_)
            | Ignore(_)
            | UnixTimestampSecond(_)
            | UnixTimestampMillisecond(_)
            | UnixTimestampMicrosecond(_)
            | UnixTimestampNanosecond(_)
            | End(_) => Err(error::Format::InsufficientTypeInformation),
        }
    }
}

impl sealed::Sealed for BorrowedFormatItem<'_> {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    #[inline]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        Ok(match *self {
            #[expect(deprecated)]
            Self::Literal(literal) => try_likely_ok!(write_bytes(output, literal)),
            Self::StringLiteral(literal) => try_likely_ok!(write(output, literal)),
            Self::Component(component) => {
                FormatDescriptionV3Inner::<'_>::from(component).format_into(output, value, state)?
            }
            Self::Compound(items) => try_likely_ok!((*items).format_into(output, value, state)),
            Self::Optional(item) => try_likely_ok!((*item).format_into(output, value, state)),
            Self::First(items) => match items {
                [] => 0,
                [item, ..] => try_likely_ok!((*item).format_into(output, value, state)),
            },
        })
    }
}

impl sealed::Sealed for [BorrowedFormatItem<'_>] {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    #[inline]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        let mut bytes = 0;
        for item in self.iter() {
            bytes += try_likely_ok!(item.format_into(output, value, state));
        }
        Ok(bytes)
    }
}

impl sealed::Sealed for OwnedFormatItem {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    #[inline]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        match self {
            #[expect(deprecated)]
            Self::Literal(literal) => Ok(try_likely_ok!(write_bytes(output, literal))),
            Self::StringLiteral(literal) => Ok(try_likely_ok!(write(output, literal))),
            Self::Component(component) => {
                FormatDescriptionV3Inner::<'_>::from(*component).format_into(output, value, state)
            }
            Self::Compound(items) => (**items).format_into(output, value, state),
            Self::Optional(item) => (**item).format_into(output, value, state),
            Self::First(items) => match &**items {
                [] => Ok(0),
                [item, ..] => (*item).format_into(output, value, state),
            },
        }
    }
}

impl sealed::Sealed for [OwnedFormatItem] {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    #[inline]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        let mut bytes = 0;
        for item in self.iter() {
            bytes += try_likely_ok!(item.format_into(output, value, state));
        }
        Ok(bytes)
    }
}

impl<T> sealed::Sealed for T
where
    T: Deref<Target: sealed::Sealed>,
{
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    #[inline]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        self.deref().format_into(output, value, state)
    }
}

#[expect(
    private_bounds,
    private_interfaces,
    reason = "irrelevant due to being a sealed trait"
)]
impl sealed::Sealed for Rfc2822 {
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        const {
            assert!(
                V::SUPPLIES_DATE && V::SUPPLIES_TIME && V::SUPPLIES_OFFSET,
                "Rfc2822 requires date, time, and offset components, but not all can be provided \
                 by this type"
            );
        }

        let mut bytes = 0;

        if value.calendar_year(state).get() < 1900
            // The RFC requires years be exactly four digits.
            || (cfg!(feature = "large-dates") && value.calendar_year(state).get() >= 10_000)
        {
            crate::hint::cold_path();
            return Err(error::Format::InvalidComponent("year"));
        }
        if value.offset_second(state).get() != 0 {
            crate::hint::cold_path();
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        // Safety: All weekday names are at least 3 bytes long.
        bytes += try_likely_ok!(write(output, unsafe {
            WEEKDAY_NAMES[value
                .weekday(state)
                .number_days_from_monday()
                .extend::<usize>()]
            .get_unchecked(..3)
        }));
        bytes += try_likely_ok!(write(output, ", "));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.day(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, " "));
        // Safety: All month names are at least 3 bytes long.
        bytes += try_likely_ok!(write(output, unsafe {
            MONTH_NAMES[u8::from(value.month(state)).extend::<usize>() - 1].get_unchecked(..3)
        }));
        bytes += try_likely_ok!(write(output, " "));
        // Safety: Years with five or more digits were rejected above. Likewise with negative years.
        bytes += try_likely_ok!(format_four_digits_pad_zero(output, unsafe {
            ru16::new_unchecked(value.calendar_year(state).get().cast_unsigned().truncate())
        }));
        bytes += try_likely_ok!(write(output, " "));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.hour(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, ":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.minute(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, ":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.second(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, " "));
        bytes += try_likely_ok!(write_if_else(
            output,
            value.offset_is_negative(state),
            "-",
            "+"
        ));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `OffsetHours` is guaranteed to be in the range `-25..=25`, so the absolute
            // value is guaranteed to be in the range `0..=25`.
            unsafe { ru8::new_unchecked(value.offset_hour(state).get().unsigned_abs()) },
            Padding::Zero,
        ));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `OffsetMinutes` is guaranteed to be in the range `-59..=59`, so the absolute
            // value is guaranteed to be in the range `0..=59`.
            unsafe { ru8::new_unchecked(value.offset_minute(state).get().unsigned_abs()) },
            Padding::Zero,
        ));

        Ok(bytes)
    }
}

#[expect(
    private_bounds,
    private_interfaces,
    reason = "irrelevant due to being a sealed trait"
)]
impl sealed::Sealed for Rfc3339 {
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        const {
            assert!(
                V::SUPPLIES_DATE && V::SUPPLIES_TIME && V::SUPPLIES_OFFSET,
                "Rfc3339 requires date, time, and offset components, but not all can be provided \
                 by this type"
            );
        }

        let offset_hour = value.offset_hour(state);
        let mut bytes = 0;

        if !(0..10_000).contains(&value.calendar_year(state).get()) {
            crate::hint::cold_path();
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset_hour.get().unsigned_abs() > 23 {
            crate::hint::cold_path();
            return Err(error::Format::InvalidComponent("offset_hour"));
        }
        if value.offset_second(state).get() != 0 {
            crate::hint::cold_path();
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        // Safety: Years outside this range were rejected above.
        bytes += try_likely_ok!(format_four_digits_pad_zero(output, unsafe {
            ru16::new_unchecked(value.calendar_year(state).get().cast_unsigned().truncate())
        }));
        bytes += try_likely_ok!(write(output, "-"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `month` is guaranteed to be in the range `1..=12`.
            unsafe { ru8::new_unchecked(u8::from(value.month(state))) },
            Padding::Zero,
        ));
        bytes += try_likely_ok!(write(output, "-"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.day(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, "T"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.hour(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, ":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.minute(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, ":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.second(state).expand(),
            Padding::Zero
        ));

        let nanos = value.nanosecond(state);
        if nanos.get() != 0 {
            bytes += try_likely_ok!(write(output, "."));
            try_likely_ok!(write(
                output,
                &num_fmt::truncated_subsecond_from_nanos(nanos)
            ));
        }

        if value.offset_is_utc(state) {
            bytes += try_likely_ok!(write(output, "Z"));
            return Ok(bytes);
        }

        bytes += try_likely_ok!(write_if_else(
            output,
            value.offset_is_negative(state),
            "-",
            "+"
        ));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `OffsetHours` is guaranteed to be in the range `-23..=23`, so the absolute
            // value is guaranteed to be in the range `0..=23`.
            unsafe { ru8::new_unchecked(offset_hour.get().unsigned_abs()) },
            Padding::Zero,
        ));
        bytes += try_likely_ok!(write(output, ":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `OffsetMinutes` is guaranteed to be in the range `-59..=59`, so the absolute
            // value is guaranteed to be in the range `0..=59`.
            unsafe { ru8::new_unchecked(value.offset_minute(state).get().unsigned_abs()) },
            Padding::Zero,
        ));

        Ok(bytes)
    }
}

impl<const CONFIG: EncodedConfig> sealed::Sealed for Iso8601<CONFIG> {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    #[inline]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        let mut bytes = 0;

        const {
            assert!(
                !Self::FORMAT_DATE || V::SUPPLIES_DATE,
                "this Iso8601 configuration formats date components, but this type cannot provide \
                 them"
            );
            assert!(
                !Self::FORMAT_TIME || V::SUPPLIES_TIME,
                "this Iso8601 configuration formats time components, but this type cannot provide \
                 them"
            );
            assert!(
                !Self::FORMAT_OFFSET || V::SUPPLIES_OFFSET,
                "this Iso8601 configuration formats offset components, but this type cannot \
                 provide them"
            );
            assert!(
                Self::FORMAT_DATE || Self::FORMAT_TIME || Self::FORMAT_OFFSET,
                "this Iso8601 configuration does not format any components"
            );
        }

        if Self::FORMAT_DATE {
            bytes += try_likely_ok!(iso8601::format_date::<_, CONFIG>(output, value, state));
        }
        if Self::FORMAT_TIME {
            bytes += try_likely_ok!(iso8601::format_time::<_, CONFIG>(output, value, state));
        }
        if Self::FORMAT_OFFSET {
            bytes += try_likely_ok!(iso8601::format_offset::<_, CONFIG>(output, value, state));
        }

        Ok(bytes)
    }
}
