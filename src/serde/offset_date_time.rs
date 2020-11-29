use core::convert::{TryFrom, TryInto};

// Date, Time, UtcOffset
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct OffsetDateTime(i32, u16, u8, u8, u8, u32, i8, i8, i8);

impl From<crate::OffsetDateTime> for OffsetDateTime {
    fn from(offset_date_time: crate::OffsetDateTime) -> Self {
        let date: crate::serde::Date = offset_date_time.date().into();
        let time: crate::serde::Time = offset_date_time.time().into();
        let offset: crate::serde::UtcOffset = offset_date_time.offset().into();
        Self(
            date.0, date.1, time.0, time.1, time.2, time.3, offset.0, offset.1, offset.2,
        )
    }
}

impl TryFrom<OffsetDateTime> for crate::OffsetDateTime {
    type Error = &'static str;

    fn try_from(
        OffsetDateTime(
            year,
            ordinal,
            hour,
            minute,
            second,
            nanosecond,
            offset_hours,
            offset_minutes,
            offset_seconds,
        ): OffsetDateTime,
    ) -> Result<Self, Self::Error> {
        let date = crate::serde::Date(year, ordinal).try_into()?;
        let time = crate::serde::Time(hour, minute, second, nanosecond).try_into()?;
        let offset =
            crate::serde::UtcOffset(offset_hours, offset_minutes, offset_seconds).try_into()?;
        Ok(crate::PrimitiveDateTime::new(date, time).assume_offset(offset))
    }
}
