//! Implementations of the [`arbitrary::Arbitrary`](arbitrary_dep::Arbitrary) trait.
//!
//! It is generally intended for use with fuzzing using AFL or libFuzzer, but can
//! also be used to generate random values for a data type
//!
//! ```
//! # use arbitrary_dep::{Arbitrary, Unstructured};
//! // use arbitrary::{Arbitrary, Unstructured};
//! use time::PrimitiveDateTime;
//!
//! # let get_input_from_fuzzer = || &[];
//! let raw_data: &[u8] = get_input_from_fuzzer();
//!
//! //Wrap it in an `Unstructured`.
//! let mut unstructured = Unstructured::new(raw_data);
//!
//! // Generate an `PrimitiveDateTime` and run our checks
//! if let Ok(datetime) = PrimitiveDateTime::arbitrary(&mut unstructured) {
//! #   let run_my_datetime_checks = |_| {};
//!     run_my_datetime_checks(datetime);
//! }
//! ```
//!
//! An implementation for `Instant` is intentionally omitted since its values are only meaningful in
//! relation to a [`Duration`], and obtaining an `Instant` from a [`Duration`] is very simple
//! anyway.

use arbitrary_dep::{size_hint, Arbitrary, Result, Unstructured};

use crate::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

impl<'a> Arbitrary<'a> for Date {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.int_in_range(Self::MIN.to_julian_day()..=Self::MAX.to_julian_day())
            .map(Self::from_julian_day_unchecked)
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        let n = core::mem::size_of::<i32>();
        (n, Some(n))
    }
}

impl<'a> Arbitrary<'a> for Duration {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.int_in_range(Self::MIN.whole_nanoseconds()..=Self::MAX.whole_nanoseconds())
            .map(Self::nanoseconds_i128)
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        size_hint::and(i64::size_hint(0), i32::size_hint(0))
    }
}

impl<'a> Arbitrary<'a> for Time {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let hour = u.int_in_range(0..=23)?;
        let minute = u.int_in_range(0..=60)?;
        let second = u.int_in_range(0..=60)?;
        let nanosecond = u.int_in_range(0..=999_999_999)?;
        Ok(Self::__from_hms_nanos_unchecked(
            hour, minute, second, nanosecond,
        ))
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        let _u8 = core::mem::size_of::<u8>();
        let _u32 = core::mem::size_of::<u32>();
        size_hint::and_all(&[
            (_u8, Some(_u8)),
            (_u8, Some(_u8)),
            (_u8, Some(_u8)),
            (_u32, Some(_u32)),
        ])
    }
}

impl<'a> Arbitrary<'a> for PrimitiveDateTime {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Self::new(Date::arbitrary(u)?, Time::arbitrary(u)?))
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        size_hint::and(Date::size_hint(0), Time::size_hint(0))
    }
}

const UTC_OFFSET_SECONDS_MIN: i32 = -86_399;
const UTC_OFFSET_SECONDS_MAX: i32 = 86_399;

impl<'a> Arbitrary<'a> for UtcOffset {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.int_in_range(UTC_OFFSET_SECONDS_MIN..=UTC_OFFSET_SECONDS_MAX)
            .map(|seconds| {
                Self::__from_hms_unchecked(
                    (seconds / 3600) as _,
                    ((seconds % 3600) / 60) as _,
                    (seconds % 60) as _,
                )
            })
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        let _i8 = core::mem::size_of::<i8>();
        size_hint::and_all(&[(_i8, Some(_i8)), (_i8, Some(_i8)), (_i8, Some(_i8))])
    }
}

impl<'a> Arbitrary<'a> for OffsetDateTime {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let datetime = PrimitiveDateTime::arbitrary(u)?;
        Ok(datetime.assume_offset(UtcOffset::arbitrary(u)?))
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        size_hint::and(PrimitiveDateTime::size_hint(0), UtcOffset::size_hint(0))
    }
}

impl<'a> Arbitrary<'a> for Weekday {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.choose(&[
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
            Weekday::Sunday,
        ])
        .map(|w| *w)
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        let n = core::mem::size_of::<u8>();
        (n, Some(n))
    }
}

impl<'a> Arbitrary<'a> for Month {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.choose(&[
            Month::January,
            Month::February,
            Month::March,
            Month::April,
            Month::May,
            Month::June,
            Month::July,
            Month::August,
            Month::September,
            Month::October,
            Month::November,
            Month::December,
        ])
        .map(|m| *m)
    }

    fn size_hint(_: usize) -> (usize, Option<usize>) {
        let n = core::mem::size_of::<u8>();
        (n, Some(n))
    }
}

#[cfg(test)]
mod test {
    use arbitrary_dep::{Arbitrary, Error, Unstructured};

    // Not really "arbitrary".  As the input data is not random, generated
    // data will be identical each run.  But we're not testing randomness,
    // we're testing for valid generated data.

    const DATA: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    #[test]
    fn test_arbitrary_duration() {
        let n = crate::Duration::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n]);
        let duration =
            crate::Duration::arbitrary(&mut u).expect("Unable to generate arbitrary Date");
        assert!(duration >= crate::Duration::MIN);
        assert!(duration <= crate::Duration::MAX);
    }

    #[test]
    fn test_arbitrary_duration_without_enough_data() {
        let n = crate::Duration::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n - 1]);
        match crate::Duration::arbitrary(&mut u) {
            Err(Error::NotEnoughData) => {}
            other => panic! {"Expected not enough data, got: {:?}", other},
        }
    }

    #[test]
    fn test_arbitrary_date() {
        let n = crate::Date::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n]);
        let date = crate::Date::arbitrary(&mut u).expect("Unable to generate arbitrary Date");
        assert!(date >= crate::Date::MIN);
        assert!(date <= crate::Date::MAX);
    }

    #[test]
    fn test_arbitrary_date_without_enough_data() {
        let n = crate::Date::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n - 1]);
        match crate::Date::arbitrary(&mut u) {
            Err(Error::NotEnoughData) => {}
            other => panic! {"Expected not enough data, got: {:?}", other},
        }
    }

    #[test]
    fn test_arbitrary_time() {
        let n = crate::Time::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n]);
        let time = crate::Time::arbitrary(&mut u).expect("Unable to generate arbitrary Time");
        assert!(time >= crate::Time::MIN);
        assert!(time <= crate::Time::MAX);
    }

    #[test]
    fn test_arbitrary_time_without_enough_data() {
        let n = crate::Time::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n - 1]);
        match crate::Time::arbitrary(&mut u) {
            Err(Error::NotEnoughData) => {}
            other => panic! {"Expected not enough data, got: {:?}", other},
        }
    }

    #[test]
    fn test_arbitrary_weekday() {
        let n = crate::Weekday::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n]);
        let _weekday =
            crate::Weekday::arbitrary(&mut u).expect("Unable to generate arbitrary Weekday");
    }

    #[test]
    fn test_arbitrary_weekday_without_enough_data() {
        let n = crate::Weekday::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n - 1]);
        match crate::Weekday::arbitrary(&mut u) {
            Err(Error::NotEnoughData) => {}
            other => panic! {"Expected not enough data, got: {:?}", other},
        }
    }

    #[test]
    fn test_arbitrary_month() {
        let n = crate::Month::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n]);
        let _month = crate::Month::arbitrary(&mut u).expect("Unable to generate arbitrary Month");
    }

    #[test]
    fn test_arbitrary_month_without_enough_data() {
        let n = crate::Month::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n - 1]);
        match crate::Month::arbitrary(&mut u) {
            Err(Error::NotEnoughData) => {}
            other => panic! {"Expected not enough data, got: {:?}", other},
        }
    }

    #[test]
    fn test_arbitrary_offset() {
        let n = crate::UtcOffset::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n]);
        let offset =
            crate::UtcOffset::arbitrary(&mut u).expect("Unable to generate arbitrary UtcOffset");
        assert!(offset.whole_seconds() >= super::UTC_OFFSET_SECONDS_MIN);
        assert!(offset.whole_seconds() <= super::UTC_OFFSET_SECONDS_MAX);
    }

    #[test]
    fn test_arbitrary_offset_without_enough_data() {
        let n = crate::UtcOffset::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n - 1]);
        match crate::UtcOffset::arbitrary(&mut u) {
            Err(Error::NotEnoughData) => {}
            other => panic! {"Expected not enough data, got: {:?}", other},
        }
    }

    #[test]
    fn test_arbitrary_primitivedatetime() {
        let n = crate::PrimitiveDateTime::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n]);
        let datetime = crate::PrimitiveDateTime::arbitrary(&mut u)
            .expect("Unable to generate arbitrary PrimitiveDateTime");
        let date = datetime.date();
        assert!(date >= crate::Date::MIN);
        assert!(date <= crate::Date::MAX);
        let time = datetime.time();
        assert!(time >= crate::Time::MIN);
        assert!(time <= crate::Time::MAX);
    }

    #[test]
    fn test_arbitrary_primitivedatetime_without_enough_data() {
        let n = crate::PrimitiveDateTime::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n - 1]);
        match crate::PrimitiveDateTime::arbitrary(&mut u) {
            Err(Error::NotEnoughData) => {}
            other => panic! {"Expected not enough data, got: {:?}", other},
        }
    }

    #[test]
    fn test_arbitrary_offsetdatetime() {
        let n = crate::OffsetDateTime::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n]);
        let datetime = crate::OffsetDateTime::arbitrary(&mut u)
            .expect("Unable to generate arbitrary OffsetDateTime");
        let date = datetime.date();
        assert!(date >= crate::Date::MIN);
        assert!(date <= crate::Date::MAX);
        let time = datetime.time();
        assert!(time >= crate::Time::MIN);
        assert!(time <= crate::Time::MAX);
        let offset = datetime.offset();
        assert!(offset.whole_seconds() >= super::UTC_OFFSET_SECONDS_MIN);
        assert!(offset.whole_seconds() <= super::UTC_OFFSET_SECONDS_MAX);
    }

    #[test]
    fn test_arbitrary_offsetdatetime_without_enough_data() {
        let n = crate::OffsetDateTime::size_hint(0).0;
        let mut u = Unstructured::new(&DATA[..n - 1]);
        match crate::OffsetDateTime::arbitrary(&mut u) {
            Err(Error::NotEnoughData) => {}
            other => panic! {"Expected not enough data, got: {:?}", other},
        }
    }
}