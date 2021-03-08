//! Differential formats for serde.
// This also includes the serde implementations for all types. This doesn't need to be externally
// documented, though.

// Types with guaranteed stable serde representations. Strings are avoided to allow for optimal
// representations in various binary forms.

pub mod timestamp;

use serde::de::Error as _;
#[cfg(feature = "serde-human-readable")]
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::ComponentRange;
#[cfg(feature = "serde-human-readable")]
use crate::{
    error,
    format_description::{modifier, Component, FormatItem},
};
use crate::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

// region: Date
/// The format used when serializing and deserializing a human-readable `Date`.
#[cfg(feature = "serde-human-readable")]
const DATE_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Year(modifier::Year {
        repr: modifier::YearRepr::Full,
        iso_week_based: false,
        sign_is_mandatory: false,
        padding: modifier::Padding::Zero,
    })),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Month(modifier::Month {
        repr: modifier::MonthRepr::Numerical,
        padding: modifier::Padding::Zero,
    })),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Day(modifier::Day {
        padding: modifier::Padding::Zero,
    })),
];

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&DATE_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `Date`")),
            });
        }

        (self.year(), self.ordinal()).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Date {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "serde-human-readable")]
        if deserializer.is_human_readable() {
            return Self::parse(<&str>::deserialize(deserializer)?, &DATE_FORMAT)
                .map_err(error::Parse::to_invalid_serde_value::<D>);
        }

        let (year, ordinal) = Deserialize::deserialize(deserializer)?;
        Self::from_ordinal_date(year, ordinal).map_err(ComponentRange::to_invalid_serde_value::<D>)
    }
}
// endregion date

// region: Duration
impl Serialize for Duration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.collect_str(&format_args!(
                "{}.{:>09}",
                self.seconds,
                self.nanoseconds.abs()
            ));
        }

        (self.seconds, self.nanoseconds).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Duration {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "serde-human-readable")]
        if deserializer.is_human_readable() {
            let s = <&str>::deserialize(deserializer)?;
            let dot = s.find('.').ok_or_else(|| {
                serde::de::Error::invalid_value(serde::de::Unexpected::Str(s), &"a decimal point")
            })?;
            let (seconds, nanoseconds) = s.split_at(dot); // TODO use case for str_split_once
            let nanoseconds = &nanoseconds[1..]; // strip the leading dot

            let seconds = seconds.parse().map_err(|_| {
                serde::de::Error::invalid_value(serde::de::Unexpected::Str(seconds), &"a number")
            })?;
            let mut nanoseconds = nanoseconds.parse().map_err(|_| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(nanoseconds),
                    &"a number",
                )
            })?;

            if seconds < 0 {
                nanoseconds *= -1;
            }

            return Ok(Self::new(seconds, nanoseconds));
        }

        let (seconds, nanoseconds) = Deserialize::deserialize(deserializer)?;
        Ok(Self::new(seconds, nanoseconds))
    }
}
// endregion Duration

// region: OffsetDateTime
/// The format used when serializing and deserializing a human-readable `OffsetDateTime`.
#[cfg(feature = "serde-human-readable")]
const OFFSET_DATE_TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Compound(DATE_FORMAT),
    FormatItem::Literal(b" "),
    FormatItem::Compound(TIME_FORMAT),
    FormatItem::Literal(b" "),
    FormatItem::Compound(UTC_OFFSET_FORMAT),
];

impl Serialize for OffsetDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&OFFSET_DATE_TIME_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `OffsetDateTime`")),
            });
        }

        (
            self.year(),
            self.ordinal(),
            self.hour(),
            self.minute(),
            self.second(),
            self.nanosecond(),
            self.offset.hours,
            self.offset.minutes,
            self.offset.seconds,
        )
            .serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for OffsetDateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "serde-human-readable")]
        if deserializer.is_human_readable() {
            return Self::parse(<&str>::deserialize(deserializer)?, &OFFSET_DATE_TIME_FORMAT)
                .map_err(error::Parse::to_invalid_serde_value::<D>);
        }

        let (
            year,
            ordinal,
            hour,
            minute,
            second,
            nanosecond,
            offset_hours,
            offset_minutes,
            offset_seconds,
        ) = Deserialize::deserialize(deserializer)?;

        Ok(Date::from_ordinal_date(year, ordinal)
            .map_err(ComponentRange::to_invalid_serde_value::<D>)?
            .with_hms_nano(hour, minute, second, nanosecond)
            .map_err(ComponentRange::to_invalid_serde_value::<D>)?
            .assume_offset(
                UtcOffset::from_hms(offset_hours, offset_minutes, offset_seconds)
                    .map_err(ComponentRange::to_invalid_serde_value::<D>)?,
            ))
    }
}
// endregion OffsetDateTime

// region: PrimitiveDateTime
/// The format used when serializing and deserializing a human-readable `PrimitiveDateTime`.
#[cfg(feature = "serde-human-readable")]
const PRIMITIVE_DATE_TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Compound(DATE_FORMAT),
    FormatItem::Literal(b" "),
    FormatItem::Compound(TIME_FORMAT),
];

impl Serialize for PrimitiveDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&PRIMITIVE_DATE_TIME_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `PrimitiveDateTime`")),
            });
        }

        (
            self.year(),
            self.ordinal(),
            self.hour(),
            self.minute(),
            self.second(),
            self.nanosecond(),
        )
            .serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for PrimitiveDateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "serde-human-readable")]
        if deserializer.is_human_readable() {
            return Self::parse(
                <&str>::deserialize(deserializer)?,
                &PRIMITIVE_DATE_TIME_FORMAT,
            )
            .map_err(error::Parse::to_invalid_serde_value::<D>);
        }

        let (year, ordinal, hour, minute, second, nanosecond) =
            Deserialize::deserialize(deserializer)?;
        Date::from_ordinal_date(year, ordinal)
            .map_err(ComponentRange::to_invalid_serde_value::<D>)?
            .with_hms_nano(hour, minute, second, nanosecond)
            .map_err(ComponentRange::to_invalid_serde_value::<D>)
    }
}
// endregion PrimitiveDateTime

// region: Time
/// The format used when serializing and deserializing a human-readable `Time`.
#[cfg(feature = "serde-human-readable")]
const TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Hour(modifier::Hour {
        padding: modifier::Padding::Zero,
        is_12_hour_clock: false,
    })),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::Minute(modifier::Minute {
        padding: modifier::Padding::Zero,
    })),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::Second(modifier::Second {
        padding: modifier::Padding::Zero,
    })),
    FormatItem::Literal(b"."),
    FormatItem::Component(Component::Subsecond(modifier::Subsecond {
        digits: modifier::SubsecondDigits::OneOrMore,
    })),
];

impl Serialize for Time {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&TIME_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `Time`")),
            });
        }

        (self.hour, self.minute, self.second, self.nanosecond).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Time {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "serde-human-readable")]
        if deserializer.is_human_readable() {
            return Self::parse(<&str>::deserialize(deserializer)?, &TIME_FORMAT)
                .map_err(error::Parse::to_invalid_serde_value::<D>);
        }

        let (hour, minute, second, nanosecond) = Deserialize::deserialize(deserializer)?;
        Self::from_hms_nano(hour, minute, second, nanosecond)
            .map_err(ComponentRange::to_invalid_serde_value::<D>)
    }
}
// endregion Time

// region: UtcOffset
/// The format used when serializing and deserializing a human-readable `UtcOffset`.
#[cfg(feature = "serde-human-readable")]
const UTC_OFFSET_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::OffsetHour(modifier::OffsetHour {
        sign_is_mandatory: true,
        padding: modifier::Padding::Zero,
    })),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::OffsetMinute(modifier::OffsetMinute {
        padding: modifier::Padding::Zero,
    })),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::OffsetSecond(modifier::OffsetSecond {
        padding: modifier::Padding::Zero,
    })),
];

impl Serialize for UtcOffset {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            return serializer.serialize_str(&match self.format(&UTC_OFFSET_FORMAT) {
                Ok(s) => s,
                Err(_) => return Err(S::Error::custom("failed formatting `UtcOffset`")),
            });
        }

        (self.hours, self.minutes, self.seconds).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for UtcOffset {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "serde-human-readable")]
        if deserializer.is_human_readable() {
            return Self::parse(<&str>::deserialize(deserializer)?, &UTC_OFFSET_FORMAT)
                .map_err(error::Parse::to_invalid_serde_value::<D>);
        }

        let (hours, minutes, seconds) = Deserialize::deserialize(deserializer)?;
        Self::from_hms(hours, minutes, seconds).map_err(ComponentRange::to_invalid_serde_value::<D>)
    }
}
// endregion UtcOffset

// region: Weekday
impl Serialize for Weekday {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "serde-human-readable")]
        if serializer.is_human_readable() {
            #[cfg(not(feature = "std"))]
            use alloc::string::ToString;
            return self.to_string().serialize(serializer);
        }

        self.number_from_monday().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Weekday {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "serde-human-readable")]
        if deserializer.is_human_readable() {
            return match <&str>::deserialize(deserializer)? {
                "Monday" => Ok(Self::Monday),
                "Tuesday" => Ok(Self::Tuesday),
                "Wednesday" => Ok(Self::Wednesday),
                "Thursday" => Ok(Self::Thursday),
                "Friday" => Ok(Self::Friday),
                "Saturday" => Ok(Self::Saturday),
                "Sunday" => Ok(Self::Sunday),
                val => Err(D::Error::invalid_value(
                    serde::de::Unexpected::Str(val),
                    &"a day of the week",
                )),
            };
        }

        match u8::deserialize(deserializer)? {
            1 => Ok(Self::Monday),
            2 => Ok(Self::Tuesday),
            3 => Ok(Self::Wednesday),
            4 => Ok(Self::Thursday),
            5 => Ok(Self::Friday),
            6 => Ok(Self::Saturday),
            7 => Ok(Self::Sunday),
            val => Err(D::Error::invalid_value(
                serde::de::Unexpected::Unsigned(val.into()),
                &"a value in the range 1..=7",
            )),
        }
    }
}
// endregion Weekday
