//! Treat an [`OffsetDateTime`] and [`PrimitiveDateTime`] as a [Unix timestamp] for the purposes of
//! serde.
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! When deserializing, the offset is assumed to be UTC.
//!
//! Also works with [`Option<OffsetDateTime>`], and [`Option<PrimitiveDateTime>`].
//!
//! [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
//! [with]: https://serde.rs/field-attrs.html#with

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::{AsWellKnown, FromWellKnown};
use crate::{OffsetDateTime, PrimitiveDateTime};

/// Serialize an [`OffsetDateTime`] and [`PrimitiveDateTime`] as its Unix timestamp
///
/// Also works with [`Option<OffsetDateTime>`], and [`Option<PrimitiveDateTime>`].
#[inline(always)]
pub fn serialize<S: Serializer, T>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsWellKnown<Timestamp>,
{
    t.serialize_from_wellknown(serializer)
}

/// Deserialize an `OffsetDateTime` from its Unix timestamp
///
/// Also works with [`Option<OffsetDateTime>`], and [`Option<PrimitiveDateTime>`].
#[inline(always)]
pub fn deserialize<'a, D: Deserializer<'a>, T>(deserializer: D) -> Result<T, D::Error>
where
    T: FromWellKnown<Timestamp>,
{
    T::deserialize_from_well_known(deserializer)
}

pub struct Timestamp;

impl AsWellKnown<Timestamp> for OffsetDateTime {
    type IntoWellKnownError = std::convert::Infallible;

    type WellKnownSer<'s> = i64 where Self: 's;

    fn as_well_known<'s>(&'s self) -> Result<Self::WellKnownSer<'s>, Self::IntoWellKnownError> {
        Ok(self.unix_timestamp())
    }
}

impl FromWellKnown<Timestamp> for OffsetDateTime {
    type FromWellKnownError = crate::error::ComponentRange;

    type WellKnownDeser<'de> = i64;

    fn fmt_err<E: serde::de::Error>(e: Self::FromWellKnownError) -> E {
        E::invalid_value(serde::de::Unexpected::Signed(e.value), &e)
    }

    fn from_well_known<'de>(
        wk: Self::WellKnownDeser<'de>,
    ) -> Result<Self, Self::FromWellKnownError> {
        OffsetDateTime::from_unix_timestamp(wk)
    }
}

impl AsWellKnown<Timestamp> for PrimitiveDateTime {
    type IntoWellKnownError = std::convert::Infallible;

    type WellKnownSer<'s> = i64 where Self: 's;

    fn as_well_known<'s>(&'s self) -> Result<Self::WellKnownSer<'s>, Self::IntoWellKnownError> {
        Ok(self.assume_utc().unix_timestamp())
    }
}

impl FromWellKnown<Timestamp> for PrimitiveDateTime {
    type FromWellKnownError = crate::error::ComponentRange;

    type WellKnownDeser<'de> = i64;

    fn from_well_known<'de>(
        wk: Self::WellKnownDeser<'de>,
    ) -> Result<Self, Self::FromWellKnownError> {
        OffsetDateTime::from_unix_timestamp(wk).map(|t| t.date().with_time(t.time()))
    }
}

// Treat an [`OffsetDateTime`] as a [Unix timestamp (milliseconds)] for the purposes of serde.
//
// Use this module in combination with serde's [`#[with]`][with] attribute.
//
// When deserializing, the offset is assumed to be UTC.
//
// [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
// [with]: https://serde.rs/field-attrs.html#with
pub mod millis {
    use super::*;

    /// Serialize an [`OffsetDateTime`] and [`PrimitiveDateTime`] as its Unix timestamp in
    /// milliseconds
    ///
    /// Also works with [`Option<OffsetDateTime>`], and [`Option<PrimitiveDateTime>`].
    #[inline(always)]
    pub fn serialize<S: Serializer, T>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsWellKnown<TimestampMillis>,
    {
        t.serialize_from_wellknown(serializer)
    }

    /// Deserialize an `OffsetDateTime` from its Unix timestamp in milliseconds
    ///
    /// Also works with [`Option<OffsetDateTime>`], and [`Option<PrimitiveDateTime>`].
    #[inline(always)]
    pub fn deserialize<'a, D: Deserializer<'a>, T>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromWellKnown<TimestampMillis>,
    {
        T::deserialize_from_well_known(deserializer)
    }

    pub struct TimestampMillis;

    impl AsWellKnown<TimestampMillis> for OffsetDateTime {
        type IntoWellKnownError = std::convert::Infallible;

        type WellKnownSer<'s> = i64 where Self: 's;

        fn as_well_known<'s>(&'s self) -> Result<Self::WellKnownSer<'s>, Self::IntoWellKnownError> {
            Ok((self.unix_timestamp_nanos() / 1_000_000) as i64)
        }
    }

    impl FromWellKnown<TimestampMillis> for OffsetDateTime {
        type FromWellKnownError = crate::error::ComponentRange;

        type WellKnownDeser<'de> = i64;

        fn fmt_err<E: serde::de::Error>(e: Self::FromWellKnownError) -> E {
            E::invalid_value(serde::de::Unexpected::Signed(e.value), &e)
        }

        fn from_well_known<'de>(
            timestamp: Self::WellKnownDeser<'de>,
        ) -> Result<Self, Self::FromWellKnownError> {
            let secs = timestamp / 1_000;
            let millis = timestamp % 1000;

            Ok(OffsetDateTime::from_unix_timestamp(secs)? + crate::Duration::milliseconds(millis))
        }
    }

    impl AsWellKnown<TimestampMillis> for PrimitiveDateTime {
        type IntoWellKnownError = std::convert::Infallible;

        type WellKnownSer<'s> = i64 where Self: 's;

        #[inline]
        fn as_well_known<'s>(&'s self) -> Result<Self::WellKnownSer<'s>, Self::IntoWellKnownError> {
            Ok((self.assume_utc().unix_timestamp_nanos() / 1_000_000) as i64)
        }
    }

    impl FromWellKnown<TimestampMillis> for PrimitiveDateTime {
        type FromWellKnownError = crate::error::ComponentRange;

        type WellKnownDeser<'de> = i64;

        fn from_well_known<'de>(
            wk: Self::WellKnownDeser<'de>,
        ) -> Result<Self, Self::FromWellKnownError> {
            let t = <OffsetDateTime as FromWellKnown<TimestampMillis>>::from_well_known(wk)?;
            Ok(t.date().with_time(t.time()))
        }
    }
}

/// Treat an `Option<OffsetDateTime>` as a [Unix timestamp] for the purposes of
/// serde.
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// When deserializing, the offset is assumed to be UTC.
///
/// [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
/// [with]: https://serde.rs/field-attrs.html#with
#[deprecated]
pub mod option {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    /// Serialize an `Option<OffsetDateTime>` as its Unix timestamp
    #[deprecated]
    pub fn serialize<S: Serializer>(
        option: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        option
            .map(OffsetDateTime::unix_timestamp)
            .serialize(serializer)
    }

    /// Deserialize an `Option<OffsetDateTime>` from its Unix timestamp
    #[deprecated]
    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        Option::deserialize(deserializer)?
            .map(OffsetDateTime::from_unix_timestamp)
            .transpose()
            .map_err(|err| de::Error::invalid_value(de::Unexpected::Signed(err.value), &err))
    }
}
