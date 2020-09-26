//! Differential formats for serde.

// Types with guaranteed stable serde representations. Strings are avoided to
// allow for optimal representations in various binary forms.

#![allow(clippy::missing_docs_in_private_items)]

mod date;
mod duration;
mod offset_date_time;
mod primitive_date_time;
mod time;
pub mod timestamp;
mod utc_offset;
mod weekday;

pub(crate) use self::time::Time;
pub(crate) use date::Date;
pub(crate) use duration::Duration;
pub(crate) use offset_date_time::OffsetDateTime;
pub(crate) use primitive_date_time::PrimitiveDateTime;
pub(crate) use utc_offset::UtcOffset;
pub(crate) use weekday::Weekday;
