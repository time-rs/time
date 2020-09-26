use core::convert::{TryFrom, TryInto};

// Date, Time
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct PrimitiveDateTime(i32, u16, u8, u8, u8, u32);

impl From<crate::PrimitiveDateTime> for PrimitiveDateTime {
    fn from(date_time: crate::PrimitiveDateTime) -> Self {
        let date: crate::serde::Date = date_time.date().into();
        let time: crate::serde::Time = date_time.time().into();
        Self(date.0, date.1, time.0, time.1, time.2, time.3)
    }
}

impl TryFrom<PrimitiveDateTime> for crate::PrimitiveDateTime {
    type Error = &'static str;

    fn try_from(
        PrimitiveDateTime(year, ordinal, hour, minute, second, nanosecond): PrimitiveDateTime,
    ) -> Result<Self, Self::Error> {
        let date = crate::serde::Date(year, ordinal).try_into()?;
        let time = crate::serde::Time(hour, minute, second, nanosecond).try_into()?;
        Ok(Self::new(date, time))
    }
}
