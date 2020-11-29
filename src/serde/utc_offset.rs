use core::convert::TryFrom;

// hours, minutes, seconds
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct UtcOffset(pub(crate) i8, pub(crate) i8, pub(crate) i8);

impl From<crate::UtcOffset> for UtcOffset {
    fn from(offset: crate::UtcOffset) -> Self {
        Self(offset.hours, offset.minutes, offset.seconds)
    }
}

impl TryFrom<UtcOffset> for crate::UtcOffset {
    type Error = &'static str;

    fn try_from(UtcOffset(hours, minutes, seconds): UtcOffset) -> Result<Self, Self::Error> {
        Self::from_hms(hours, minutes, seconds).map_err(|_| "invalid offset")
    }
}
