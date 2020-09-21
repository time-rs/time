use standback::convert::TryFrom;

// seconds offset from UTC, positive is east
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct UtcOffset(i32);

impl From<crate::UtcOffset> for UtcOffset {
    fn from(original: crate::UtcOffset) -> Self {
        Self(original.as_seconds())
    }
}

impl TryFrom<UtcOffset> for crate::UtcOffset {
    type Error = &'static str;

    fn try_from(original: UtcOffset) -> Result<Self, Self::Error> {
        Self::seconds(original.0).map_err(|_| "invalid value")
    }
}
