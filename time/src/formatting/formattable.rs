//! A trait that can be used to format an item from its components.

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Deref;
use std::io;

use num_conv::prelude::*;

use crate::error;
use crate::format_description::well_known::iso8601::EncodedConfig;
use crate::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use crate::format_description::{BorrowedFormatItem, OwnedFormatItem};
use crate::formatting::{
    ComponentProvider, MONTH_NAMES, WEEKDAY_NAMES, format_component, format_number_pad_zero,
    iso8601, write,
};

/// A type that describes a format.
///
/// Implementors of [`Formattable`] are [format descriptions](crate::format_description).
///
/// [`Date::format`] and [`Time::format`] each use a format description to generate
/// a String from their data. See the respective methods for usage examples.
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
            self.format_into(&mut buf, value, state)?;
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
            Self::Literal(literal) => write(output, literal)?,
            Self::Component(component) => format_component(output, component, value, state)?,
            Self::Compound(items) => (*items).format_into(output, value, state)?,
            Self::Optional(item) => (*item).format_into(output, value, state)?,
            Self::First(items) => match items {
                [] => 0,
                [item, ..] => (*item).format_into(output, value, state)?,
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
            bytes += (*item).format_into(output, value, state)?;
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
            Self::Literal(literal) => Ok(write(output, literal)?),
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
            bytes += item.format_into(output, value, state)?;
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

impl sealed::Sealed for Rfc2822 {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        if !(V::SUPPLIES_DATE && V::SUPPLIES_TIME && V::SUPPLIES_OFFSET) {
            return Err(error::Format::InsufficientTypeInformation);
        }

        let mut bytes = 0;

        if value.calendar_year(state) < 1900 {
            return Err(error::Format::InvalidComponent("year"));
        }
        if value.offset_second(state) != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        bytes += write(
            output,
            &WEEKDAY_NAMES[value
                .weekday(state)
                .number_days_from_monday()
                .extend::<usize>()][..3],
        )?;
        bytes += write(output, b", ")?;
        bytes += format_number_pad_zero::<2>(output, value.day(state))?;
        bytes += write(output, b" ")?;
        bytes += write(
            output,
            &MONTH_NAMES[u8::from(value.month(state)).extend::<usize>() - 1][..3],
        )?;
        bytes += write(output, b" ")?;
        bytes += format_number_pad_zero::<4>(output, value.calendar_year(state).cast_unsigned())?;
        bytes += write(output, b" ")?;
        bytes += format_number_pad_zero::<2>(output, value.hour(state))?;
        bytes += write(output, b":")?;
        bytes += format_number_pad_zero::<2>(output, value.minute(state))?;
        bytes += write(output, b":")?;
        bytes += format_number_pad_zero::<2>(output, value.second(state))?;
        bytes += write(output, b" ")?;
        bytes += write(
            output,
            if value.offset_is_negative(state) {
                b"-"
            } else {
                b"+"
            },
        )?;
        bytes += format_number_pad_zero::<2>(output, value.offset_hour(state).unsigned_abs())?;
        bytes += format_number_pad_zero::<2>(output, value.offset_minute(state).unsigned_abs())?;

        Ok(bytes)
    }
}

impl sealed::Sealed for Rfc3339 {
    #[expect(
        private_bounds,
        private_interfaces,
        reason = "irrelevant due to being a sealed trait"
    )]
    fn format_into<V>(
        &self,
        output: &mut (impl io::Write + ?Sized),
        value: &V,
        state: &mut V::State,
    ) -> Result<usize, error::Format>
    where
        V: ComponentProvider,
    {
        if !(V::SUPPLIES_DATE && V::SUPPLIES_TIME && V::SUPPLIES_OFFSET) {
            return Err(error::Format::InsufficientTypeInformation);
        }

        let offset_hour = value.offset_hour(state);
        let mut bytes = 0;

        if !(0..10_000).contains(&value.calendar_year(state)) {
            return Err(error::Format::InvalidComponent("year"));
        }
        if offset_hour.unsigned_abs() > 23 {
            return Err(error::Format::InvalidComponent("offset_hour"));
        }
        if value.offset_second(state) != 0 {
            return Err(error::Format::InvalidComponent("offset_second"));
        }

        bytes += format_number_pad_zero::<4>(output, value.calendar_year(state).cast_unsigned())?;
        bytes += write(output, b"-")?;
        bytes += format_number_pad_zero::<2>(output, u8::from(value.month(state)))?;
        bytes += write(output, b"-")?;
        bytes += format_number_pad_zero::<2>(output, value.day(state))?;
        bytes += write(output, b"T")?;
        bytes += format_number_pad_zero::<2>(output, value.hour(state))?;
        bytes += write(output, b":")?;
        bytes += format_number_pad_zero::<2>(output, value.minute(state))?;
        bytes += write(output, b":")?;
        bytes += format_number_pad_zero::<2>(output, value.second(state))?;

        let nanos = value.nanosecond(state);
        if nanos != 0 {
            bytes += write(output, b".")?;
            bytes += if nanos % 10 != 0 {
                format_number_pad_zero::<9>(output, nanos)
            } else if (nanos / 10) % 10 != 0 {
                format_number_pad_zero::<8>(output, nanos / 10)
            } else if (nanos / 100) % 10 != 0 {
                format_number_pad_zero::<7>(output, nanos / 100)
            } else if (nanos / 1_000) % 10 != 0 {
                format_number_pad_zero::<6>(output, nanos / 1_000)
            } else if (nanos / 10_000) % 10 != 0 {
                format_number_pad_zero::<5>(output, nanos / 10_000)
            } else if (nanos / 100_000) % 10 != 0 {
                format_number_pad_zero::<4>(output, nanos / 100_000)
            } else if (nanos / 1_000_000) % 10 != 0 {
                format_number_pad_zero::<3>(output, nanos / 1_000_000)
            } else if (nanos / 10_000_000) % 10 != 0 {
                format_number_pad_zero::<2>(output, nanos / 10_000_000)
            } else {
                format_number_pad_zero::<1>(output, nanos / 100_000_000)
            }?;
        }

        if value.offset_is_utc(state) {
            bytes += write(output, b"Z")?;
            return Ok(bytes);
        }

        bytes += write(
            output,
            if value.offset_is_negative(state) {
                b"-"
            } else {
                b"+"
            },
        )?;
        bytes += format_number_pad_zero::<2>(output, offset_hour.unsigned_abs())?;
        bytes += write(output, b":")?;
        bytes += format_number_pad_zero::<2>(output, value.offset_minute(state).unsigned_abs())?;

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

        if Self::FORMAT_DATE {
            if !V::SUPPLIES_DATE {
                return Err(error::Format::InsufficientTypeInformation);
            }
            bytes += iso8601::format_date::<_, CONFIG>(output, value, state)?;
        }
        if Self::FORMAT_TIME {
            if !V::SUPPLIES_TIME {
                return Err(error::Format::InsufficientTypeInformation);
            }
            bytes += iso8601::format_time::<_, CONFIG>(output, value, state)?;
        }
        if Self::FORMAT_OFFSET {
            if !V::SUPPLIES_OFFSET {
                return Err(error::Format::InsufficientTypeInformation);
            }
            bytes += iso8601::format_offset::<_, CONFIG>(output, value, state)?;
        }

        if bytes == 0 {
            // The only reason there would be no bytes written is if the format was only for
            // parsing.
            panic!("attempted to format a parsing-only format description");
        }

        Ok(bytes)
    }
}
