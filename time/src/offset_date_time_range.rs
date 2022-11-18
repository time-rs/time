//! Implementations of useful methods for the [`Range<OffsetDateTime>`] and [`&[Range<OffsetDateTime>]`](https://doc.rust-lang.org/std/primitive.slice.html) structs.

use std::ops::Range;
use crate::{OffsetDateTime, Duration, util, Month, Weekday};
use iter::*;

/// Extension of [`Range<OffsetDateTime>`] containing useful methods.
/// 
/// If you want to access these methods, you must import the [`OffsetDateTimeRangeExt`] trait.
pub trait OffsetDateTimeRangeExt {
    /// Returns the length of the range using the specified duration as the unit.
    /// 
    /// Attention, using `Duration::DAY` as unit wouldn't count how many days are in the range, but rather how many day-long ranges would fits inside.
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

/// Implementation of the `OffsetDateTimeRange` trait for `Range<OffsetDateTime>`.
impl OffsetDateTimeRangeExt for Range<OffsetDateTime> {

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// # use time::Duration;
    /// let start = datetime!(2022-04-18 03:17:12.200 UTC);
    /// let end = datetime!(2022-04-18 03:17:17.600 UTC);
    /// assert_eq!((start..end).len(Duration::SECOND), 5);
    /// ```
    fn len(self, unit: Duration) -> usize {
        // if the range start is after the range end, the range is empty
        // and we return 0
        if self.end <= self.start {
            0
        }
        
        // if the range isn't empty, then the duration is positive
        else {
            let range_duration = (self.end - self.start).whole_nanoseconds() as u128;
            let unit_duration = unit.whole_nanoseconds() as u128;
            (range_duration / unit_duration) as usize
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:12.200 UTC);
    /// let end = datetime!(2022-04-18 03:17:17.600 UTC);
    /// let mut seconds = (start..end).seconds();
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:13 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:14 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:15 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:16 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:17 UTC)));
    /// assert_eq!(seconds.next(), None);
    /// ```
    fn seconds(self) -> Seconds {
        Seconds {
            current: self.start.ceil_seconds(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:17 UTC);
    /// let end = datetime!(2022-04-18 03:19:42 UTC);
    /// let mut minutes = (start..end).minutes();
    /// assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:18:00 UTC)));
    /// assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:19:00 UTC)));
    /// assert_eq!(minutes.next(), None);
    /// ```
    fn minutes(self) -> Minutes {
        Minutes {
            current: self.start.ceil_minutes(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:00 UTC);
    /// let end = datetime!(2022-04-18 07:42:00 UTC);
    /// let mut hours = (start..end).hours();
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 04:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 05:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 06:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 07:00:00 UTC)));
    /// assert_eq!(hours.next(), None);
    /// ```
    fn hours(self) -> Hours {
        Hours {
            current: self.start.ceil_hours(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 04:00:00 UTC);
    /// let end = datetime!(2022-04-20 08:00:00 UTC);
    /// let mut days = (start..end).days();
    /// assert_eq!(days.next(), Some(datetime!(2022-04-19 00:00:00 UTC)));
    /// assert_eq!(days.next(), Some(datetime!(2022-04-20 00:00:00 UTC)));
    /// assert_eq!(days.next(), None);
    /// ```
    fn days(self) -> Days {
        Days {
            current: self.start.ceil_days(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-07 00:00:00 UTC);
    /// let end = datetime!(2022-04-21 00:00:00 UTC);
    /// let mut weeks = (start..end).monday_based_weeks();
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-11 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-18 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), None);
    /// ```
    fn monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.ceil_monday_based_weeks(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-07 00:00:00 UTC);
    /// let end = datetime!(2022-04-21 00:00:00 UTC);
    /// let mut weeks = (start..end).sunday_based_weeks();
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-10 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-17 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), None);
    /// ```
    fn sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.ceil_sunday_based_weeks(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-13 00:00:00 UTC);
    /// let end = datetime!(2022-06-26 00:00:00 UTC);
    /// let mut months = (start..end).months();
    /// assert_eq!(months.next(), Some(datetime!(2022-05-01 00:00:00 UTC)));
    /// assert_eq!(months.next(), Some(datetime!(2022-06-01 00:00:00 UTC)));
    /// assert_eq!(months.next(), None);
    /// ```
    fn months(self) -> Months {
        Months {
            current: self.start.ceil_months(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2020-06-01 00:00:00 UTC);
    /// let end = datetime!(2022-09-01 00:00:00 UTC);
    /// let mut years = (start..end).years();
    /// assert_eq!(years.next(), Some(datetime!(2021-01-01 00:00:00 UTC)));
    /// assert_eq!(years.next(), Some(datetime!(2022-01-01 00:00:00 UTC)));
    /// assert_eq!(years.next(), None);
    /// ```
    fn years(self) -> Years {
        Years {
            current: self.start.ceil_years(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:12.200 UTC);
    /// let end = datetime!(2022-04-18 03:17:17.600 UTC);
    /// let mut seconds = (start..end).full_seconds();
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:13 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:14 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:15 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:16 UTC)));
    /// assert_eq!(seconds.next(), None);
    /// ```
    fn full_seconds(self) -> Seconds {
        Seconds {
            current: self.start.ceil_seconds(),
            end: self.end.floor_seconds(),
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:17 UTC);
    /// let end = datetime!(2022-04-18 03:19:42 UTC);
    /// let mut minutes = (start..end).full_minutes();
    /// assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:18:00 UTC)));
    /// assert_eq!(minutes.next(), None);
    /// ```
    fn full_minutes(self) -> Minutes {
        Minutes {
            current: self.start.ceil_minutes(),
            end: self.end.floor_minutes(),
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:00 UTC);
    /// let end = datetime!(2022-04-18 07:42:00 UTC);
    /// let mut hours = (start..end).full_hours();
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 04:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 05:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 06:00:00 UTC)));
    /// assert_eq!(hours.next(), None);
    /// ```
    fn full_hours(self) -> Hours {
        Hours {
            current: self.start.ceil_hours(),
            end: self.end.floor_hours(),
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 04:00:00 UTC);
    /// let end = datetime!(2022-04-20 08:00:00 UTC);
    /// let mut days = (start..end).full_days();
    /// assert_eq!(days.next(), Some(datetime!(2022-04-19 00:00:00 UTC)));
    /// assert_eq!(days.next(), None);
    /// ```
    fn full_days(self) -> Days {
        Days {
            current: self.start.ceil_days(),
            end: self.end.floor_days(),
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-7 00:00:00 UTC);
    /// let end = datetime!(2022-04-21 00:00:00 UTC);
    /// let mut weeks = (start..end).full_monday_based_weeks();
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-11 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), None);
    /// ```
    fn full_monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.ceil_monday_based_weeks(),
            end: self.end.floor_monday_based_weeks(),
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-7 00:00:00 UTC);
    /// let end = datetime!(2022-04-21 00:00:00 UTC);
    /// let mut weeks = (start..end).full_sunday_based_weeks();
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-10 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), None);
    /// ```
    fn full_sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.ceil_sunday_based_weeks(),
            end: self.end.floor_sunday_based_weeks(),
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-13 00:00:00 UTC);
    /// let end = datetime!(2022-06-26 00:00:00 UTC);
    /// let mut months = (start..end).full_months();
    /// assert_eq!(months.next(), Some(datetime!(2022-05-01 00:00:00 UTC)));
    /// assert_eq!(months.next(), None);
    /// ```
    fn full_months(self) -> Months {
        Months {
            current: self.start.ceil_months(),
            end: self.end.floor_months(),
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2020-06-01 00:00:00 UTC);
    /// let end = datetime!(2022-09-01 00:00:00 UTC);
    /// let mut years = (start..end).full_years();
    /// assert_eq!(years.next(), Some(datetime!(2021-01-01 00:00:00 UTC)));
    /// assert_eq!(years.next(), None);
    /// ```
    fn full_years(self) -> Years {
        Years {
            current: self.start.ceil_years(),
            end: self.end.floor_years(),
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:12.200 UTC);
    /// let end = datetime!(2022-04-18 03:17:17.600 UTC);
    /// let mut seconds = (start..end).overlapping_seconds();
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:12 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:13 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:14 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:15 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:16 UTC)));
    /// assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:17 UTC)));
    /// assert_eq!(seconds.next(), None);
    /// ```
    fn overlapping_seconds(self) -> Seconds {
        Seconds {
            current: self.start.floor_seconds(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:17 UTC);
    /// let end = datetime!(2022-04-18 03:19:42 UTC);
    /// let mut minutes = (start..end).overlapping_minutes();
    /// assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:17:00 UTC)));
    /// assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:18:00 UTC)));
    /// assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:19:00 UTC)));
    /// assert_eq!(minutes.next(), None);
    /// ```
    fn overlapping_minutes(self) -> Minutes {
        Minutes {
            current: self.start.floor_minutes(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 03:17:00 UTC);
    /// let end = datetime!(2022-04-18 07:42:00 UTC);
    /// let mut hours = (start..end).overlapping_hours();
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 03:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 04:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 05:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 06:00:00 UTC)));
    /// assert_eq!(hours.next(), Some(datetime!(2022-04-18 07:00:00 UTC)));
    /// assert_eq!(hours.next(), None);
    /// ```
    fn overlapping_hours(self) -> Hours {
        Hours {
            current: self.start.floor_hours(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-18 04:00:00 UTC);
    /// let end = datetime!(2022-04-20 08:00:00 UTC);
    /// let mut days = (start..end).overlapping_days();
    /// assert_eq!(days.next(), Some(datetime!(2022-04-18 00:00:00 UTC)));
    /// assert_eq!(days.next(), Some(datetime!(2022-04-19 00:00:00 UTC)));
    /// assert_eq!(days.next(), Some(datetime!(2022-04-20 00:00:00 UTC)));
    /// assert_eq!(days.next(), None);
    /// ```
    fn overlapping_days(self) -> Days {
        Days {
            current: self.start.floor_days(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-07 00:00:00 UTC);
    /// let end = datetime!(2022-04-21 00:00:00 UTC);
    /// let mut weeks = (start..end).overlapping_monday_based_weeks();
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-04 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-11 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-18 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), None);
    /// ```
    fn overlapping_monday_based_weeks(self) -> MondayBasedWeeks {
        MondayBasedWeeks {
            current: self.start.floor_monday_based_weeks(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-07 00:00:00 UTC);
    /// let end = datetime!(2022-04-21 00:00:00 UTC);
    /// let mut weeks = (start..end).overlapping_sunday_based_weeks();
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-03 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-10 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), Some(datetime!(2022-04-17 00:00:00 UTC)));
    /// assert_eq!(weeks.next(), None);
    /// ```
    fn overlapping_sunday_based_weeks(self) -> SundayBasedWeeks {
        SundayBasedWeeks {
            current: self.start.floor_sunday_based_weeks(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2022-04-13 00:00:00 UTC);
    /// let end = datetime!(2022-06-26 00:00:00 UTC);
    /// let mut months = (start..end).overlapping_months();
    /// assert_eq!(months.next(), Some(datetime!(2022-04-01 00:00:00 UTC)));
    /// assert_eq!(months.next(), Some(datetime!(2022-05-01 00:00:00 UTC)));
    /// assert_eq!(months.next(), Some(datetime!(2022-06-01 00:00:00 UTC)));
    /// assert_eq!(months.next(), None);
    /// ```
    fn overlapping_months(self) -> Months {
        Months {
            current: self.start.floor_months(),
            end: self.end,
        }
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let start = datetime!(2020-06-01 00:00:00 UTC);
    /// let end = datetime!(2022-09-01 00:00:00 UTC);
    /// let mut years = (start..end).overlapping_years();
    /// assert_eq!(years.next(), Some(datetime!(2020-01-01 00:00:00 UTC)));
    /// assert_eq!(years.next(), Some(datetime!(2021-01-01 00:00:00 UTC)));
    /// assert_eq!(years.next(), Some(datetime!(2022-01-01 00:00:00 UTC)));
    /// assert_eq!(years.next(), None);
    /// ```
    fn overlapping_years(self) -> Years {
        Years {
            current: self.start.floor_years(),
            end: self.end,
        }
    }

    /// Here `range1` and `range2` overlap between the `2022-04-08 UTC` and the `2022-04-10 UTC`.
    /// 
    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
    /// let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);
    /// // ranges can't be copied, so we are using `clone()` here
    /// assert!(range1.clone().overlaps(range2.clone()));
    /// assert!(range2.clone().overlaps(range1.clone()));
    /// ```
    /// 
    /// Overlap, as `range1` is contained within `range2`.
    /// 
    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
    /// let range2 = datetime!(2022-04-03 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);
    /// // ranges can't be copied, so we are using `clone()` here
    /// assert!(range1.clone().overlaps(range2.clone()));
    /// assert!(range2.clone().overlaps(range1.clone()));
    /// ```
    /// 
    /// No overlap here, there is a gap between the two ranges.
    /// 
    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
    /// let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);
    /// // ranges can't be copied, so we are using `clone()` here
    /// assert!(!range1.clone().overlaps(range2.clone()));
    /// assert!(!range2.clone().overlaps(range1.clone()));
    /// ```
    /// 
    /// Negative ranges dont't overlap.
    /// 
    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let range1 = datetime!(2022-04-10 00:00:00 UTC)..datetime!(2022-04-06 00:00:00 UTC);
    /// let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);
    /// // ranges can't be copied, so we are using `clone()` here
    /// assert!(!range1.clone().overlaps(range2.clone()));
    /// assert!(!range2.clone().overlaps(range1.clone()));
    /// ```
    fn overlaps(self, other: Range<OffsetDateTime>) -> bool {
        self.start < self.end && other.start < other.end && self.start < other.end && other.start < self.end
    }

    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
    /// let range2 = datetime!(2022-04-10 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);
    /// // ranges can't be copied, so we are using `clone()` here
    /// assert!(range1.clone().left_adjacent_to(range2.clone()));
    /// assert!(!range2.clone().left_adjacent_to(range1.clone()));
    /// ```
    /// 
    /// ```rust
    /// # use time::macros::datetime;
    /// # use time::OffsetDateTimeRangeExt;
    /// let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
    /// let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);
    /// // ranges can't be copied, so we are using `clone()` here
    /// assert!(!range1.clone().left_adjacent_to(range2.clone()));
    /// assert!(!range2.clone().left_adjacent_to(range1.clone()));
    /// ```
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
            self.end = self.end.to_offset(start_offset);
            date = date.to_offset(start_offset);
            
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
            .filter(|date| self.contains(date))
            // filter out dates that are the same as the start or end of the range
            .filter(|date| *date != self.start && *date != self.end)
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
        SplitDuration {
            current: self.start,
            end: self.end,
            unit,
        }
    }

    fn divide_equally(self, number_of_parts: usize) -> SplitCount {
        SplitCount {
            current: self.start,
            end: self.end,
            number_of_parts,
        }
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
        todo!()
    }

    fn difference_multiple(self, others: &[Range<OffsetDateTime>]) -> Vec<Range<OffsetDateTime>> {
        todo!()
    }
}
