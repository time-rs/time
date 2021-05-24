use time::Month::*;

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
