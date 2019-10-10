use crate::Duration;

/// Create `Duration`s from primitive and core numeric types.
pub trait NumericalDuration {
    /// Create a `Duration` from the number of nanoseconds.
    fn nanoseconds(self) -> Duration;
    /// Create a `Duration` from the number of microseconds.
    fn microseconds(self) -> Duration;
    /// Create a `Duration` from the number of milliseconds.
    fn milliseconds(self) -> Duration;
    /// Create a `Duration` from the number of seconds.
    fn seconds(self) -> Duration;
    /// Create a `Duration` from the number of minutes.
    fn minutes(self) -> Duration;
    /// Create a `Duration` from the number of hours.
    fn hours(self) -> Duration;
    /// Create a `Duration` from the number of days.
    fn days(self) -> Duration;
    /// Create a `Duration` from the number of weeks.
    fn weeks(self) -> Duration;
}

macro_rules! impl_numerical_duration {
    ($($type:ty),* $(,)?) => {
        $(
            #[allow(trivial_numeric_casts, clippy::use_self)]
            impl NumericalDuration for $type {
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self as i64)
                }

                fn microseconds(self) -> Duration {
                    Duration::microseconds(self as i64)
                }

                fn milliseconds(self) -> Duration {
                    Duration::milliseconds(self as i64)
                }

                fn seconds(self) -> Duration {
                    Duration::seconds(self as i64)
                }

                fn minutes(self) -> Duration {
                    Duration::minutes(self as i64)
                }

                fn hours(self) -> Duration {
                    Duration::hours(self as i64)
                }

                fn days(self) -> Duration {
                    Duration::days(self as i64)
                }

                fn weeks(self) -> Duration {
                    Duration::weeks(self as i64)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_duration_nonzero {
    ($($type:ty),* $(,)?) => {
        $(
            #[allow(trivial_numeric_casts, clippy::use_self)]
            impl NumericalDuration for $type {
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self.get() as i64)
                }

                fn microseconds(self) -> Duration {
                    Duration::microseconds(self.get() as i64)
                }

                fn milliseconds(self) -> Duration {
                    Duration::milliseconds(self.get() as i64)
                }

                fn seconds(self) -> Duration {
                    Duration::seconds(self.get() as i64)
                }

                fn minutes(self) -> Duration {
                    Duration::minutes(self.get() as i64)
                }

                fn hours(self) -> Duration {
                    Duration::hours(self.get() as i64)
                }

                fn days(self) -> Duration {
                    Duration::days(self.get() as i64)
                }

                fn weeks(self) -> Duration {
                    Duration::weeks(self.get() as i64)
                }
            }
        )*
    };
}

impl_numerical_duration![u8, u16, u32, i8, i16, i32, i64];
impl_numerical_duration_nonzero![
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
    core::num::NonZeroI8,
    core::num::NonZeroI16,
    core::num::NonZeroI32,
    core::num::NonZeroI64,
];
