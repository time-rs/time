use time::Month::{self, *};

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

#[allow(clippy::cognitive_complexity)] // all test the same thing
#[test]
fn nth_next() {
    assert_eq!(January.nth_next(0), January);
    assert_eq!(January.nth_next(1), February);
    assert_eq!(January.nth_next(2), March);
    assert_eq!(January.nth_next(3), April);
    assert_eq!(January.nth_next(4), May);
    assert_eq!(January.nth_next(5), June);
    assert_eq!(January.nth_next(6), July);
    assert_eq!(January.nth_next(7), August);
    assert_eq!(January.nth_next(8), September);
    assert_eq!(January.nth_next(9), October);
    assert_eq!(January.nth_next(10), November);
    assert_eq!(January.nth_next(11), December);

    assert_eq!(December.nth_next(0), December);
    assert_eq!(December.nth_next(1), January);
    assert_eq!(December.nth_next(2), February);
    assert_eq!(December.nth_next(3), March);
    assert_eq!(December.nth_next(4), April);
    assert_eq!(December.nth_next(5), May);
    assert_eq!(December.nth_next(6), June);
    assert_eq!(December.nth_next(7), July);
    assert_eq!(December.nth_next(8), August);
    assert_eq!(December.nth_next(9), September);
    assert_eq!(December.nth_next(10), October);
    assert_eq!(December.nth_next(11), November);

    assert_eq!(January.nth_next(12), January);
    assert_eq!(January.nth_next(u8::MAX), April);
    assert_eq!(December.nth_next(12), December);
    assert_eq!(December.nth_next(u8::MAX), March);
}

#[allow(clippy::cognitive_complexity)] // all test the same thing
#[test]
fn nth_prev() {
    assert_eq!(January.nth_prev(0), January);
    assert_eq!(January.nth_prev(1), December);
    assert_eq!(January.nth_prev(2), November);
    assert_eq!(January.nth_prev(3), October);
    assert_eq!(January.nth_prev(4), September);
    assert_eq!(January.nth_prev(5), August);
    assert_eq!(January.nth_prev(6), July);
    assert_eq!(January.nth_prev(7), June);
    assert_eq!(January.nth_prev(8), May);
    assert_eq!(January.nth_prev(9), April);
    assert_eq!(January.nth_prev(10), March);
    assert_eq!(January.nth_prev(11), February);

    assert_eq!(December.nth_prev(0), December);
    assert_eq!(December.nth_prev(1), November);
    assert_eq!(December.nth_prev(2), October);
    assert_eq!(December.nth_prev(3), September);
    assert_eq!(December.nth_prev(4), August);
    assert_eq!(December.nth_prev(5), July);
    assert_eq!(December.nth_prev(6), June);
    assert_eq!(December.nth_prev(7), May);
    assert_eq!(December.nth_prev(8), April);
    assert_eq!(December.nth_prev(9), March);
    assert_eq!(December.nth_prev(10), February);
    assert_eq!(December.nth_prev(11), January);

    assert_eq!(January.nth_prev(12), January);
    assert_eq!(January.nth_prev(u8::MAX), October);
    assert_eq!(December.nth_prev(12), December);
    assert_eq!(December.nth_prev(u8::MAX), September);
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
fn from_str() {
    assert_eq!("January".parse(), Ok(January));
    assert_eq!("February".parse(), Ok(February));
    assert_eq!("March".parse(), Ok(March));
    assert_eq!("April".parse(), Ok(April));
    assert_eq!("May".parse(), Ok(May));
    assert_eq!("June".parse(), Ok(June));
    assert_eq!("July".parse(), Ok(July));
    assert_eq!("August".parse(), Ok(August));
    assert_eq!("September".parse(), Ok(September));
    assert_eq!("October".parse(), Ok(October));
    assert_eq!("November".parse(), Ok(November));
    assert_eq!("December".parse(), Ok(December));
    assert_eq!("foo".parse::<Month>(), Err(time::error::InvalidVariant));
}

#[test]
fn to_u8() {
    assert_eq!(u8::from(January), 1);
    assert_eq!(u8::from(February), 2);
    assert_eq!(u8::from(March), 3);
    assert_eq!(u8::from(April), 4);
    assert_eq!(u8::from(May), 5);
    assert_eq!(u8::from(June), 6);
    assert_eq!(u8::from(July), 7);
    assert_eq!(u8::from(August), 8);
    assert_eq!(u8::from(September), 9);
    assert_eq!(u8::from(October), 10);
    assert_eq!(u8::from(November), 11);
    assert_eq!(u8::from(December), 12);
}

#[test]
fn try_from_u8() {
    assert!(matches!(Month::try_from(0), Err(err) if err.name() == "month"));
    assert_eq!(Month::try_from(1), Ok(January));
    assert_eq!(Month::try_from(2), Ok(February));
    assert_eq!(Month::try_from(3), Ok(March));
    assert_eq!(Month::try_from(4), Ok(April));
    assert_eq!(Month::try_from(5), Ok(May));
    assert_eq!(Month::try_from(6), Ok(June));
    assert_eq!(Month::try_from(7), Ok(July));
    assert_eq!(Month::try_from(8), Ok(August));
    assert_eq!(Month::try_from(9), Ok(September));
    assert_eq!(Month::try_from(10), Ok(October));
    assert_eq!(Month::try_from(11), Ok(November));
    assert_eq!(Month::try_from(12), Ok(December));
    assert!(matches!(Month::try_from(13), Err(err) if err.name() == "month"));
}
