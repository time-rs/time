// second, nanosecond
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Duration(i64, i32);

impl From<crate::Duration> for Duration {
    fn from(duration: crate::Duration) -> Self {
        Self(duration.seconds, duration.nanoseconds)
    }
}

impl From<Duration> for crate::Duration {
    fn from(Duration(seconds, nanoseconds): Duration) -> Self {
        Self::new(seconds, nanoseconds)
    }
}
