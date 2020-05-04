//! Types with guaranteed stable serde representations.
//!
//! Generally speaking, types are able to use the stabilized equivalents, as the
//! internal representation will not change. However, some structs may not have
//! an internal representation guaranteed (yet).

#![allow(clippy::missing_docs_in_private_items)]

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Date {
    year: i32,
    ordinal: u16,
}

impl From<crate::Date> for Date {
    #[inline]
    fn from(original: crate::Date) -> Self {
        Self {
            year: original.year(),
            ordinal: original.ordinal(),
        }
    }
}

impl From<Date> for crate::Date {
    #[inline]
    fn from(original: Date) -> Self {
        crate::internals::Date::from_yo_unchecked(original.year, original.ordinal)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl From<crate::Time> for Time {
    #[inline]
    fn from(original: crate::Time) -> Self {
        Self {
            hour: original.hour,
            minute: original.minute,
            second: original.second,
            nanosecond: original.nanosecond,
        }
    }
}

impl From<Time> for crate::Time {
    #[inline]
    fn from(original: Time) -> Self {
        Self {
            hour: original.hour,
            minute: original.minute,
            second: original.second,
            nanosecond: original.nanosecond,
        }
    }
}
