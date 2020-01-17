use core::convert::TryFrom;

// 1-indexed day from Monday
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Weekday(u8);

impl From<crate::Weekday> for Weekday {
    #[inline]
    fn from(original: crate::Weekday) -> Self {
        Self(original.iso_weekday_number())
    }
}

impl TryFrom<Weekday> for crate::Weekday {
    type Error = &'static str;

    #[inline]
    fn try_from(original: Weekday) -> Result<Self, Self::Error> {
        match original {
            Weekday(1) => Ok(crate::Weekday::Monday),
            Weekday(2) => Ok(crate::Weekday::Tuesday),
            Weekday(3) => Ok(crate::Weekday::Wednesday),
            Weekday(4) => Ok(crate::Weekday::Thursday),
            Weekday(5) => Ok(crate::Weekday::Friday),
            Weekday(6) => Ok(crate::Weekday::Saturday),
            Weekday(7) => Ok(crate::Weekday::Sunday),
            _ => Err("invalid value"),
        }
    }
}
