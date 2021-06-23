#![allow(clippy::let_underscore_drop, clippy::clone_on_copy)]

//! Tests for internal details.
//!
//! This module should only be used when it is not possible to test the implementation in a
//! reasonable manner externally.

use std::num::NonZeroU8;

use crate::format_description::modifier::Modifiers;
use crate::formatting::DigitCount;
use crate::{duration, Month};

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
fn month_from_number() {
    macro_rules! nz {
        ($x:literal) => {
            NonZeroU8::new($x).unwrap()
        };
    }

    assert_eq!(Month::from_number(nz!(1)), Ok(Month::January));
    assert_eq!(Month::from_number(nz!(2)), Ok(Month::February));
    assert_eq!(Month::from_number(nz!(3)), Ok(Month::March));
    assert_eq!(Month::from_number(nz!(4)), Ok(Month::April));
    assert_eq!(Month::from_number(nz!(5)), Ok(Month::May));
    assert_eq!(Month::from_number(nz!(6)), Ok(Month::June));
    assert_eq!(Month::from_number(nz!(7)), Ok(Month::July));
    assert_eq!(Month::from_number(nz!(8)), Ok(Month::August));
    assert_eq!(Month::from_number(nz!(9)), Ok(Month::September));
    assert_eq!(Month::from_number(nz!(10)), Ok(Month::October));
    assert_eq!(Month::from_number(nz!(11)), Ok(Month::November));
    assert_eq!(Month::from_number(nz!(12)), Ok(Month::December));
    assert!(Month::from_number(nz!(13)).is_err());
}

#[test]
fn default() {
    assert_eq!(
        duration::Padding::Optimize.clone(),
        duration::Padding::default()
    );
}

#[test]
fn debug() {
    let _ = format!("{:?}", duration::Padding::Optimize);
    let _ = format!("{:?}", Modifiers::default());
}
