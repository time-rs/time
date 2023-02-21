//! Treat an [`OffsetDateTime`] and [`PrimitiveDateTime`] as a [Unix timestamp] for the purposes of serde.
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

use crate::OffsetDateTime;
use crate::PrimitiveDateTime;

/// Serialize an [`OffsetDateTime`] and [`PrimitiveDateTime`] as its Unix timestamp
/// 
/// Also works with [`Option<OffsetDateTime>`], and [`Option<PrimitiveDateTime>`].
pub fn serialize<S: Serializer, T>(
    datetime: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> 
    where for <'a> __private::Timestamp<&'a T> : Serialize {
    Serialize::serialize(&__private::Timestamp(datetime),serializer)
}

/// Deserialize an `OffsetDateTime` from its Unix timestamp
/// 
/// Also works with [`Option<OffsetDateTime>`], and [`Option<PrimitiveDateTime>`].
pub fn deserialize<'a, D: Deserializer<'a>, T>(deserializer: D) -> Result<T, D::Error> 
    where __private::Timestamp<T> : Deserialize<'a> {
    let t = __private::Timestamp::deserialize(deserializer)?;
    Ok(t.0)
}

fn drop_offset(v : OffsetDateTime) -> PrimitiveDateTime {
    v.date().with_time(v.time())
}

#[doc(hidden)]
mod __private {
    use super::*;

    pub struct Timestamp<T>(pub(super) T);

    impl<'de> Deserialize<'de> for Timestamp<OffsetDateTime> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
            let d = 
                OffsetDateTime::from_unix_timestamp(<_>::deserialize(deserializer)?)
                    .map_err(|err| de::Error::invalid_value(de::Unexpected::Signed(err.value), &err))?;
            Ok(Timestamp(d))
        }
    }

    impl<'de> Deserialize<'de> for Timestamp<Option<OffsetDateTime>> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
            let d = Option::deserialize(deserializer)?
                .map(OffsetDateTime::from_unix_timestamp)
                .transpose()
                .map_err(|err| de::Error::invalid_value(de::Unexpected::Signed(err.value), &err))?;

            Ok(Timestamp(d))
        }
    }

    impl<'de> Deserialize<'de> for Timestamp<Vec<OffsetDateTime>> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
            let d = Vec::deserialize(deserializer)?
                .into_iter()
                .map(OffsetDateTime::from_unix_timestamp)
                .collect::<Result<_,_>>()
                .map_err(|err| de::Error::invalid_value(de::Unexpected::Signed(err.value), &err))?;

            Ok(Timestamp(d))
        }
    }

    impl<'a> Serialize for Timestamp<&'a OffsetDateTime> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            self.0.unix_timestamp().serialize(serializer)
        }
    }

    impl<'a> Serialize for Timestamp<&'a Option<OffsetDateTime>> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            self.0.map(OffsetDateTime::unix_timestamp).serialize(serializer)
        }
    }

    impl<'a> Serialize for Timestamp<&'a [OffsetDateTime]> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for t in self.0 {
                serde::ser::SerializeSeq::serialize_element(&mut seq, &Timestamp(t))?;
            }
            serde::ser::SerializeSeq::end(seq)
        }
    }

    impl<'a> Serialize for Timestamp<&'a Vec<OffsetDateTime>> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            Timestamp(&self.0[..]).serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Timestamp<PrimitiveDateTime> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
            let t : Timestamp<OffsetDateTime> = <_>::deserialize(deserializer)?;
            Ok(Timestamp(t.0.date().with_time(t.0.time())))
        }
    }

    impl<'de> Deserialize<'de> for Timestamp<Option<PrimitiveDateTime>> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
            let t : Timestamp<Option<OffsetDateTime>> = <_>::deserialize(deserializer)?;

            Ok(Timestamp(t.0.map(|t| t.date().with_time(t.time()))))
        }
    }

    impl<'de> Deserialize<'de> for Timestamp<Vec<PrimitiveDateTime>> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
            let t : Timestamp<Vec<OffsetDateTime>> = <_>::deserialize(deserializer)?;

            Ok(Timestamp(t.0.into_iter().map(|t| t.date().with_time(t.time())).collect()))
        }
    }

    impl<'a> Serialize for Timestamp<&'a PrimitiveDateTime> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            Timestamp(&self.0.assume_utc()).serialize(serializer)
        }
    }

    impl<'a> Serialize for Timestamp<&'a Option<PrimitiveDateTime>> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            Timestamp(&self.0.map(|t| t.assume_utc())).serialize(serializer)
        }
    }

    impl<'a> Serialize for Timestamp<&'a [PrimitiveDateTime]> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for t in self.0 {
                serde::ser::SerializeSeq::serialize_element(&mut seq, &Timestamp(t))?;
            }
            serde::ser::SerializeSeq::end(seq)
        }
    }

    impl<'a> Serialize for Timestamp<&'a Vec<PrimitiveDateTime>> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            Timestamp(&self.0[..]).serialize(serializer)
        }
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

    /// Serialize as a Unix timestamp in milliseconds
    pub fn serialize<S: Serializer, T>(
        datetime: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> 
        where for <'a> private::TimestampMillis<&'a T> : Serialize {
        Serialize::serialize(&private::TimestampMillis(datetime),serializer)
    }

    /// Deserialize as a Unix timestamp in milliseconds
    pub fn deserialize<'a, D: Deserializer<'a>, T>(deserializer: D) -> Result<T, D::Error> 
        where private::TimestampMillis<T> : Deserialize<'a> {
        let t = private::TimestampMillis::deserialize(deserializer)?;
        Ok(t.0)
    }

    #[doc(hidden)]
    mod private {
        use super::*;

        fn from_i64<E : serde::de::Error>(v : i64) -> Result<OffsetDateTime,E> {
            let seconds = v / 1000;
            let millis = v % 1000;

            let d = 
                OffsetDateTime::from_unix_timestamp(seconds)
                    .map_err(|err| de::Error::invalid_value(de::Unexpected::Signed(err.value), &err))?
                    + crate::Duration::milliseconds(millis);
            Ok(d)
        }

        fn to_i64(v : OffsetDateTime) -> i64 {
            let seconds = v.unix_timestamp() * 1000;
            seconds + v.millisecond() as i64
        }

        pub struct TimestampMillis<T>(pub(super) T);

        impl<'de> Deserialize<'de> for TimestampMillis<OffsetDateTime> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                let timestamp : i64 = <_>::deserialize(deserializer)?;
                
                Ok(TimestampMillis(from_i64(timestamp)?))
            }
        }

        impl<'de> Deserialize<'de> for TimestampMillis<Option<OffsetDateTime>> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                let d = Option::deserialize(deserializer)?
                    .map(from_i64)
                    .transpose()?;

                Ok(TimestampMillis(d))
            }
        }

        impl<'de> Deserialize<'de> for TimestampMillis<Vec<OffsetDateTime>> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                let t = Vec::deserialize(deserializer)?;
    
                Ok(TimestampMillis(t.into_iter().map(from_i64).collect::<Result<Vec<_>,_>>()?))
            }
        }

        impl<'a> Serialize for TimestampMillis<&'a OffsetDateTime> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                to_i64(*self.0).serialize(serializer)
            }
        }

        impl<'a> Serialize for TimestampMillis<&'a Option<OffsetDateTime>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                self.0.map(to_i64).serialize(serializer)
            }
        }

        impl<'a> Serialize for TimestampMillis<&'a [OffsetDateTime]> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
                for t in self.0 {
                    serde::ser::SerializeSeq::serialize_element(&mut seq, &TimestampMillis(t))?;
                }
                serde::ser::SerializeSeq::end(seq)
            }
        }
    
        impl<'a> Serialize for TimestampMillis<&'a Vec<OffsetDateTime>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                    TimestampMillis(&self.0[..]).serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for TimestampMillis<PrimitiveDateTime> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                let t : TimestampMillis<OffsetDateTime> = <_>::deserialize(deserializer)?;
                Ok(TimestampMillis(t.0.date().with_time(t.0.time())))
            }
        }
    
        impl<'de> Deserialize<'de> for TimestampMillis<Option<PrimitiveDateTime>> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                let t : TimestampMillis<Option<OffsetDateTime>> = <_>::deserialize(deserializer)?;
    
                Ok(TimestampMillis(t.0.map(|t| t.date().with_time(t.time()))))
            }
        }

        impl<'de> Deserialize<'de> for TimestampMillis<Vec<PrimitiveDateTime>> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                let t : TimestampMillis<Vec<OffsetDateTime>> = <_>::deserialize(deserializer)?;
    
                Ok(TimestampMillis(t.0.into_iter().map(drop_offset).collect()))
            }
        }
    
        impl<'a> Serialize for TimestampMillis<&'a PrimitiveDateTime> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                TimestampMillis(&self.0.assume_utc()).serialize(serializer)
            }
        }
    
        impl<'a> Serialize for TimestampMillis<&'a Option<PrimitiveDateTime>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                TimestampMillis(&self.0.map(|t| t.assume_utc())).serialize(serializer)
            }
        }

        impl<'a> Serialize for TimestampMillis<&'a [PrimitiveDateTime]> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
                for t in self.0 {
                    serde::ser::SerializeSeq::serialize_element(&mut seq, &TimestampMillis(t))?;
                }
                serde::ser::SerializeSeq::end(seq)
            }
        }
    
        impl<'a> Serialize for TimestampMillis<&'a Vec<PrimitiveDateTime>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                    TimestampMillis(&self.0[..]).serialize(serializer)
            }
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
