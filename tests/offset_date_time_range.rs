use time::macros::datetime;
use time::OffsetDateTimeRangeExt;
use time::OffsetDateTimeRangeSliceExt;
use time::Duration;


mod offset_date_time_range_ext {
    use super::*;

    #[test]
    fn len() {
        let start = datetime!(2022-04-18 03:17:12.200 UTC);
        let end = datetime!(2022-04-18 03:17:17.600 UTC);

        assert_eq!((start..end).len(Duration::SECOND), 5);
        assert_eq!((start..end).len(Duration::ZERO), 0);
        assert_eq!((start..end).len(-Duration::SECOND), 0);
        assert_eq!((end..start).len(Duration::SECOND), 0);
    }

    #[test]
    fn seconds() {
        let start = datetime!(2022-04-18 03:17:12.200 UTC);
        let end = datetime!(2022-04-18 03:17:17.600 UTC);
        let mut seconds = (start..end).seconds();

        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:13 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:14 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:15 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:16 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:17 UTC)));
        assert_eq!(seconds.next(), None);

        assert_eq!((end..start).seconds().next(), None);
    }

    #[test]
    fn minutes() {
        let start = datetime!(2022-04-18 03:17:17 UTC);
        let end = datetime!(2022-04-18 03:19:42 UTC);
        let mut minutes = (start..end).minutes();
        assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:18:00 UTC)));
        assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:19:00 UTC)));
        assert_eq!(minutes.next(), None);

        assert_eq!((end..start).minutes().next(), None);
    }

    #[test]
    fn hours() {
        let start = datetime!(2022-04-18 03:17:00 UTC);
        let end = datetime!(2022-04-18 07:42:00 UTC);
        let mut hours = (start..end).hours();
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 04:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 05:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 06:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 07:00:00 UTC)));
        assert_eq!(hours.next(), None);
        
        assert_eq!((end..start).hours().next(), None);
    }

    #[test]
    fn days() {
        let start = datetime!(2022-04-18 04:00:00 UTC);
        let end = datetime!(2022-04-20 08:00:00 UTC);
        let mut days = (start..end).days();

        assert_eq!(days.next(), Some(datetime!(2022-04-19 00:00:00 UTC)));
        assert_eq!(days.next(), Some(datetime!(2022-04-20 00:00:00 UTC)));
        assert_eq!(days.next(), None);
        
        assert_eq!((end..start).days().next(), None);
    }

    #[test]
    fn monday_based_weeks() {
        let start = datetime!(2022-04-07 00:00:00 UTC);
        let end = datetime!(2022-04-21 00:00:00 UTC);
        let mut weeks = (start..end).monday_based_weeks();

        assert_eq!(weeks.next(), Some(datetime!(2022-04-11 00:00:00 UTC)));
        assert_eq!(weeks.next(), Some(datetime!(2022-04-18 00:00:00 UTC)));
        assert_eq!(weeks.next(), None);
        
        assert_eq!((end..start).monday_based_weeks().next(), None);
    }

    #[test]
    fn sunday_based_weeks() {
        let start = datetime!(2022-04-07 00:00:00 UTC);
        let end = datetime!(2022-04-21 00:00:00 UTC);
        let mut weeks = (start..end).sunday_based_weeks();
        
        assert_eq!(weeks.next(), Some(datetime!(2022-04-10 00:00:00 UTC)));
        assert_eq!(weeks.next(), Some(datetime!(2022-04-17 00:00:00 UTC)));
        assert_eq!(weeks.next(), None);
        
        assert_eq!((end..start).sunday_based_weeks().next(), None);
    }

    #[test]
    fn months() {
        let start = datetime!(2022-04-13 00:00:00 UTC);
        let end = datetime!(2022-06-26 00:00:00 UTC);
        let mut months = (start..end).months();

        assert_eq!(months.next(), Some(datetime!(2022-05-01 00:00:00 UTC)));
        assert_eq!(months.next(), Some(datetime!(2022-06-01 00:00:00 UTC)));
        assert_eq!(months.next(), None);
        
        assert_eq!((end..start).months().next(), None);
    }

    #[test]
    fn years() {
        let start = datetime!(2020-06-01 00:00:00 UTC);
        let end = datetime!(2022-09-01 00:00:00 UTC);
        let mut years = (start..end).years();

        assert_eq!(years.next(), Some(datetime!(2021-01-01 00:00:00 UTC)));
        assert_eq!(years.next(), Some(datetime!(2022-01-01 00:00:00 UTC)));
        assert_eq!(years.next(), None);
        
        assert_eq!((end..start).years().next(), None);
    }

    #[test]
    fn full_seconds() {
        let start = datetime!(2022-04-18 03:17:12.200 UTC);
        let end = datetime!(2022-04-18 03:17:17.600 UTC);
        let mut seconds = (start..end).full_seconds();

        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:13 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:14 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:15 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:16 UTC)));
        assert_eq!(seconds.next(), None);

        assert_eq!((end..start).full_seconds().next(), None);
    }

    #[test]
    fn full_minutes() {
        let start = datetime!(2022-04-18 03:17:17 UTC);
        let end = datetime!(2022-04-18 03:19:42 UTC);
        let mut minutes = (start..end).full_minutes();

        assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:18:00 UTC)));
        assert_eq!(minutes.next(), None);

        assert_eq!((end..start).full_minutes().next(), None);
    }

    #[test]
    fn full_hours() {
        let start = datetime!(2022-04-18 03:17:00 UTC);
        let end = datetime!(2022-04-18 07:42:00 UTC);
        let mut hours = (start..end).full_hours();

        assert_eq!(hours.next(), Some(datetime!(2022-04-18 04:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 05:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 06:00:00 UTC)));
        assert_eq!(hours.next(), None);

        assert_eq!((end..start).full_hours().next(), None);
    }

    #[test]
    fn full_days() {
        let start = datetime!(2022-04-18 04:00:00 UTC);
        let end = datetime!(2022-04-20 08:00:00 UTC);
        let mut days = (start..end).full_days();

        assert_eq!(days.next(), Some(datetime!(2022-04-19 00:00:00 UTC)));
        assert_eq!(days.next(), None);

        assert_eq!((end..start).full_days().next(), None);
    }

    #[test]
    fn full_monday_based_weeks() {
        let start = datetime!(2022-04-7 00:00:00 UTC);
        let end = datetime!(2022-04-21 00:00:00 UTC);
        let mut weeks = (start..end).full_monday_based_weeks();

        assert_eq!(weeks.next(), Some(datetime!(2022-04-11 00:00:00 UTC)));
        assert_eq!(weeks.next(), None);

        assert_eq!((end..start).full_monday_based_weeks().next(), None);
    }

    #[test]
    fn full_sunday_based_weeks() {
        let start = datetime!(2022-04-7 00:00:00 UTC);
        let end = datetime!(2022-04-21 00:00:00 UTC);
        let mut weeks = (start..end).full_sunday_based_weeks();

        assert_eq!(weeks.next(), Some(datetime!(2022-04-10 00:00:00 UTC)));
        assert_eq!(weeks.next(), None);

        assert_eq!((end..start).full_sunday_based_weeks().next(), None);
    }

    #[test]
    fn full_months() {
        let start = datetime!(2022-04-13 00:00:00 UTC);
        let end = datetime!(2022-06-26 00:00:00 UTC);
        let mut months = (start..end).full_months();

        assert_eq!(months.next(), Some(datetime!(2022-05-01 00:00:00 UTC)));
        assert_eq!(months.next(), None);

        assert_eq!((end..start).full_months().next(), None);
    }

    #[test]
    fn full_years() {
        let start = datetime!(2020-06-01 00:00:00 UTC);
        let end = datetime!(2022-09-01 00:00:00 UTC);
        let mut years = (start..end).full_years();

        assert_eq!(years.next(), Some(datetime!(2021-01-01 00:00:00 UTC)));
        assert_eq!(years.next(), None);

        assert_eq!((end..start).full_years().next(), None);
    }

    #[test]
    fn overlapping_seconds() {
        let start = datetime!(2022-04-18 03:17:12.200 UTC);
        let end = datetime!(2022-04-18 03:17:17.600 UTC);
        let mut seconds = (start..end).overlapping_seconds();

        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:12 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:13 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:14 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:15 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:16 UTC)));
        assert_eq!(seconds.next(), Some(datetime!(2022-04-18 03:17:17 UTC)));
        assert_eq!(seconds.next(), None);

        assert_eq!((end..start).overlapping_seconds().next(), None);
    }

    #[test]
    fn overlapping_minutes() {
        let start = datetime!(2022-04-18 03:17:17 UTC);
        let end = datetime!(2022-04-18 03:19:42 UTC);
        let mut minutes = (start..end).overlapping_minutes();

        assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:17:00 UTC)));
        assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:18:00 UTC)));
        assert_eq!(minutes.next(), Some(datetime!(2022-04-18 03:19:00 UTC)));
        assert_eq!(minutes.next(), None);

        assert_eq!((end..start).overlapping_minutes().next(), None);
    }

    #[test]
    fn overlapping_hours() {
        let start = datetime!(2022-04-18 03:17:00 UTC);
        let end = datetime!(2022-04-18 07:42:00 UTC);
        let mut hours = (start..end).overlapping_hours();

        assert_eq!(hours.next(), Some(datetime!(2022-04-18 03:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 04:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 05:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 06:00:00 UTC)));
        assert_eq!(hours.next(), Some(datetime!(2022-04-18 07:00:00 UTC)));
        assert_eq!(hours.next(), None);

        assert_eq!((end..start).overlapping_hours().next(), None);
    }

    #[test]
    fn overlapping_days() {
        let start = datetime!(2022-04-18 04:00:00 UTC);
        let end = datetime!(2022-04-20 08:00:00 UTC);
        let mut days = (start..end).overlapping_days();

        assert_eq!(days.next(), Some(datetime!(2022-04-18 00:00:00 UTC)));
        assert_eq!(days.next(), Some(datetime!(2022-04-19 00:00:00 UTC)));
        assert_eq!(days.next(), Some(datetime!(2022-04-20 00:00:00 UTC)));
        assert_eq!(days.next(), None);

        assert_eq!((end..start).overlapping_days().next(), None);
    }

    #[test]
    fn overlapping_monday_based_weeks() {
        let start = datetime!(2022-04-07 00:00:00 UTC);
        let end = datetime!(2022-04-21 00:00:00 UTC);
        let mut weeks = (start..end).overlapping_monday_based_weeks();

        assert_eq!(weeks.next(), Some(datetime!(2022-04-04 00:00:00 UTC)));
        assert_eq!(weeks.next(), Some(datetime!(2022-04-11 00:00:00 UTC)));
        assert_eq!(weeks.next(), Some(datetime!(2022-04-18 00:00:00 UTC)));
        assert_eq!(weeks.next(), None);

        assert_eq!((end..start).overlapping_monday_based_weeks().next(), None);
    }

    #[test]
    fn overlapping_sunday_based_weeks() {
        let start = datetime!(2022-04-07 00:00:00 UTC);
        let end = datetime!(2022-04-21 00:00:00 UTC);
        let mut weeks = (start..end).overlapping_sunday_based_weeks();

        assert_eq!(weeks.next(), Some(datetime!(2022-04-03 00:00:00 UTC)));
        assert_eq!(weeks.next(), Some(datetime!(2022-04-10 00:00:00 UTC)));
        assert_eq!(weeks.next(), Some(datetime!(2022-04-17 00:00:00 UTC)));
        assert_eq!(weeks.next(), None);

        assert_eq!((end..start).overlapping_sunday_based_weeks().next(), None);
    }

    #[test]
    fn overlapping_months() {
        let start = datetime!(2022-04-13 00:00:00 UTC);
        let end = datetime!(2022-06-26 00:00:00 UTC);
        let mut months = (start..end).overlapping_months();

        assert_eq!(months.next(), Some(datetime!(2022-04-01 00:00:00 UTC)));
        assert_eq!(months.next(), Some(datetime!(2022-05-01 00:00:00 UTC)));
        assert_eq!(months.next(), Some(datetime!(2022-06-01 00:00:00 UTC)));
        assert_eq!(months.next(), None);

        assert_eq!((end..start).overlapping_months().next(), None);
    }

    #[test]
    fn overlapping_years() {
        let start = datetime!(2020-06-01 00:00:00 UTC);
        let end = datetime!(2022-09-01 00:00:00 UTC);
        let mut years = (start..end).overlapping_years();

        assert_eq!(years.next(), Some(datetime!(2020-01-01 00:00:00 UTC)));
        assert_eq!(years.next(), Some(datetime!(2021-01-01 00:00:00 UTC)));
        assert_eq!(years.next(), Some(datetime!(2022-01-01 00:00:00 UTC)));
        assert_eq!(years.next(), None);

        assert_eq!((end..start).overlapping_years().next(), None);
    }

    #[test]
    fn overlaps() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(range1.clone().overlaps(range2.clone()));
        assert!(range2.clone().overlaps(range1.clone()));
        
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-03 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(range1.clone().overlaps(range2.clone()));
        assert!(range2.clone().overlaps(range1.clone()));
        
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().overlaps(range2.clone()));
        assert!(!range2.clone().overlaps(range1.clone()));
        
        let range1 = datetime!(2022-04-10 00:00:00 UTC)..datetime!(2022-04-06 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().overlaps(range2.clone()));
        assert!(!range2.clone().overlaps(range1.clone()));
    }

    #[test]
    fn left_adjacent_to() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-10 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(range1.clone().left_adjacent_to(range2.clone()));
        assert!(!range2.clone().left_adjacent_to(range1.clone()));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().left_adjacent_to(range2.clone()));
        assert!(!range2.clone().left_adjacent_to(range1.clone()));
    }

    #[test]
    fn right_adjacent_to() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-10 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().right_adjacent_to(range2.clone()));
        assert!(range2.clone().right_adjacent_to(range1.clone()));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().right_adjacent_to(range2.clone()));
        assert!(!range2.clone().right_adjacent_to(range1.clone()));
    }

    #[test]
    fn engulfs() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(range1.clone().engulfs(range2.clone()));
        assert!(!range2.clone().engulfs(range1.clone()));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range2 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(range1.clone().engulfs(range2.clone()));
        assert!(range2.clone().engulfs(range1.clone()));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-06 00:00:00 UTC);
        let range2 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().engulfs(range2.clone()));
        assert!(!range2.clone().engulfs(range1.clone()));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().engulfs(range2.clone()));
        assert!(!range2.clone().engulfs(range1.clone()));
    }

    #[test]
    fn engulfed_by() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().engulfed_by(range2.clone()));
        assert!(range2.clone().engulfed_by(range1.clone()));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range2 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(range1.clone().engulfed_by(range2.clone()));
        assert!(range2.clone().engulfed_by(range1.clone()));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-06 00:00:00 UTC);
        let range2 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().engulfed_by(range2.clone()));
        assert!(!range2.clone().engulfed_by(range1.clone()));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert!(!range1.clone().engulfed_by(range2.clone()));
        assert!(!range2.clone().engulfed_by(range1.clone()));
    }

    #[test]
    fn split_at() {
        let range = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let split = datetime!(2022-04-08 00:00:00 UTC);

        let (left, right) = range.split_at(split).unwrap();

        assert_eq!(left, datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-08 00:00:00 UTC));
        assert_eq!(right, datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC));
        
        let range = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let split = datetime!(2022-04-18 00:00:00 UTC);

        assert_eq!(range.split_at(split), None);
        
        let range = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let split = datetime!(2022-04-06 00:00:00 UTC);

        assert_eq!(range.split_at(split), None);
        
        let range = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-06 00:00:00 UTC);
        let split = datetime!(2022-04-06 00:00:00 UTC);

        assert_eq!(range.split_at(split), None);
    }

    #[test]
    fn split_at_multiple() {
        let range = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let splits = vec![
            datetime!(2022-04-07 00:00:00 UTC),
            datetime!(2022-04-08 00:00:00 UTC),
            datetime!(2022-04-09 00:00:00 UTC),
            datetime!(2022-04-15 00:00:00 UTC),
        ];
        let ranges = range.split_at_multiple(&splits);

        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0], datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-07 00:00:00 UTC));
        assert_eq!(ranges[1], datetime!(2022-04-07 00:00:00 UTC)..datetime!(2022-04-08 00:00:00 UTC));
        assert_eq!(ranges[2], datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-09 00:00:00 UTC));
        assert_eq!(ranges[3], datetime!(2022-04-09 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC));

        let range = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-06 00:00:00 UTC);
        let splits = vec![
            datetime!(2022-04-07 00:00:00 UTC),
        ];
        let ranges = range.split_at_multiple(&splits);

        assert_eq!(ranges.len(), 0);
    }

    #[test]
    fn split_by() {
        let range = datetime!(2022-04-06 14:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let mut split = range.split_by(Duration::days(1));

        assert_eq!(split.next(), Some(datetime!(2022-04-06 14:00:00 UTC)..datetime!(2022-04-07 14:00:00 UTC)));
        assert_eq!(split.next(), Some(datetime!(2022-04-07 14:00:00 UTC)..datetime!(2022-04-08 14:00:00 UTC)));
        assert_eq!(split.next(), Some(datetime!(2022-04-08 14:00:00 UTC)..datetime!(2022-04-09 14:00:00 UTC)));
        assert_eq!(split.next(), Some(datetime!(2022-04-09 14:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC)));
        assert_eq!(split.next(), None);
    }

    #[test]
    fn divide_equally() {
        let range = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00.000_000_002 UTC);
        let mut split = range.divide_equally(4);

        assert_eq!(split.next(), Some(datetime!(2022-04-06 00:00:00.000_000_000 UTC)..datetime!(2022-04-07 00:00:00.000_000_001 UTC)));
        assert_eq!(split.next(), Some(datetime!(2022-04-07 00:00:00.000_000_001 UTC)..datetime!(2022-04-08 00:00:00.000_000_002 UTC)));
        assert_eq!(split.next(), Some(datetime!(2022-04-08 00:00:00.000_000_002 UTC)..datetime!(2022-04-09 00:00:00.000_000_002 UTC)));
        assert_eq!(split.next(), Some(datetime!(2022-04-09 00:00:00.000_000_002 UTC)..datetime!(2022-04-10 00:00:00.000_000_002 UTC)));
        assert_eq!(split.next(), None);
    }

    #[test]
    fn intersection() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert_eq!(range1.clone().intersection(range2.clone()), Some(datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC)));
        assert_eq!(range2.clone().intersection(range1.clone()), Some(datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC)));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert_eq!(range1.clone().intersection(range2.clone()), None);
        assert_eq!(range2.clone().intersection(range1.clone()), None);
    }

    #[test]
    fn union() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert_eq!(range1.clone().union(range2.clone()), Some(datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC)));
        assert_eq!(range2.clone().union(range1.clone()), Some(datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC)));

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert_eq!(range1.clone().union(range2.clone()), None);
        assert_eq!(range2.clone().union(range1.clone()), None);
    }

    #[test]
    fn difference() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert_eq!(range1.clone().difference(range2.clone()), vec![datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-08 00:00:00 UTC)]);
        assert_eq!(range2.clone().difference(range1.clone()), vec![datetime!(2022-04-10 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC)]);

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert_eq!(range1.clone().difference(range2.clone()), vec![datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC)]);
        assert_eq!(range2.clone().difference(range1.clone()), vec![datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC)]);

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert_eq!(range1.clone().difference(range2.clone()), vec![
            datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-08 00:00:00 UTC),
            datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC),
        ]);
        assert_eq!(range2.clone().difference(range1.clone()), vec![]);
    }

    #[test]
    fn difference_multiple() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range3 = datetime!(2022-04-10 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        // ranges can't be copied, so we are using `clone()` here
        assert_eq!(
            range1.clone().difference_multiple(&vec![range2.clone(), range3.clone()]),
            vec![
                datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-08 00:00:00 UTC),
                datetime!(2022-04-13 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC),
            ]
        );
    }
}

mod offset_date_time_range_slice_ext {
    use super::*;

    #[test]
    fn intersection() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range3 = datetime!(2022-04-09 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        assert_eq!(
            [range1, range2, range3].intersection(),
            Some(datetime!(2022-04-09 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC))
        );

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-13 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC);

        assert_eq!(
            [range1, range2].intersection(),
            None
        );
    }

    #[test]
    fn union() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range3 = datetime!(2022-04-09 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);

        assert_eq!(
            [range1, range2, range3].union(),
            Some(datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC))
        );

        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-13 00:00:00 UTC)..datetime!(2022-04-15 00:00:00 UTC);

        assert_eq!(
            [range1, range2].union(),
            None
        );
    }

    #[test]
    fn merge() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range3 = datetime!(2022-04-09 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);
        let range4 = datetime!(2022-04-15 00:00:00 UTC)..datetime!(2022-04-19 00:00:00 UTC);
        let range5 = datetime!(2022-04-15 00:00:00 UTC)..datetime!(2022-04-17 00:00:00 UTC);
        let range6 = datetime!(2022-04-19 00:00:00 UTC)..datetime!(2022-04-20 00:00:00 UTC);
        
        assert_eq!(
            [range1, range2, range3, range4, range5, range6].merge(),
            vec![
                datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC),
                datetime!(2022-04-15 00:00:00 UTC)..datetime!(2022-04-20 00:00:00 UTC),
            ]
        );
    }

    #[test]
    fn xor() {
        let range1 = datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-10 00:00:00 UTC);
        let range2 = datetime!(2022-04-08 00:00:00 UTC)..datetime!(2022-04-12 00:00:00 UTC);
        let range3 = datetime!(2022-04-09 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC);
        let range4 = datetime!(2022-04-15 00:00:00 UTC)..datetime!(2022-04-19 00:00:00 UTC);
        let range5 = datetime!(2022-04-15 00:00:00 UTC)..datetime!(2022-04-17 00:00:00 UTC);
        
        assert_eq!(
            [range1, range2, range3, range4, range5].xor(),
            vec![
                datetime!(2022-04-06 00:00:00 UTC)..datetime!(2022-04-08 00:00:00 UTC),
                datetime!(2022-04-12 00:00:00 UTC)..datetime!(2022-04-13 00:00:00 UTC),
                datetime!(2022-04-17 00:00:00 UTC)..datetime!(2022-04-19 00:00:00 UTC),
            ]
        );
    }
}
