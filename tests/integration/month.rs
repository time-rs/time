use std::convert::TryFrom;

use time::{Month, Month::*};

#[test]
fn previous() {
    assert_eq!(January.previous(), December);
    assert_eq!(February.previous(), January);
    assert_eq!(March.previous(), February);
    assert_eq!(April.previous(), March);
    assert_eq!(May.previous(), April);
    assert_eq!(June.previous(), May);
    assert_eq!(July.previous(), June);
    assert_eq!(August.previous(), July);
    assert_eq!(September.previous(), August);
    assert_eq!(October.previous(), September);
    assert_eq!(November.previous(), October);
    assert_eq!(December.previous(), November);
}

#[test]
fn next() {
    assert_eq!(January.next(), February);
    assert_eq!(February.next(), March);
    assert_eq!(March.next(), April);
    assert_eq!(April.next(), May);
    assert_eq!(May.next(), June);
    assert_eq!(June.next(), July);
    assert_eq!(July.next(), August);
    assert_eq!(August.next(), September);
    assert_eq!(September.next(), October);
    assert_eq!(October.next(), November);
    assert_eq!(November.next(), December);
    assert_eq!(December.next(), January);
}

#[test]
fn display() {
    assert_eq!(January.to_string(), "January");
    assert_eq!(February.to_string(), "February");
    assert_eq!(March.to_string(), "March");
    assert_eq!(April.to_string(), "April");
    assert_eq!(May.to_string(), "May");
    assert_eq!(June.to_string(), "June");
    assert_eq!(July.to_string(), "July");
    assert_eq!(August.to_string(), "August");
    assert_eq!(September.to_string(), "September");
    assert_eq!(October.to_string(), "October");
    assert_eq!(November.to_string(), "November");
    assert_eq!(December.to_string(), "December");
}

#[test]
fn to_u8() {
    assert_eq!(u8::from(January), 1u8);
    assert_eq!(u8::from(February), 2u8);
    assert_eq!(u8::from(March), 3u8);
    assert_eq!(u8::from(April), 4u8);
    assert_eq!(u8::from(May), 5u8);
    assert_eq!(u8::from(June), 6u8);
    assert_eq!(u8::from(July), 7u8);
    assert_eq!(u8::from(August), 8u8);
    assert_eq!(u8::from(September), 9u8);
    assert_eq!(u8::from(October), 10u8);
    assert_eq!(u8::from(November), 11u8);
    assert_eq!(u8::from(December), 12u8);
}

#[test]
fn try_from_u8() {
    assert!(Month::try_from(0u8).is_err());
    assert_eq!(Month::try_from(1u8), Ok(January));
    assert_eq!(Month::try_from(2u8), Ok(February));
    assert_eq!(Month::try_from(3u8), Ok(March));
    assert_eq!(Month::try_from(4u8), Ok(April));
    assert_eq!(Month::try_from(5u8), Ok(May));
    assert_eq!(Month::try_from(6u8), Ok(June));
    assert_eq!(Month::try_from(7u8), Ok(July));
    assert_eq!(Month::try_from(8u8), Ok(August));
    assert_eq!(Month::try_from(9u8), Ok(September));
    assert_eq!(Month::try_from(10u8), Ok(October));
    assert_eq!(Month::try_from(11u8), Ok(November));
    assert_eq!(Month::try_from(12u8), Ok(December));
    assert!(Month::try_from(13u8).is_err());
}
