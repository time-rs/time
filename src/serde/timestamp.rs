//! De/serialize [`OffsetDateTime`] from/to [Unix timestamps](https://en.wikipedia.org/wiki/Unix_time).
//!
//! Use this module in combination with [serde's with-annotation](https://serde.rs/field-attrs.html#with).
//!
//! Note that the timestamp represenatioon lacks [`UtcOffset`],
//! which is being lost on serialization and assumed 0 on deserialization.
//!
//! # Examples
//!
//! ```
//! # #[cfg(feature = "_serde_json")] {
//! # use serde::{Deserialize, Serialize};
//! # use _serde_json as serde_json;
//! # use serde_json::json;
//! use time::serde::timestamp;
//! # use time::{date, time, OffsetDateTime};
//!
//! #[derive(Deserialize, Serialize)]
//! struct S {
//!     #[serde(with = "timestamp")]
//!     datetime: OffsetDateTime,
//! }
//!
//! # fn test() -> Result<(), serde_json::Error> {
//! let s = S {
//!     datetime: date!(1970-01-01).with_time(time!(1:00)).assume_utc(),
//! };
//! let v = json!({ "datetime": 3600 });
//! assert_eq!(s.datetime, serde_json::from_value::<S>(v.clone())?.datetime);
//! assert_eq!(v, serde_json::to_value(&s)?);
//! # Ok(())
//! # }
//! # test().unwrap();
//! # }
//! ```
//!
//! [`OffsetDateTime`]: crate::OffsetDateTime
//! [`UtcOffset`]: crate::UtcOffset

use crate::OffsetDateTime;
use core::fmt;
use serde::{
    de::{self, Visitor},
    Deserializer, Serializer,
};

/// Fullfills the requirements for [serde's deserialize_with-annotation](https://serde.rs/field-attrs.html#deserialize_with).
///
/// Prefer using the parent module instead for brevity.
#[allow(single_use_lifetimes)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    /// A Visitor that deserializes a OffsetDateTime from a Unix timestamp
    struct OffsetDateTimeVisitor;
    impl Visitor<'_> for OffsetDateTimeVisitor {
        type Value = OffsetDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a Unix timestamp")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Self::Value::from_unix_timestamp(value))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Self::Value::from_unix_timestamp(value as i64))
        }
    }

    deserializer.deserialize_i64(OffsetDateTimeVisitor)
}

/// Fullfills the requirements for [serde's serialize_with-annotation](https://serde.rs/field-attrs.html#serialize_with).
///
/// Prefer using the parent module instead for brevity.
pub fn serialize<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(datetime.timestamp())
}

/// De/serialize [`Option`]`<`[`OffsetDateTime`]`>` from/to [Unix timestamps](https://en.wikipedia.org/wiki/Unix_time).
///
/// Use this module in combination with [serde's with-annotation](https://serde.rs/field-attrs.html#with).
///
/// Note that the timestamp represenatioon lacks [`UtcOffset`],
/// which is being lost on serialization and assumed 0 on deserialization.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "_serde_json")] {
/// # use serde::{Deserialize, Serialize};
/// # use _serde_json as serde_json;
/// # use serde_json::json;
/// use time::serde::timestamp;
/// # use time::{date, time, OffsetDateTime};
///
/// #[derive(Deserialize, Serialize)]
/// struct S {
///     #[serde(with = "timestamp::option")]
///     datetime: Option<OffsetDateTime>,
/// }
///
/// # fn test() -> Result<(), serde_json::Error> {
/// let s = S {
///     datetime: Some(date!(1970-01-01).with_time(time!(1:00)).assume_utc()),
/// };
/// let v = json!({ "datetime": 3600 });
/// assert_eq!(s.datetime, serde_json::from_value::<S>(v.clone())?.datetime);
/// assert_eq!(v, serde_json::to_value(&s)?);
///
/// let s = S { datetime: None };
/// let v = json!({ "datetime": null });
/// assert_eq!(s.datetime, serde_json::from_value::<S>(v.clone())?.datetime);
/// assert_eq!(v, serde_json::to_value(&s)?);
/// # Ok(())
/// # }
/// # test().unwrap();
/// # }
/// ```
///
/// [`OffsetDateTime`]: crate::OffsetDateTime
/// [`UtcOffset`]: crate::UtcOffset
pub mod option {
    use super::*;

    /// Fullfills the requirements for [serde's deserialize_with-annotation](https://serde.rs/field-attrs.html#deserialize_with).
    ///
    /// Prefer using the parent module instead for brevity.
    #[allow(single_use_lifetimes)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// A Visitor that deserializes an optional OffsetDateTime from a Unix timestamp
        struct OffsetDateTimeOptionVisitor;
        impl<'de> Visitor<'de> for OffsetDateTimeOptionVisitor {
            type Value = Option<OffsetDateTime>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an optional Unix timestamp")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                super::deserialize(deserializer).map(Some)
            }
        }

        deserializer.deserialize_option(OffsetDateTimeOptionVisitor)
    }

    /// Fullfills the requirements for [serde's serialize_with-annotation](https://serde.rs/field-attrs.html#serialize_with).
    ///
    /// Prefer using the parent module instead for brevity.
    pub fn serialize<S>(option: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *option {
            Some(ref datetime) => serializer.serialize_some(&datetime.timestamp()),
            None => serializer.serialize_none(),
        }
    }
}
