use time::Weekday::{self, *};

#[test]
fn previous() {
    assert_eq!(Sunday.previous(), Saturday);
    assert_eq!(Monday.previous(), Sunday);
    assert_eq!(Tuesday.previous(), Monday);
    assert_eq!(Wednesday.previous(), Tuesday);
    assert_eq!(Thursday.previous(), Wednesday);
    assert_eq!(Friday.previous(), Thursday);
    assert_eq!(Saturday.previous(), Friday);
}

#[test]
fn next() {
    assert_eq!(Sunday.next(), Monday);
    assert_eq!(Monday.next(), Tuesday);
    assert_eq!(Tuesday.next(), Wednesday);
    assert_eq!(Wednesday.next(), Thursday);
    assert_eq!(Thursday.next(), Friday);
    assert_eq!(Friday.next(), Saturday);
    assert_eq!(Saturday.next(), Sunday);
}

#[test]
fn nth_next() {
    assert_eq!(Sunday.nth_next(0), Sunday);
    assert_eq!(Sunday.nth_next(1), Monday);
    assert_eq!(Sunday.nth_next(2), Tuesday);
    assert_eq!(Sunday.nth_next(3), Wednesday);
    assert_eq!(Sunday.nth_next(4), Thursday);
    assert_eq!(Sunday.nth_next(5), Friday);
    assert_eq!(Sunday.nth_next(6), Saturday);

    assert_eq!(Monday.nth_next(0), Monday);
    assert_eq!(Monday.nth_next(1), Tuesday);
    assert_eq!(Monday.nth_next(2), Wednesday);
    assert_eq!(Monday.nth_next(3), Thursday);
    assert_eq!(Monday.nth_next(4), Friday);
    assert_eq!(Monday.nth_next(5), Saturday);
    assert_eq!(Monday.nth_next(6), Sunday);

    assert_eq!(Sunday.nth_next(7), Sunday);
    assert_eq!(Sunday.nth_next(u8::MAX), Wednesday);
    assert_eq!(Monday.nth_next(7), Monday);
    assert_eq!(Monday.nth_next(u8::MAX), Thursday);
}

#[test]
fn number_from_monday() {
    assert_eq!(Monday.number_from_monday(), 1);
    assert_eq!(Tuesday.number_from_monday(), 2);
    assert_eq!(Wednesday.number_from_monday(), 3);
    assert_eq!(Thursday.number_from_monday(), 4);
    assert_eq!(Friday.number_from_monday(), 5);
    assert_eq!(Saturday.number_from_monday(), 6);
    assert_eq!(Sunday.number_from_monday(), 7);
}

#[test]
fn number_from_sunday() {
    assert_eq!(Sunday.number_from_sunday(), 1);
    assert_eq!(Monday.number_from_sunday(), 2);
    assert_eq!(Tuesday.number_from_sunday(), 3);
    assert_eq!(Wednesday.number_from_sunday(), 4);
    assert_eq!(Thursday.number_from_sunday(), 5);
    assert_eq!(Friday.number_from_sunday(), 6);
    assert_eq!(Saturday.number_from_sunday(), 7);
}

#[test]
fn number_days_from_monday() {
    assert_eq!(Monday.number_days_from_monday(), 0);
    assert_eq!(Tuesday.number_days_from_monday(), 1);
    assert_eq!(Wednesday.number_days_from_monday(), 2);
    assert_eq!(Thursday.number_days_from_monday(), 3);
    assert_eq!(Friday.number_days_from_monday(), 4);
    assert_eq!(Saturday.number_days_from_monday(), 5);
    assert_eq!(Sunday.number_days_from_monday(), 6);
}

#[test]
fn number_days_from_sunday() {
    assert_eq!(Sunday.number_days_from_sunday(), 0);
    assert_eq!(Monday.number_days_from_sunday(), 1);
    assert_eq!(Tuesday.number_days_from_sunday(), 2);
    assert_eq!(Wednesday.number_days_from_sunday(), 3);
    assert_eq!(Thursday.number_days_from_sunday(), 4);
    assert_eq!(Friday.number_days_from_sunday(), 5);
    assert_eq!(Saturday.number_days_from_sunday(), 6);
}

#[test]
fn display() {
    assert_eq!(Monday.to_string(), "Monday");
    assert_eq!(Tuesday.to_string(), "Tuesday");
    assert_eq!(Wednesday.to_string(), "Wednesday");
    assert_eq!(Thursday.to_string(), "Thursday");
    assert_eq!(Friday.to_string(), "Friday");
    assert_eq!(Saturday.to_string(), "Saturday");
    assert_eq!(Sunday.to_string(), "Sunday");
}

#[test]
fn from_str() {
    assert_eq!("Monday".parse(), Ok(Monday));
    assert_eq!("Tuesday".parse(), Ok(Tuesday));
    assert_eq!("Wednesday".parse(), Ok(Wednesday));
    assert_eq!("Thursday".parse(), Ok(Thursday));
    assert_eq!("Friday".parse(), Ok(Friday));
    assert_eq!("Saturday".parse(), Ok(Saturday));
    assert_eq!("Sunday".parse(), Ok(Sunday));
    assert_eq!("foo".parse::<Weekday>(), Err(time::error::InvalidVariant));
}
