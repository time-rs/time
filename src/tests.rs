//! Tests for internal details.
//!
//! This module should only be used when it is not possible to test the implementation in a
//! reasonable manner externally.

use crate::formatting::DigitCount;
use crate::util::days_in_year_month;
use crate::Month;

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
    assert_eq!(days_in_year_month(2019, Month::January), 31);
    assert_eq!(days_in_year_month(2019, Month::February), 28);
    assert_eq!(days_in_year_month(2019, Month::March), 31);
    assert_eq!(days_in_year_month(2019, Month::April), 30);
    assert_eq!(days_in_year_month(2019, Month::May), 31);
    assert_eq!(days_in_year_month(2019, Month::June), 30);
    assert_eq!(days_in_year_month(2019, Month::July), 31);
    assert_eq!(days_in_year_month(2019, Month::August), 31);
    assert_eq!(days_in_year_month(2019, Month::September), 30);
    assert_eq!(days_in_year_month(2019, Month::October), 31);
    assert_eq!(days_in_year_month(2019, Month::November), 30);
    assert_eq!(days_in_year_month(2019, Month::December), 31);

    // Leap year
    assert_eq!(days_in_year_month(2020, Month::January), 31);
    assert_eq!(days_in_year_month(2020, Month::February), 29);
    assert_eq!(days_in_year_month(2020, Month::March), 31);
    assert_eq!(days_in_year_month(2020, Month::April), 30);
    assert_eq!(days_in_year_month(2020, Month::May), 31);
    assert_eq!(days_in_year_month(2020, Month::June), 30);
    assert_eq!(days_in_year_month(2020, Month::July), 31);
    assert_eq!(days_in_year_month(2020, Month::August), 31);
    assert_eq!(days_in_year_month(2020, Month::September), 30);
    assert_eq!(days_in_year_month(2020, Month::October), 31);
    assert_eq!(days_in_year_month(2020, Month::November), 30);
    assert_eq!(days_in_year_month(2020, Month::December), 31);
}
