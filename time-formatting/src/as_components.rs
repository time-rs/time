//! Helper trait to abstract away the details of any given type.

use time::{Date, Time, UtcOffset};

/// Convert a value into its raw (possibly computed) components.
pub trait AsComponents {
    /// Obtain a type as its raw components. The date and time components must
    /// be the values in the offset, if present.
    fn as_components(&self) -> (Option<Date>, Option<Time>, Option<UtcOffset>);
}

impl AsComponents for Date {
    fn as_components(&self) -> (Option<Date>, Option<Time>, Option<UtcOffset>) {
        (Some(*self), None, None)
    }
}

impl AsComponents for Time {
    fn as_components(&self) -> (Option<Date>, Option<Time>, Option<UtcOffset>) {
        (None, Some(*self), None)
    }
}

impl AsComponents for UtcOffset {
    fn as_components(&self) -> (Option<Date>, Option<Time>, Option<UtcOffset>) {
        (None, None, Some(*self))
    }
}

impl AsComponents for time::PrimitiveDateTime {
    fn as_components(&self) -> (Option<Date>, Option<Time>, Option<UtcOffset>) {
        (Some(self.date()), Some(self.time()), None)
    }
}

impl AsComponents for time::OffsetDateTime {
    fn as_components(&self) -> (Option<Date>, Option<Time>, Option<UtcOffset>) {
        (Some(self.date()), Some(self.time()), Some(self.offset()))
    }
}
