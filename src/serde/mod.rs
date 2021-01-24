//! Differential formats for serde.
// This also includes the serde implementations for all types. This doesn't need to be externally
// documented, though.

// Types with guaranteed stable serde representations. Strings are avoided to allow for optimal
// representations in various binary forms.

pub mod timestamp;

use crate::{
    error::ComponentRange, Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset,
    Weekday,
};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.year(), self.ordinal()).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Date {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let (year, ordinal) = Deserialize::deserialize(deserializer)?;
        Self::from_ordinal_date(year, ordinal).map_err(ComponentRange::to_invalid_serde_value::<D>)
    }
}

impl Serialize for Duration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.seconds, self.nanoseconds).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Duration {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let (seconds, nanoseconds) = Deserialize::deserialize(deserializer)?;
        Ok(Self::new(seconds, nanoseconds))
    }
}

impl Serialize for OffsetDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
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

impl Serialize for PrimitiveDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
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
        let (year, ordinal, hour, minute, second, nanosecond) =
            Deserialize::deserialize(deserializer)?;
        Date::from_ordinal_date(year, ordinal)
            .map_err(ComponentRange::to_invalid_serde_value::<D>)?
            .with_hms_nano(hour, minute, second, nanosecond)
            .map_err(ComponentRange::to_invalid_serde_value::<D>)
    }
}

impl Serialize for Time {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.hour, self.minute, self.second, self.nanosecond).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Time {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let (hour, minute, second, nanosecond) = Deserialize::deserialize(deserializer)?;
        Self::from_hms_nano(hour, minute, second, nanosecond)
            .map_err(ComponentRange::to_invalid_serde_value::<D>)
    }
}

impl Serialize for UtcOffset {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.hours, self.minutes, self.seconds).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for UtcOffset {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let (hours, minutes, seconds) = Deserialize::deserialize(deserializer)?;
        Self::from_hms(hours, minutes, seconds).map_err(ComponentRange::to_invalid_serde_value::<D>)
    }
}

impl Serialize for Weekday {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.number_from_monday().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Weekday {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
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
