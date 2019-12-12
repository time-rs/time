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
            Weekday(1) => Ok(Self::Monday),
            Weekday(2) => Ok(Self::Tuesday),
            Weekday(3) => Ok(Self::Wednesday),
            Weekday(4) => Ok(Self::Thursday),
            Weekday(5) => Ok(Self::Friday),
            Weekday(6) => Ok(Self::Saturday),
            Weekday(7) => Ok(Self::Sunday),
            _ => Err("invalid value"),
        }
    }
}
