//! Implementations of useful methods for the [`Range<OffsetDateTime>`] and [`&[Range<OffsetDateTime>]`](https://doc.rust-lang.org/std/primitive.slice.html) structs.

use crate::{Duration, Month, OffsetDateTime, Weekday};
use iter::*;
use std::ops::Range;

/// Extension of [`Range<OffsetDateTime>`] containing useful methods.
///
/// If you want to access these methods, you must import the [`OffsetDateTimeRangeExt`] trait.
pub trait OffsetDateTimeRangeExt {
    /// Returns the length of the range using the specified duration as the unit.
    ///
    /// Attention, using `Duration::DAY` as unit wouldn't count how many days are in the range, but rather how many day-long ranges would fit inside.
    /// If you want to know the number of days starting in the range, use [`OffsetDateTimeRangeExt::days`] and [`iter::Days::len`] instead.
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
    fn split_at(
        self,
        date: OffsetDateTime,
    ) -> Option<(Range<OffsetDateTime>, Range<OffsetDateTime>)>;

    /// Split the range into multiple ranges at the specified `OffsetDateTime`.
    ///
    /// The range will only be split at the specified `OffsetDateTime` if it is within the range.
    ///
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as this range start.
    fn split_at_multiple(self, dates: &[OffsetDateTime]) -> Vec<Range<OffsetDateTime>>;

    /// Split the range into ranges of specified duration.
    ///
    /// If the unit is not a multiple of the range's duration, the last range will be shorter.
    /// If the unit is zero or negative, the method will panic.
    ///
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned by the iterator will have the same offset as this range start.
    fn split_by(self, unit: Duration) -> SplitDuration;

    /// Split the range into a list of ranges of equal duration.
    ///
    /// If the `number_of_parts` is not a multiple of the range's duration, the first ranges returned will be longer than the last ones.
    /// If the `number_of_parts` is larger than the range's duration, the last ranges will be empty.
    /// If the `number_of_parts` is 0, the method will panic.
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

/// Extension of [`&[Range<OffsetDateTime>]`](https://doc.rust-lang.org/std/primitive.slice.html) containing useful methods.
///
/// If you want to access these methods, you must import the [`OffsetDateTimeRangeSliceExt`] trait.
pub trait OffsetDateTimeRangeSliceExt {
    /// Returns an intersection of all the ranges in the slice.
    ///
    /// If there is no point where all the ranges intersect, the method will return `None`.
    ///
    /// The `OffsetDateTime` in the `Range<OffsetDateTime>` returned will have the same offset as the first range start.
    fn intersection(&self) -> Option<Range<OffsetDateTime>>;

    /// Returns a range that is the union of all the ranges in the slice. If the intervals are adjacent, they will be merged.
    /// It essentially returns a range with start equal to the smallest start in the slice and end equal to the largest end in the slice.
    ///
    /// If the ranges don't fully overlap, ie there are gaps that are not covered by any range, the method will return `None`.
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
            if self.end <= self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_second = self.end.nanosecond() != 0;

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
            if self.end <= self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_minute = self.end.second() != 0 || self.end.nanosecond() != 0;

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
            if self.end <= self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_hour =
                self.end.minute() != 0 || self.end.second() != 0 || self.end.nanosecond() != 0;

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
            if self.end <= self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_day = self.end.hour() != 0
                || self.end.minute() != 0
                || self.end.second() != 0
                || self.end.nanosecond() != 0;

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
            if self.end <= self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_week = self.end.weekday() != Weekday::Monday
                || self.end.hour() != 0
                || self.end.minute() != 0
                || self.end.second() != 0
                || self.end.nanosecond() != 0;

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
            if self.end <= self.current {
                return 0;
            }

            // get the duration of the range
            let duration = self.end - self.current;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_week = self.end.weekday() != Weekday::Sunday
                || self.end.hour() != 0
                || self.end.minute() != 0
                || self.end.second() != 0
                || self.end.nanosecond() != 0;

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
            if self.end <= self.current {
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
            let has_exceeded_month = self.end.day() != 1
                || self.end.hour() != 0
                || self.end.minute() != 0
                || self.end.second() != 0
                || self.end.nanosecond() != 0;

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
            if self.end <= self.current {
                return 0;
            }

            // count the number of whole years
            let whole_years = self.end.year() as usize - self.current.year() as usize;

            // basically a ceil function, but without f64 conversion
            // this way we don't have to worry about rounding errors
            let has_exceeded_year = self.end.month() != Month::January
                || self.end.day() != 1
                || self.end.hour() != 0
                || self.end.minute() != 0
                || self.end.second() != 0
                || self.end.nanosecond() != 0;

            whole_years + has_exceeded_year as usize
        }
    }

    /// An iterator of the unit-wide ranges within a range.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SplitDuration {
        current: OffsetDateTime,
        end: OffsetDateTime,
        unit: Duration,
    }

    impl SplitDuration {
        pub(super) fn new(start: OffsetDateTime, end: OffsetDateTime, unit: Duration) -> Self {
            // it is impossible to add enough 0 to get to the end of the range, so we panic
            if unit == Duration::ZERO {
                panic!("Cannot split a range into units of 0");
            }

            // if the unit is negative, we panic
            if unit.is_negative() {
                Self {
                    current: start,
                    end,
                    unit,
                }
            }
            // we build the iterator
            else {
                Self {
                    current: start,
                    end,
                    unit,
                }
            }
        }
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
        current: OffsetDateTime,
        end: OffsetDateTime,
        remaining_extended_ranges: usize,
        extended_ranges_duration: Duration,
        remaining_normal_ranges: usize,
        normal_ranges_duration: Duration,
    }

    impl SplitCount {
        pub(super) fn new(current: OffsetDateTime, end: OffsetDateTime, number_of_parts: usize) -> Self {
            // it is impossible to divide something by 0, so we panic
            if number_of_parts == 0 {
                panic!("Cannot split a range into 0 parts");
            }

            // if the range start is after the range end, the range is empty
            if end <= current {
                Self {
                    current,
                    end,
                    remaining_extended_ranges: 0,
                    extended_ranges_duration: Duration::ZERO,
                    remaining_normal_ranges: 0,
                    normal_ranges_duration: Duration::ZERO,
                }
            }
            // in case everything is normal
            else {
                // compute the duration of normal ranges
                let range_duration = (end - current).whole_nanoseconds() as u128;
                let normal_ranges_length = (range_duration / number_of_parts as u128) as i64;
                let extended_ranges_count = (range_duration % number_of_parts as u128) as usize;

                let normal_ranges_duration = Duration::nanoseconds(normal_ranges_length);
                let extended_ranges_duration = Duration::nanoseconds(normal_ranges_length + 1);

                Self {
                    current,
                    end,
                    remaining_extended_ranges: extended_ranges_count,
                    extended_ranges_duration,
                    remaining_normal_ranges: number_of_parts - extended_ranges_count,
                    normal_ranges_duration,
                }
            }
        }
    }

    impl Iterator for SplitCount {
        type Item = Range<OffsetDateTime>;

        fn next(&mut self) -> Option<Self::Item> {
            // if there are still extended ranges to be returned, then return them first
            if self.remaining_extended_ranges != 0 {
                self.remaining_extended_ranges -= 1;
                let current = self.current;
                self.current += self.extended_ranges_duration;
                Some(current..self.current)
            }
            // if there we don't have exhausted all the ranges yet, then continue returning them
            else if self.remaining_normal_ranges != 0 {
                self.remaining_normal_ranges -= 1;
                let current = self.current;
                self.current += self.normal_ranges_duration;
                if self.current > self.end {
                    Some(current..self.end)
                } else {
                    Some(current..self.current)
                }
            }
            // once we are out of ranges, return None
            else {
                None
            }
        }
    }

    impl ExactSizeIterator for SplitCount {
        fn len(&self) -> usize {
            self.remaining_extended_ranges + self.remaining_normal_ranges
        }
    }
}

/// Implementation of the `OffsetDateTimeRangeExt` trait for `Range<OffsetDateTime>`.
impl OffsetDateTimeRangeExt for Range<OffsetDateTime> {
    fn len(self, unit: Duration) -> usize {
        // if the range start is after the range end, the range is empty
        // and we return 0
        if self.end <= self.start {
            return 0;
        }

        // if the unit is zero, we return 0
        if unit == Duration::ZERO {
            return 0;
        }

        // if the duration is negative, we return 0
        if unit.is_negative() {
            return 0;
        }
        // if the range isn't empty, then the duration is positive
        else {
            let range_duration = (self.end - self.start).whole_nanoseconds() as u128;
            let unit_duration = unit.whole_nanoseconds() as u128;
            (range_duration / unit_duration) as usize
        }
    }

    fn seconds(self) -> Seconds {
        Seconds {
            current: self.start.ceil_seconds(),
            end: self.end,
        }
    }

    fn minutes(self) -> Minutes {
        Minutes {
            current: self.start.ceil_minutes(),
            end: self.end,
        }
    }

    fn hours(self) -> Hours {
        Hours {
            current: self.start.ceil_hours(),
            end: self.end,
        }
    }

    fn days(self) -> Days {
        Days {
            current: self.start.ceil_days(),
            end: self.end,
        }
    }

    fn monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.ceil_monday_based_weeks(),
            end: self.end,
        }
    }

    fn sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.ceil_sunday_based_weeks(),
            end: self.end,
        }
    }

    fn months(self) -> Months {
        Months {
            current: self.start.ceil_months(),
            end: self.end,
        }
    }

    fn years(self) -> Years {
        Years {
            current: self.start.ceil_years(),
            end: self.end,
        }
    }

    fn full_seconds(self) -> Seconds {
        Seconds {
            current: self.start.ceil_seconds(),
            end: self.end.floor_seconds(),
        }
    }

    fn full_minutes(self) -> Minutes {
        Minutes {
            current: self.start.ceil_minutes(),
            end: self.end.floor_minutes(),
        }
    }

    fn full_hours(self) -> Hours {
        Hours {
            current: self.start.ceil_hours(),
            end: self.end.floor_hours(),
        }
    }

    fn full_days(self) -> Days {
        Days {
            current: self.start.ceil_days(),
            end: self.end.floor_days(),
        }
    }

    fn full_monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.ceil_monday_based_weeks(),
            end: self.end.floor_monday_based_weeks(),
        }
    }

    fn full_sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.ceil_sunday_based_weeks(),
            end: self.end.floor_sunday_based_weeks(),
        }
    }

    fn full_months(self) -> Months {
        Months {
            current: self.start.ceil_months(),
            end: self.end.floor_months(),
        }
    }

    fn full_years(self) -> Years {
        Years {
            current: self.start.ceil_years(),
            end: self.end.floor_years(),
        }
    }

    fn overlapping_seconds(self) -> Seconds {
        Seconds {
            current: self.start.floor_seconds(),
            end: self.end.ceil_seconds(),
        }
    }

    fn overlapping_minutes(self) -> Minutes {
        Minutes {
            current: self.start.floor_minutes(),
            end: self.end.ceil_minutes(),
        }
    }

    fn overlapping_hours(self) -> Hours {
        Hours {
            current: self.start.floor_hours(),
            end: self.end.ceil_hours(),
        }
    }

    fn overlapping_days(self) -> Days {
        Days {
            current: self.start.floor_days(),
            end: self.end.ceil_days(),
        }
    }

    fn overlapping_monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.floor_monday_based_weeks(),
            end: self.end.ceil_monday_based_weeks(),
        }
    }

    fn overlapping_sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.floor_sunday_based_weeks(),
            end: self.end.ceil_sunday_based_weeks(),
        }
    }

    fn overlapping_months(self) -> Months {
        Months {
            current: self.start.floor_months(),
            end: self.end.ceil_months(),
        }
    }

    fn overlapping_years(self) -> Years {
        Years {
            current: self.start.floor_years(),
            end: self.end.ceil_years(),
        }
    }

    fn overlaps(self, other: Range<OffsetDateTime>) -> bool {
        self.start < self.end && other.start < other.end && self.start < other.end && other.start < self.end
    }

    fn left_adjacent_to(self, other: Range<OffsetDateTime>) -> bool {
        self.start < self.end && other.start < other.end && self.end == other.start
    }

    fn right_adjacent_to(self, other: Range<OffsetDateTime>) -> bool {
        self.start < self.end && other.start < other.end && self.start == other.end
    }

    fn engulfs(self, other: Range<OffsetDateTime>) -> bool {
        self.start < self.end && other.start < other.end && self.start <= other.start && other.end <= self.end
    }

    fn engulfed_by(self, other: Range<OffsetDateTime>) -> bool {
        self.start < self.end && other.start < other.end && other.start <= self.start && self.end <= other.end
    }

    fn split_at(mut self, mut date: OffsetDateTime) -> Option<(Range<OffsetDateTime>, Range<OffsetDateTime>)> {
        // if the range is is the right order, and the date is within the range, return `Some`
        if self.start < self.end && self.start < date && date < self.end {
            // find the start offset to make sure each sub-range has the same offset as the original
            let start_offset = self.start.offset();

            // update the offsets to match the original offset
            date = date.to_offset(start_offset);
            self.end = self.end.to_offset(start_offset);

            // split the range
            Some((self.start..date, date..self.end))
        }
        // if the range is reversed or the date is outside of the range, return `None`
        else {
            None
        }
    }

    fn split_at_multiple(mut self, dates: &[OffsetDateTime]) -> Vec<Range<OffsetDateTime>> {
        // if the range is reversed, return an empty vector
        if self.end <= self.start {
            return vec![];
        }

        // find the start offset to make sure each sub-range has the same offset as the original
        let start_offset = self.start.offset();

        // ensure the end is in the same offset, else the last range will end with a different offset compared to others
        self.end = self.end.to_offset(start_offset);

        let mut sorted_dates: Vec<OffsetDateTime> = dates
            // iterate over the dates in the slice
            .iter()
            // convert &OffsetDateTime to OffsetDateTime
            .copied()
            // filter out dates that are not in the range
            .filter(|date| self.start < *date && *date < self.end)
            // make sure all dates have the same offset as the start of the range
            .map(|date| date.to_offset(start_offset))
            // collect the date into a vector
            .collect();

        // sort the dates
        sorted_dates.sort();

        // filter out any dates that are adjacent to each other
        sorted_dates.dedup();

        // generate the ranges and collect them into a vector
        let mut ranges = vec![];
        let mut tmp = self;

        for date in sorted_dates {
            ranges.push(tmp.start..date);
            tmp = date..tmp.end;
        }
        ranges.push(tmp); // add the last range

        // return the ranges
        ranges
    }

    fn split_by(self, unit: Duration) -> SplitDuration {
        SplitDuration::new(self.start, self.end, unit)
    }

    fn divide_equally(self, number_of_parts: usize) -> SplitCount {
        SplitCount::new(self.start, self.end, number_of_parts)
    }

    fn intersection(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start < end {
            Some(start..end)
        } else {
            None
        }
    }

    fn union(self, other: Range<OffsetDateTime>) -> Option<Range<OffsetDateTime>> {
        if self.start < other.end && other.start < self.end {
            let start = self.start.min(other.start);
            let end = self.end.max(other.end);
            Some(start..end)
        } else {
            None
        }
    }

    fn difference(self, other: Range<OffsetDateTime>) -> Vec<Range<OffsetDateTime>> {
        self.difference_multiple(&[other])
    }

    fn difference_multiple(self, others: &[Range<OffsetDateTime>]) -> Vec<Range<OffsetDateTime>> {
        // if the range is reversed, return an empty vector
        if self.end <= self.start {
            return vec![];
        }

        // compute the difference
        [&[self.clone()], others]
            .concat()
            .xor()
            .into_iter()
            .filter_map(|range| range.intersection(self.clone()))
            .filter(|range| range.start < range.end)
            .collect::<Vec<_>>()
    }
}

/// Implementation of the `OffsetDateTimeRangeSliceExt` trait for `&[Range<OffsetDateTime>]`.
impl OffsetDateTimeRangeSliceExt for [Range<OffsetDateTime>] {
    fn intersection(&self) -> Option<Range<OffsetDateTime>> {
        // we intersect all the ranges one after another
        let mut intersection = self.first()?.clone();

        for range in self.iter().skip(1).cloned() {
            // if one of the intersection fails, return `None`
            intersection = intersection.intersection(range)?;
        }

        // return the intersection
        Some(intersection)
    }

    fn union(&self) -> Option<Range<OffsetDateTime>> {
        // we union all the ranges one after another
        let mut union = self.first()?.clone();

        for range in self.iter().skip(1).cloned() {
            // if one of the union fails, return `None`
            union = union.union(range)?;
        }

        // return the union
        Some(union)
    }

    fn merge(&self) -> Vec<Range<OffsetDateTime>> {
        // if slice is empty, return an empty vector
        if self.is_empty() {
            return vec![];
        }

        // find the start offset
        let start_offset = self[0].start.offset();

        // clone the range slice
        let mut ranges: Vec<Range<OffsetDateTime>> = self
            .iter()
            .filter(|range| range.start < range.end)
            .map(|range| range.start.to_offset(start_offset)..range.end.to_offset(start_offset))
            .collect();

        // sort the ranges
        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        // combine the ranges
        let (mut merged, last_option): (Vec<Range<OffsetDateTime>>, Option<Range<OffsetDateTime>>) =
            ranges.iter().fold(
                (vec![], None),
                |(mut partial_merge, current_option), range| {
                    if let Some(current) = current_option {
                        if range.start <= current.end {
                            (
                                partial_merge,
                                Some(current.start..current.end.max(range.end)),
                            )
                        } else {
                            partial_merge.push(current);
                            (partial_merge, Some(range.clone()))
                        }
                    } else {
                        (partial_merge, Some(range.clone()))
                    }
                },
            );

        // add the last range to the merged ranges if it exists
        if let Some(last) = last_option {
            merged.push(last);
        }

        // return the merged ranges
        merged
    }

    fn xor(&self) -> Vec<Range<OffsetDateTime>> {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Timestamp {
            Start(OffsetDateTime),
            End(OffsetDateTime),
        }

        // if slice is empty, return an empty vector
        if self.is_empty() {
            return vec![];
        }

        // find the start offset
        let start_offset = self[0].start.offset();

        // split the ranges into start and end timestamps
        let mut timestamps: Vec<Timestamp> = vec![];

        // keep only the ranges that are not reversed
        let ranges_iter = self
            .iter()
            .filter(|range| range.start < range.end)
            .map(|range| range.start.to_offset(start_offset)..range.end.to_offset(start_offset));

        for range in ranges_iter {
            timestamps.push(Timestamp::Start(range.start));
            timestamps.push(Timestamp::End(range.end));
        }

        // sort the timestamps by when they occur
        timestamps.sort_by(|a, b| {
            match (a, b) {
                (Timestamp::Start(a), Timestamp::Start(b)) => a.cmp(b),
                (Timestamp::End(a), Timestamp::End(b)) => a.cmp(b),
                (Timestamp::Start(a), Timestamp::End(b)) => a.cmp(b),
                (Timestamp::End(a), Timestamp::Start(b)) => a.cmp(b),
            }
        });

        let mut ranges = vec![];
        let mut count: usize = 0;
        let mut start: Option<OffsetDateTime> = None;

        for timestamp in timestamps {
            let current_timestamp = match timestamp {
                Timestamp::Start(date) => {
                    count += 1;
                    date
                }
                Timestamp::End(date) => {
                    count -= 1;
                    date
                }
            };

            if count == 1 {
                start = Some(current_timestamp);
            } else {
                if let Some(start_timestamp) = start {
                    if start_timestamp != current_timestamp {
                        ranges.push(start_timestamp..current_timestamp)
                    }

                    start = None;
                }
            }
        }

        ranges.merge()
    }
}
