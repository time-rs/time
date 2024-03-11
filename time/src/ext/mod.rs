//! Extension traits.

mod digit_count;
mod numerical_duration;
mod numerical_std_duration;

pub(crate) use self::digit_count::DigitCount;
pub use self::numerical_duration::NumericalDuration;
pub use self::numerical_std_duration::NumericalStdDuration;
