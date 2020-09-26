use core::convert::TryFrom;

// year, ordinal
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Date(pub(crate) i32, pub(crate) u16);

impl From<crate::Date> for Date {
    fn from(date: crate::Date) -> Self {
        Self(date.year(), date.ordinal())
    }
}

impl TryFrom<Date> for crate::Date {
    type Error = &'static str;

    fn try_from(Date(year, ordinal): Date) -> Result<Self, Self::Error> {
        Self::from_yo(year, ordinal).map_err(|_| "invalid date")
    }
}
