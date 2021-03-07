//! Tests for internal details.
//!
//! This module should only be used when it is not possible to test the implementation in a
//! reasonable manner externally.

use crate::formatting::DigitCount;
use crate::util::days_in_year_month;

#[test]
fn digit_count() {
    assert_eq!(1_u8.num_digits(), 1);
    assert_eq!(10_u8.num_digits(), 2);
    assert_eq!(100_u8.num_digits(), 3);

    assert_eq!(1_u16.num_digits(), 1);
    assert_eq!(10_u16.num_digits(), 2);
    assert_eq!(100_u16.num_digits(), 3);
    assert_eq!(1_000_u16.num_digits(), 4);
    assert_eq!(10_000_u16.num_digits(), 5);

    assert_eq!(1_u32.num_digits(), 1);
    assert_eq!(10_u32.num_digits(), 2);
    assert_eq!(100_u32.num_digits(), 3);
    assert_eq!(1_000_u32.num_digits(), 4);
    assert_eq!(10_000_u32.num_digits(), 5);
    assert_eq!(100_000_u32.num_digits(), 6);
    assert_eq!(1_000_000_u32.num_digits(), 7);
    assert_eq!(10_000_000_u32.num_digits(), 8);
    assert_eq!(100_000_000_u32.num_digits(), 9);
    assert_eq!(1_000_000_000_u32.num_digits(), 10);
}

#[test]
fn test_days_in_year_month() {
    // Common year
    assert_eq!(days_in_year_month(2019, 1), 31);
    assert_eq!(days_in_year_month(2019, 2), 28);
    assert_eq!(days_in_year_month(2019, 3), 31);
    assert_eq!(days_in_year_month(2019, 4), 30);
    assert_eq!(days_in_year_month(2019, 5), 31);
    assert_eq!(days_in_year_month(2019, 6), 30);
    assert_eq!(days_in_year_month(2019, 7), 31);
    assert_eq!(days_in_year_month(2019, 8), 31);
    assert_eq!(days_in_year_month(2019, 9), 30);
    assert_eq!(days_in_year_month(2019, 10), 31);
    assert_eq!(days_in_year_month(2019, 11), 30);
    assert_eq!(days_in_year_month(2019, 12), 31);

    // Leap year
    assert_eq!(days_in_year_month(2020, 1), 31);
    assert_eq!(days_in_year_month(2020, 2), 29);
    assert_eq!(days_in_year_month(2020, 3), 31);
    assert_eq!(days_in_year_month(2020, 4), 30);
    assert_eq!(days_in_year_month(2020, 5), 31);
    assert_eq!(days_in_year_month(2020, 6), 30);
    assert_eq!(days_in_year_month(2020, 7), 31);
    assert_eq!(days_in_year_month(2020, 8), 31);
    assert_eq!(days_in_year_month(2020, 9), 30);
    assert_eq!(days_in_year_month(2020, 10), 31);
    assert_eq!(days_in_year_month(2020, 11), 30);
    assert_eq!(days_in_year_month(2020, 12), 31);
}
