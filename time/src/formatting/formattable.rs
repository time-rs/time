//! A trait that can be used to format an item from its components.

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Deref;
use std::io;

use deranged::{RangedU8, RangedU16};
use num_conv::prelude::*;

use crate::format_description::modifier::Padding;
use crate::format_description::well_known::iso8601::EncodedConfig;
use crate::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use crate::format_description::{BorrowedFormatItem, OwnedFormatItem};
use crate::formatting::{
    ComponentProvider, MONTH_NAMES, WEEKDAY_NAMES, format_component, format_four_digits_pad_zero,
    format_two_digits, iso8601, write, write_if_else,
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

    /// Format the item using a format description, the intended output, and the various components.
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    pub trait Sealed {
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
            let mut buf = Vec::new();
            try_likely_ok!(self.format_into(&mut buf, value, state));
            Ok(String::from_utf8_lossy(&buf).into_owned())
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
            Self::Literal(literal) => try_likely_ok!(write(output, literal)),
            Self::Component(component) => {
                try_likely_ok!(format_component(output, component, value, state))
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
            Self::Literal(literal) => Ok(try_likely_ok!(write(output, literal))),
            Self::Component(component) => format_component(output, *component, value, state),
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
        bytes += try_likely_ok!(write(output, b", "));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.day(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, b" "));
        // Safety: All month names are at least 3 bytes long.
        bytes += try_likely_ok!(write(output, unsafe {
            MONTH_NAMES[u8::from(value.month(state)).extend::<usize>() - 1].get_unchecked(..3)
        }));
        bytes += try_likely_ok!(write(output, b" "));
        // Safety: Years with five or more digits were rejected above. Likewise with negative years.
        bytes += try_likely_ok!(format_four_digits_pad_zero(output, unsafe {
            RangedU16::new_unchecked(value.calendar_year(state).get().cast_unsigned().truncate())
        }));
        bytes += try_likely_ok!(write(output, b" "));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.hour(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, b":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.minute(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, b":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.second(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, b" "));
        bytes += try_likely_ok!(write_if_else(
            output,
            value.offset_is_negative(state),
            b"-",
            b"+"
        ));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `OffsetHours` is guaranteed to be in the range `-25..=25`, so the absolute
            // value is guaranteed to be in the range `0..=25`.
            unsafe { RangedU8::new_unchecked(value.offset_hour(state).get().unsigned_abs()) },
            Padding::Zero,
        ));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `OffsetMinutes` is guaranteed to be in the range `-59..=59`, so the absolute
            // value is guaranteed to be in the range `0..=59`.
            unsafe { RangedU8::new_unchecked(value.offset_minute(state).get().unsigned_abs()) },
            Padding::Zero,
        ));

        Ok(bytes)
    }

    #[inline]
    fn format<V>(&self, value: &V, state: &mut V::State) -> Result<String, error::Format>
    where
        V: ComponentProvider,
    {
        let mut buf = Vec::with_capacity(31);
        try_likely_ok!(self.format_into(&mut buf, value, state));
        // Safety: All components output are ASCII.
        Ok(unsafe { String::from_utf8_unchecked(buf) })
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
            RangedU16::new_unchecked(value.calendar_year(state).get().cast_unsigned().truncate())
        }));
        bytes += try_likely_ok!(write(output, b"-"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `month` is guaranteed to be in the range `1..=12`.
            unsafe { RangedU8::new_unchecked(u8::from(value.month(state))) },
            Padding::Zero,
        ));
        bytes += try_likely_ok!(write(output, b"-"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.day(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, b"T"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.hour(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, b":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.minute(state).expand(),
            Padding::Zero
        ));
        bytes += try_likely_ok!(write(output, b":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            value.second(state).expand(),
            Padding::Zero
        ));

        let nanos = value.nanosecond(state);
        if nanos.get() != 0 {
            bytes += try_likely_ok!(write(output, b"."));
            try_likely_ok!(write(
                output,
                num_fmt::truncated_subsecond_from_nanos(nanos).as_bytes(),
            ));
        }

        if value.offset_is_utc(state) {
            bytes += try_likely_ok!(write(output, b"Z"));
            return Ok(bytes);
        }

        bytes += try_likely_ok!(write_if_else(
            output,
            value.offset_is_negative(state),
            b"-",
            b"+"
        ));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `OffsetHours` is guaranteed to be in the range `-23..=23`, so the absolute
            // value is guaranteed to be in the range `0..=23`.
            unsafe { RangedU8::new_unchecked(offset_hour.get().unsigned_abs()) },
            Padding::Zero,
        ));
        bytes += try_likely_ok!(write(output, b":"));
        bytes += try_likely_ok!(format_two_digits(
            output,
            // Safety: `OffsetMinutes` is guaranteed to be in the range `-59..=59`, so the absolute
            // value is guaranteed to be in the range `0..=59`.
            unsafe { RangedU8::new_unchecked(value.offset_minute(state).get().unsigned_abs()) },
            Padding::Zero,
        ));

        Ok(bytes)
    }

    fn format<V>(&self, value: &V, state: &mut V::State) -> Result<String, error::Format>
    where
        V: ComponentProvider,
    {
        let mut buf = Vec::with_capacity(35);
        try_likely_ok!(self.format_into(&mut buf, value, state));
        // Safety: All components output are ASCII.
        Ok(unsafe { String::from_utf8_unchecked(buf) })
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
