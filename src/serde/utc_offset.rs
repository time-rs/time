use core::convert::TryFrom;

// seconds offset from UTC, positive is east
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct UtcOffset(pub(crate) i32);

impl From<crate::UtcOffset> for UtcOffset {
    fn from(offset: crate::UtcOffset) -> Self {
        Self(offset.as_seconds())
    }
}

impl TryFrom<UtcOffset> for crate::UtcOffset {
    type Error = &'static str;

    fn try_from(UtcOffset(offset): UtcOffset) -> Result<Self, Self::Error> {
        Self::seconds(offset).map_err(|_| "invalid offset")
    }
}
