//! De/serialize [`OffsetDateTime`] from/to [Unix timestamps](https://en.wikipedia.org/wiki/Unix_time).
//!
//! Use this module in combination with [serde's with-annotation](https://serde.rs/field-attrs.html#with).
//!
//! Note that the timestamp represenatioon lacks [`UtcOffset`],
//! which is being lost on serialization and assumed 0 on deserialization.
//!
//! # Examples
//!
//! ```ignore
//! # #[cfg(feature = "_serde_json")] {
//! # use serde::{Deserialize, Serialize};
//! # use _serde_json as serde_json;
//! # use serde_json::json;
//! use time::serde::timestamp;
//! # use time::{date, time, OffsetDateTime};
//!
//! # #[derive(Debug, PartialEq)]
//! #[derive(Deserialize, Serialize)]
//! struct S {
//!     #[serde(with = "timestamp")]
//!     datetime: OffsetDateTime,
//! }
//!
//! # fn test() -> Result<(), serde_json::Error> {
//! let s = S {
//!     datetime: date!(2019-01-01).midnight().assume_utc(),
//! };
//! let v = json!({ "datetime": 1_546_300_800 });
//! assert_eq!(v, serde_json::to_value(&s)?);
//! assert_eq!(s, serde_json::from_value(v)?);
//! # Ok(())
//! # }
//! # test().unwrap();
//! # }
//! ```
//!
//! [`UtcOffset`]: crate::UtcOffset

use crate::OffsetDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Fullfills the requirements for [serde's serialize_with-annotation](https://serde.rs/field-attrs.html#serialize_with).
///
/// Prefer using the parent module instead for brevity.
#[allow(single_use_lifetimes)]
pub fn serialize<S: Serializer>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    #[derive(Serialize)]
    #[serde(transparent)]
    struct Wrapper<'a>(&'a i64);

    Wrapper(&datetime.timestamp()).serialize(serializer)
}

/// Fullfills the requirements for [serde's deserialize_with-annotation](https://serde.rs/field-attrs.html#deserialize_with).
///
/// Prefer using the parent module instead for brevity.
#[allow(single_use_lifetimes)]
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<OffsetDateTime, D::Error> {
    #[derive(Deserialize)]
    #[serde(transparent)]
    struct Wrapper(i64);

    Wrapper::deserialize(deserializer)
        .map(|Wrapper(timestamp)| timestamp)
        .map(OffsetDateTime::from_unix_timestamp)
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
/// ```ignore
/// # #[cfg(feature = "_serde_json")] {
/// # use serde::{Deserialize, Serialize};
/// # use _serde_json as serde_json;
/// # use serde_json::json;
/// use time::serde::timestamp;
/// # use time::{date, time, OffsetDateTime};
///
/// # #[derive(Debug, PartialEq)]
/// #[derive(Deserialize, Serialize)]
/// struct S {
///     #[serde(default, with = "timestamp::option")]
///     datetime: Option<OffsetDateTime>,
/// }
///
/// # fn test() -> Result<(), serde_json::Error> {
/// let s_some = S {
///     datetime: Some(date!(2019-01-01).midnight().assume_utc()),
/// };
/// let v_some = json!({ "datetime": 1_546_300_800 });
/// assert_eq!(v_some, serde_json::to_value(&s_some)?);
/// assert_eq!(s_some, serde_json::from_value(v_some)?);
///
/// let s_none = S { datetime: None };
/// let v_null = json!({ "datetime": null });
/// assert_eq!(v_null, serde_json::to_value(&s_none)?);
/// assert_eq!(s_none, serde_json::from_value(v_null)?);
///
/// let v_missing = json!({});
/// assert_eq!(s_none, serde_json::from_value(v_missing)?);
/// # Ok(())
/// # }
/// # test().unwrap();
/// # }
/// ```
///
/// [`UtcOffset`]: crate::UtcOffset
pub mod option {
    use super::*;

    /// Fullfills the requirements for [serde's serialize_with-annotation](https://serde.rs/field-attrs.html#serialize_with).
    ///
    /// Prefer using the parent module instead for brevity.
    #[allow(single_use_lifetimes)]
    pub fn serialize<S: Serializer>(
        option: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Wrapper<'a>(#[serde(with = "super")] &'a OffsetDateTime);

        option.as_ref().map(Wrapper).serialize(serializer)
    }

    /// Fullfills the requirements for [serde's deserialize_with-annotation](https://serde.rs/field-attrs.html#deserialize_with).
    ///
    /// Prefer using the parent module instead for brevity.
    #[allow(single_use_lifetimes)]
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Wrapper(#[serde(with = "super")] OffsetDateTime);

        Option::deserialize(deserializer).map(|opt| opt.map(|Wrapper(datetime)| datetime))
    }
}
