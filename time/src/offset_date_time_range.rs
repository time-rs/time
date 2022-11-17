//! Implementations of useful methods for the [`Range<OffsetDateTime>`] and [`Vec<Range<OffsetDateTime>>`] structs.

use std::ops::Range;
use crate::{OffsetDateTime, Duration, util, Month, Weekday};
use iter::*;

/// Useful methods for the [`Range<OffsetDateTime>`] struct to expose.
pub trait OffsetDateTimeRange {
    /// Returns the length of the range using the specified duration as the unit.
    /// 
    /// As an example, using `Duration::DAY` as unit wouldn't count how many days are in the range, but rather how many day-long ranges are in the range.
    fn len(self, unit: Duration) -> usize;

    /// Returns an iterator of all the seconds that start within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn seconds(self) -> Seconds;

    /// Returns an iterator of all the minutes that start within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn minutes(self) -> Minutes;

    /// Returns an iterator of all the hours that start within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn hours(self) -> Hours;

    /// Returns an iterator of all the days that start within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn days(self) -> Days;

    /// Returns an iterator of all the monday-based weeks that start within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn monday_based_weeks(self) -> MondayBasedWeeks;

    /// Returns an iterator of all the sunday-based weeks that start within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn sunday_based_weeks(self) -> SundayBasedWeeks;

    /// Returns an iterator of all the months that start within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn months(self) -> Months;

    /// Returns an iterator of all the years that start within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn years(self) -> Years;

    /// Returns an iterator of all the seconds that start and end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_seconds(self) -> Seconds;

    /// Returns an iterator of all the minutes that start and end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_minutes(self) -> Minutes;

    /// Returns an iterator of all the hours that start and end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_hours(self) -> Hours;

    /// Returns an iterator of all the days that start and end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_days(self) -> Days;

    /// Returns an iterator of all the monday-based weeks that start and end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_monday_based_weeks(self) -> MondayBasedWeeks;

    /// Returns an iterator of all the sunday-based weeks that start and end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_sunday_based_weeks(self) -> SundayBasedWeeks;

    /// Returns an iterator of all the months that start and end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_months(self) -> Months;

    /// Returns an iterator of all the years that start and end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn full_years(self) -> Years;

    /// Returns an iterator of all the seconds that start or end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_seconds(self) -> Seconds;

    /// Returns an iterator of all the minutes that start or end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_minutes(self) -> Minutes;

    /// Returns an iterator of all the hours that start or end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_hours(self) -> Hours;

    /// Returns an iterator of all the days that start or end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_days(self) -> Days;

    /// Returns an iterator of all the monday-based weeks that start or end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_monday_based_weeks(self) -> MondayBasedWeeks;

    /// Returns an iterator of all the sunday-based weeks that start or end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_sunday_based_weeks(self) -> SundayBasedWeeks;

    /// Returns an iterator of all the months that start or end within the range.
    /// 
    /// The `OffsetDateTime` returned by the iterator will have the same offset as the range start.
    fn overlapping_months(self) -> Months;

    /// Returns an iterator of all the years that start or end within the range.
    /// 
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
    /// 
    /// If the `OffsetDateTime` is not within the range, the method will return `None`.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the range start.
    fn split_at(self, date: OffsetDateTime) -> Option<(Range<OffsetDateTime>, Range<OffsetDateTime>)>;

    /// Split the range into multiple ranges at the specified `OffsetDateTime`.
    /// 
    /// The range will only be split at the specified `OffsetDateTime` if it is within the range.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn split_at_multiple(self, dates: &[OffsetDateTime]) -> Vec<Range<OffsetDateTime>>;

    /// Split the range into ranges of specified duration.
    /// 
    /// If the unit is not a multiple of the range's duration, the last range will be shorter.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned by the iterator will have the same offset as this range start.
    fn split_by(self, unit: Duration) -> SplitDuration;

    /// Split the range into a list of ranges of equal duration.
    /// 
    /// If the number_of_parts is not a multiple of the range's duration, the first ranges returned will be longer than the last ones.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned by the iterator will have the same offset as this range start.
    fn divide_equally(self, number_of_parts: usize) -> SplitCount;

    /// Returns a range that is the intersection of this range and the specified one.
    /// 
    /// If the ranges do not intersect, the method will return `None`.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn intersection(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>>;

    /// Returns a range that is the union of this range and the specified one.
    /// 
    /// If the ranges do not overlap, the method will return `None`.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn union(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>>;

    /// Returns a list of ranges that don't overlap with the specified one.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn difference(self, other: Range<OffsetDateTime>) -> Vec<Range<OffsetDateTime>>;

    /// Returns a list of ranges that don't overlap with any of the specified one
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn difference_multiple(self, others: &[Range<OffsetDateTime>]) -> Vec<Range<OffsetDateTime>>;
}

/// Useful methods for the [`[Range<OffsetDateTime>]`](https://doc.rust-lang.org/std/primitive.slice.html) struct to expose.
pub trait OffsetDateTimeRangeSlice {
    /// Returns an intersection of all the ranges in the slice.
    /// 
    /// If there is no point where all the ranges intersect, the method will return `None`.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn intersection(&self) -> Option<Range<OffsetDateTime>>;

    /// Returns a range that is the union of all the ranges in the slice. If the intervals are adjacent, they will be merged.
    /// It essentially returns a range with start equal to the smallest start in the slice and end equal to the largest end in the slice.
    /// 
    /// If the ranges don't fully overlap, ie there are spots that are not covered by any range, the method will return `None`.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn union(&self) -> Option<Range<OffsetDateTime>>;

    /// Returns a list of ranges that are reduced to the minimum set of ranges that don't overlap.
    /// 
    /// This function combines oferlapping and adjacent ranges.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn merge(&self) -> Vec<Range<OffsetDateTime>>;

    /// Returns a list of ranges that only appear in one of the specified ranges.
    /// 
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn xor(&self) -> Vec<Range<OffsetDateTime>>;
}

/// [`Range<OffsetDateTime>`] related iterators.
pub mod iter {
    use super::*;

    /// An iterator of the seconds within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Seconds {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
    }

    impl Iterator for Seconds {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current = current.next_second();
                Some(current)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for Seconds {
        // We make the assumption that the range is not longer than usize::MAX seconds.
        // We also assume that the iterator has been instantiated correctly, and starts at the beginning of a second.
        fn len(&self) -> usize {
            // if the range start is after the range end, the range is empty
            // in that case, we return 0
            if self.end < self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_second =
                self.end.nanosecond() != 0;
            
            duration.whole_seconds() as usize + has_exceeded_second as usize
        }
    }
    
    /// An iterator of the minutes within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Minutes {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
    }

    impl Iterator for Minutes {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current = current.next_minute();
                Some(current)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for Minutes {
        // We make the assumption that the range is not longer than usize::MAX minutes.
        // We also assume that the iterator has been instantiated correctly, and starts at the beginning of a minute.
        fn len(&self) -> usize {
            // if the range start is after the range end, the range is empty
            // in that case, we return 0
            if self.end < self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_minute =
                self.end.second() != 0 ||
                self.end.nanosecond() != 0;
                
            duration.whole_minutes() as usize + has_exceeded_minute as usize
        }
    }

    /// An iterator of the hours within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Hours {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
    }

    impl Iterator for Hours {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current = current.next_hour();
                Some(current)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for Hours {
        // We make the assumption that the range is not longer than usize::MAX hours.
        // We also assume that the iterator has been instantiated correctly, and starts at the beginning of an hour.
        fn len(&self) -> usize {
            // if the range start is after the range end, the range is empty
            // in that case, we return 0
            if self.end < self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_hour =
                self.end.minute() != 0 ||
                self.end.second() != 0 ||
                self.end.nanosecond() != 0;
                
            duration.whole_hours() as usize + has_exceeded_hour as usize
        }
    }

    /// An iterator of the days within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Days {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
    }

    impl Iterator for Days {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current = current.next_day();
                Some(current)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for Days {
        // We make the assumption that the range is not longer than usize::MAX days.
        // We also assume that the iterator has been instantiated correctly, and starts at the beginning of a day.
        fn len(&self) -> usize {
            // if the range start is after the range end, the range is empty
            // in that case, we return 0
            if self.end < self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_day =
                self.end.hour() != 0 ||
                self.end.minute() != 0 ||
                self.end.second() != 0 ||
                self.end.nanosecond() != 0;
            
            duration.whole_days() as usize + has_exceeded_day as usize
        }
    }

    /// An iterator of the monday-based weeks within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MondayBasedWeeks {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
    }

    impl Iterator for MondayBasedWeeks {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current = current.next_monday_based_week();
                Some(current)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for MondayBasedWeeks {
        // We make the assumption that the range is not longer than usize::MAX weeks.
        // We also assume that the iterator has been instantiated correctly, and starts at the beginning of a monday-based week.
        fn len(&self) -> usize {
            // if the range start is after the range end, the range is empty
            // in that case, we return 0
            if self.end < self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_week =
                self.end.weekday() != Weekday::Monday ||
                self.end.hour() != 0 ||
                self.end.minute() != 0 ||
                self.end.second() != 0 ||
                self.end.nanosecond() != 0;
                
            duration.whole_weeks() as usize + has_exceeded_week as usize
        }
    }

    /// An iterator of the sunday-based weeks within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SundayBasedWeeks {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
    }

    impl Iterator for SundayBasedWeeks {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current = current.next_sunday_based_week();
                Some(current)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for SundayBasedWeeks {
        // We make the assumption that the range is not longer than usize::MAX weeks.
        // We also assume that the iterator has been instantiated correctly, and starts at the beginning of a sunday-based week.
        fn len(&self) -> usize {
            // if the range start is after the range end, the range is empty
            // in that case, we return 0
            if self.end < self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_week =
                self.end.weekday() != Weekday::Sunday ||
                self.end.hour() != 0 ||
                self.end.minute() != 0 ||
                self.end.second() != 0 ||
                self.end.nanosecond() != 0;
                
            duration.whole_weeks() as usize + has_exceeded_week as usize
        }
    }

    /// An iterator of the months within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Months {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
    }

    impl Iterator for Months {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current = current.next_month();
                Some(current)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for Months {
        // We make the assumption that the range is not longer than usize::MAX months.
        // We also assume that the iterator has been instantiated correctly, and starts at the beginning of a month.
        fn len(&self) -> usize {
            // if the range start is after the range end, the range is empty
            // in that case, we return 0
            if self.end < self.current {
                return 0;
            }

            // count the number of whole months
            let whole_months = if self.current.year() == self.end.year() {
                self.end.month() as usize - self.current.month() as usize
            } else {
                let whole_years = self.end.year() as usize - self.current.year() as usize - 1;
                let start_months = 13 - self.current.month() as usize;
                let end_months = self.end.month() as usize - 1;
                start_months + (whole_years * 12) + end_months
            };

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_month =
                self.end.day() != 1 ||
                self.end.hour() != 0 ||
                self.end.minute() != 0 ||
                self.end.second() != 0 ||
                self.end.nanosecond() != 0;
            
            whole_months + has_exceeded_month as usize
        }
    }

    /// An iterator of the years within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Years {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
    }

    impl Iterator for Years {
        type Item = OffsetDateTime;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current = current.next_year();
                Some(current)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for Years {
        // We make the assumption that the range is not longer than usize::MAX years.
        // We also assume that the iterator has been instantiated correctly, and starts at the beginning of a year.
        fn len(&self) -> usize {
            // if the range start is after the range end, the range is empty
            // in that case, we return 0
            if self.end < self.current {
                return 0;
            }

            // count the number of whole years
            let whole_years = self.end.year() as usize - self.current.year() as usize;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_year =
                self.end.month() != Month::January ||
                self.end.day() != 1 ||
                self.end.hour() != 0 ||
                self.end.minute() != 0 ||
                self.end.second() != 0 ||
                self.end.nanosecond() != 0;
                
            whole_years + has_exceeded_year as usize
        }
    }

    /// An iterator of the unit-wide ranges within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SplitDuration {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
        pub(super) unit: Duration,
    }

    impl Iterator for SplitDuration {
        type Item = Range<OffsetDateTime>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current += self.unit;
                if self.current > self.end {
                    Some(current..self.end)
                } else {
                    Some(current..self.current)
                }
            } else {
                None
            }
        }
    }

    /// An iterator of the  ranges within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SplitCount {
        pub(super) current: OffsetDateTime,
        pub(super) end: OffsetDateTime,
        pub(super) number_of_parts: usize,
    }

    impl Iterator for SplitCount {
        type Item = Range<OffsetDateTime>;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }
}

impl OffsetDateTimeRange for Range<OffsetDateTime> {
    fn len(self, unit: Duration) -> usize {
        todo!()
    }

    fn seconds(self) -> Seconds {
        Seconds {
            current: self.start.next_second(),
            end: self.end,
        }
    }

    fn minutes(self) -> Minutes {
        Minutes {
            current: self.start.next_minute(),
            end: self.end,
        }
    }

    fn hours(self) -> Hours {
        Hours {
            current: self.start.next_hour(),
            end: self.end,
        }
    }

    fn days(self) -> Days {
        Days {
            current: self.start.next_day(),
            end: self.end,
        }
    }

    fn monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.next_monday_based_week(),
            end: self.end,
        }
    }

    fn sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.next_sunday_based_week(),
            end: self.end,
        }
    }

    fn months(self) -> Months {
        Months {
            current: self.start.next_month(),
            end: self.end,
        }
    }

    fn years(self) -> Years {
        Years {
            current: self.start.next_year(),
            end: self.end,
        }
    }

    fn full_seconds(self) -> Seconds {
        Seconds {
            current: self.start.next_second(),
            end: self.end.floor_seconds(),
        }
    }

    fn full_minutes(self) -> Minutes {
        Minutes {
            current: self.start.next_minute(),
            end: self.end.floor_minutes(),
        }
    }

    fn full_hours(self) -> Hours {
        Hours {
            current: self.start.next_hour(),
            end: self.end.floor_hours(),
        }
    }

    fn full_days(self) -> Days {
        Days {
            current: self.start.next_day(),
            end: self.end.floor_days(),
        }
    }

    fn full_monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.next_monday_based_week(),
            end: self.end.floor_monday_based_weeks(),
        }
    }

    fn full_sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.next_sunday_based_week(),
            end: self.end.floor_sunday_based_weeks(),
        }
    }

    fn full_months(self) -> Months {
        Months {
            current: self.start.next_month(),
            end: self.end.floor_months(),
        }
    }

    fn full_years(self) -> Years {
        Years {
            current: self.start.next_year(),
            end: self.end.floor_years(),
        }
    }

    fn overlapping_seconds(self) -> Seconds {
        Seconds {
            current: self.start.floor_seconds(),
            end: self.end,
        }
    }

    fn overlapping_minutes(self) -> Minutes {
        Minutes {
            current: self.start.floor_minutes(),
            end: self.end,
        }
    }

    fn overlapping_hours(self) -> Hours {
        Hours {
            current: self.start.floor_hours(),
            end: self.end,
        }
    }

    fn overlapping_days(self) -> Days {
        Days {
            current: self.start.floor_days(),
            end: self.end,
        }
    }

    fn overlapping_monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.floor_monday_based_weeks(),
            end: self.end,
        }
    }

    fn overlapping_sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.floor_sunday_based_weeks(),
            end: self.end,
        }
    }

    fn overlapping_months(self) -> Months {
        Months {
            current: self.start.floor_months(),
            end: self.end,
        }
    }

    fn overlapping_years(self) -> Years {
        Years {
            current: self.start.floor_years(),
            end: self.end,
        }
    }

    fn overlaps(self, other: Range<OffsetDateTime>) -> bool {
        todo!()
    }

    fn left_adjacent_to(self, other: Range<OffsetDateTime>) -> bool {
        todo!()
    }

    fn right_adjacent_to(self, other: Range<OffsetDateTime>) -> bool {
        todo!()
    }

    fn engulfs(self, other: Range<OffsetDateTime>) -> bool {
        todo!()
    }

    fn engulfed_by(self, other: Range<OffsetDateTime>) -> bool {
        todo!()
    }

    fn split_at(self, date: OffsetDateTime) -> Option<(Range<OffsetDateTime>, Range<OffsetDateTime>)> {
        todo!()
    }

    fn split_at_multiple(self, dates: &[OffsetDateTime]) -> Vec<Range<OffsetDateTime>> {
        todo!()
    }

    fn split_by(self, unit: Duration) -> SplitDuration {
        todo!()
    }

    fn divide_equally(self, number_of_parts: usize) -> SplitCount {
        todo!()
    }

    fn intersection(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>> {
        todo!()
    }

    fn union(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>> {
        todo!()
    }

    fn difference(self, other: Range<OffsetDateTime>) -> Vec<Range<OffsetDateTime>> {
        todo!()
    }

    fn difference_multiple(self, others: &[Range<OffsetDateTime>]) -> Vec<Range<OffsetDateTime>> {
        todo!()
    }
}
