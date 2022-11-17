//! Implementations of useful methods for the [`Range<OffsetDateTime>`] and [`Vec<Range<OffsetDateTime>>`] structs.

use std::ops::Range;
use crate::{OffsetDateTime, Duration, util};
use iter::*;

/// Define useful methods for the [`Range<OffsetDateTime>`] struct to expose.
pub trait OffsetDateTimeRange {
    /// Returns an iterator of all the seconds that start within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn seconds(self) -> Seconds;

    /// Returns an iterator of all the minutes that start within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn minutes(self) -> Minutes;

    /// Returns an iterator of all the hours that start within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn hours(self) -> Hours;

    /// Returns an iterator of all the days that start within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn days(self) -> Days;

    /// Returns an iterator of all the monday-based weeks that start within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn monday_based_weeks(self) -> Weeks;

    /// Returns an iterator of all the sunday-based weeks that start within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn sunday_based_weeks(self) -> Weeks;

    /// Returns an iterator of all the months that start within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn months(self) -> Months;

    /// Returns an iterator of all the years that start within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn years(self) -> Years;

    /// Returns an iterator of all the seconds that start and end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_seconds(self) -> Seconds;

    /// Returns an iterator of all the minutes that start and end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_minutes(self) -> Minutes;

    /// Returns an iterator of all the hours that start and end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_hours(self) -> Hours;

    /// Returns an iterator of all the days that start and end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_days(self) -> Days;

    /// Returns an iterator of all the monday-based weeks that start and end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_monday_based_weeks(self) -> Weeks;

    /// Returns an iterator of all the sunday-based weeks that start and end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_sunday_based_weeks(self) -> Weeks;

    /// Returns an iterator of all the months that start and end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_months(self) -> Months;

    /// Returns an iterator of all the years that start and end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_years(self) -> Years;

    /// Returns an iterator of all the seconds that start or end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_seconds(self) -> Seconds;

    /// Returns an iterator of all the minutes that start or end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_minutes(self) -> Minutes;

    /// Returns an iterator of all the hours that start or end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_hours(self) -> Hours;

    /// Returns an iterator of all the days that start or end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_days(self) -> Days;

    /// Returns an iterator of all the monday-based weeks that start or end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_monday_based_weeks(self) -> Weeks;

    /// Returns an iterator of all the sunday-based weeks that start or end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_sunday_based_weeks(self) -> Weeks;

    /// Returns an iterator of all the months that start or end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_months(self) -> Months;

    /// Returns an iterator of all the years that start or end within the range.
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_years(self) -> Years;

    /// Returns whether this range overlaps with the specified one.
    fn overlaps(self, other: Range<OffsetDateTime>) -> bool;

    /// Returns whether this range's end is equal to the specified one's start.
    fn left_adjacent_to(self, other: Range<OffsetDateTime>) -> bool;

    /// Returns whether this range's start is equal to the specified one's end.
    fn right_adjacent_to(self, other: Range<OffsetDateTime>) -> bool;

    /// Returns whether the specified range is contained within this range.
    fn engulfs(self, other: Range<OffsetDateTime>) -> bool;

    /// Returns whether this range is contained within the specified range.
    fn engulfed_by(self, other: Range<OffsetDateTime>) -> bool;

    /// Split the range into two ranges at the specified `OffsetDateTime`.
    /// If the `OffsetDateTime` is not within the range, the method will panic.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the range start.
    fn split_at(self, date: OffsetDateTime) -> (Range<OffsetDateTime>, Range<OffsetDateTime>);

    /// Split the range into two ranges at the specified `OffsetDateTime`.
    /// If the `OffsetDateTime` is not within the range, the method will return `None`.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn try_split_at(self, date: OffsetDateTime) -> Option<(Range<OffsetDateTime>, Range<OffsetDateTime>)>;

    /// Split the range into multiple ranges at the specified `OffsetDateTime`.
    /// The range will only be split at the specified `OffsetDateTime` if it is within the range.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn split_at_multiple(self, dates: &[OffsetDateTime]) -> Vec<Range<OffsetDateTime>>;

    /// Split the range into ranges of specified duration.
    /// If the duration is not a multiple of the range's duration, the last range will be shorter.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned by the iterator will have the same offset as this range start.
    fn split_by(self, duration: Duration) -> SplitDuration;

    /// Split the range into a list of ranges of equal duration.
    /// If the number_of_parts is not a multiple of the range's duration, the first ranges returned will be longer than the last ones.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned by the iterator will have the same offset as this range start.
    fn divide_equally(self, number_of_parts: usize) -> SplitCount;

    /// Returns a range that is the intersection of this range and the specified one.
    /// If the ranges do not intersect, the method will return `None`.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn intersection(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>>;

    /// Returns a range that is the union of this range and the specified one.
    /// If the ranges do not overlap, the method will return `None`.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn union(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>>;

    /// Returns a range that doesn't overlap with the specified one.
    /// If the ranges completely overlap, the method will return `None`.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn difference(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>>;

    /// Returns a range that doesn't overlap with any of the specified one
    /// If the ranges completely overlap, the method will return `None`.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn difference_multiple(self, others: &[Range<OffsetDateTime>]) -> Vec<Range<OffsetDateTime>>;
}

/// Define useful methods for the [`[Range<OffsetDateTime>]`] struct to expose.
trait OffsetDateTimeRangeSlice {
    /// Returns an intersection of all the ranges in the slice.
    /// If there is no point where all the ranges intersect, the method will return `None`.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn intersection(&self) -> Option<Range<OffsetDateTime>>;

    /// Returns a range that is the union of all the ranges in the slice. If the intervals are adjacent, they will be merged.
    /// If the ranges don't fully overlap, the method will return `None`.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn union(&self) -> Option<Range<OffsetDateTime>>;

    /// Returns a list of ranges that are reduced to the minimum set of ranges that don't overlap.
    /// This function combines oferlapping and adjacent ranges.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn merge(&self) -> Vec<Range<OffsetDateTime>>;

    /// Returns a list of ranges that only appear in one of the specified ranges.
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn xor(&self) -> Vec<Range<OffsetDateTime>>;
}

// impl OffsetDateTimeRangeSlice for [Range<OffsetDateTime>] {
//     fn intersection(&self) -> Range<OffsetDateTime> {
//         todo!();
//     }
// }

/// [`Range<OffsetDateTime>`] related iterators.
pub mod iter {
    use alloc::collections::binary_heap::Iter;

    use super::*;

    /// An iterator of the seconds within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Seconds {
        current: OffsetDateTime,
        end: OffsetDateTime,
        inclusive: bool,
    }

    impl Iterator for Seconds {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end || (self.inclusive && self.current == self.end) {
                let current = self.current;
                self.current = current.checked_add(Duration::SECOND).unwrap();
                Some(current)
            } else {
                None
            }
        }
    }
    
    /// An iterator of the minutes within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Minutes {
        current: OffsetDateTime,
        end: OffsetDateTime,
        inclusive: bool,
    }

    impl Iterator for Minutes {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end || (self.inclusive && self.current == self.end) {
                let current = self.current;
                self.current = current.checked_add(Duration::MINUTE).unwrap();
                Some(current)
            } else {
                None
            }
        }
    }

    /// An iterator of the hours within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Hours {
        current: OffsetDateTime,
        end: OffsetDateTime,
        inclusive: bool,
    }

    impl Iterator for Hours {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end || (self.inclusive && self.current == self.end) {
                let current = self.current;
                self.current = current.checked_add(Duration::HOUR).unwrap();
                Some(current)
            } else {
                None
            }
        }
    }

    /// An iterator of the days within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Days {
        current: OffsetDateTime,
        end: OffsetDateTime,
        inclusive: bool,
    }

    impl Iterator for Days {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end || (self.inclusive && self.current == self.end) {
                let current = self.current;
                self.current = current.checked_add(Duration::DAY).unwrap();
                Some(current)
            } else {
                None
            }
        }
    }

    /// An iterator of the weeks within a range.
    /// Might be monday-based or sunday-based depending on the range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Weeks {
        current: OffsetDateTime,
        end: OffsetDateTime,
        inclusive: bool,
    }

    impl Iterator for Weeks {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end || (self.inclusive && self.current == self.end) {
                let current = self.current;
                self.current = current.checked_add(Duration::WEEK).unwrap();
                Some(current)
            } else {
                None
            }
        }
    }

    /// An iterator of the months within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Months {
        current: OffsetDateTime,
        end: OffsetDateTime,
        inclusive: bool,
    }

    impl Iterator for Months {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end || (self.inclusive && self.current == self.end) {
                let current = self.current;
                self.current = current.checked_add(Duration::days(util::days_in_year_month(current.year(), current.month()) as i64)).unwrap();
                Some(current)
            } else {
                None
            }
        }
    }

    /// An iterator of the years within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Years {
        current: OffsetDateTime,
        end: OffsetDateTime,
        inclusive: bool,
    }

    impl Iterator for Years {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end || (self.inclusive && self.current == self.end) {
                let current = self.current;
                self.current = current.checked_add(Duration::days(util::days_in_year(current.year()) as i64)).unwrap();
                Some(current)
            } else {
                None
            }
        }
    }

    pub struct SplitDuration {
    }

    impl Iterator for SplitDuration {
        type Item = Range<OffsetDateTime>;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    pub struct SplitCount {
    }

    impl Iterator for SplitCount {
        type Item = Range<OffsetDateTime>;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }
}

// TODO: implement the methods for Range<OffsetDateTime> and RangeInclusive<OffsetDateTime>
