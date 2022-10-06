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
