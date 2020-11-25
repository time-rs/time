use core::convert::TryFrom;

// hour, minute, second, nanosecond
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Time(pub(crate) u8, pub(crate) u8, pub(crate) u8, pub(crate) u32);

impl From<crate::Time> for Time {
    fn from(time: crate::Time) -> Self {
        Self(time.hour, time.minute, time.second, time.nanosecond)
    }
}

impl TryFrom<Time> for crate::Time {
    type Error = &'static str;

    fn try_from(Time(hour, minute, second, nanosecond): Time) -> Result<Self, Self::Error> {
        Self::from_hms_nano(hour, minute, second, nanosecond).map_err(|_| "invalid time")
    }
}
