//! Types with guaranteed stable serde representations.
//!
//! This allows for the ability to change the internal structure of a type while
//! maintaining backwards compatibility.
//!
//! Strings are avoided where possible to allow for optimal representations in
//! various binary forms.

#![allow(clippy::missing_docs_in_private_items)]

// OffsetDateTime is in the primitive_date_time module.

mod date;
mod duration;
mod primitive_date_time;
mod sign;
mod time;
mod utc_offset;
mod weekday;

/// De/serialize [`OffsetDateTime`] from/to [Unix timestamps](https://en.wikipedia.org/wiki/Unix_time).
///
/// Use this module in combination with [serde's with-annotation](https://serde.rs/field-attrs.html#with).
///
/// Note that the timestamp represenatioon lacks [`UtcOffset`],
/// which is being lost on serialization and assumed 0 on deserialization.
///
/// # Examples
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use serde_json::json;
/// use time::serde::timestamp;
/// # use time::{date, time, OffsetDateTime};
///
/// #[derive(Deserialize, Serialize)]
/// struct S {
///     #[serde(with = "timestamp")]
///     datetime: OffsetDateTime,
/// }
///
/// let s = S {
///     datetime: date!(1970-01-01).with_time(time!(1:00)).assume_utc(),
/// };
/// let v = json!({ "datetime": 3600 });
/// assert_eq!(s.datetime, serde_json::from_value::<S>(v.clone())?.datetime);
/// assert_eq!(v, serde_json::to_value(&s)?);
/// # Ok::<(), serde_json::Error>(())
/// ```
pub mod timestamp;

pub(crate) use self::time::Time;
pub(crate) use date::Date;
pub(crate) use duration::Duration;
pub(crate) use primitive_date_time::PrimitiveDateTime;
#[allow(deprecated)]
pub(crate) use sign::Sign;
pub(crate) use utc_offset::UtcOffset;
pub(crate) use weekday::Weekday;
