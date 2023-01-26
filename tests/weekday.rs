use quickcheck_macros::quickcheck;
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
fn nth() {
    assert_eq!(Sunday.nth(0), Sunday);
    assert_eq!(Sunday.nth(1), Monday);
    assert_eq!(Sunday.nth(2), Tuesday);
    assert_eq!(Sunday.nth(3), Wednesday);
    assert_eq!(Sunday.nth(4), Thursday);
    assert_eq!(Sunday.nth(5), Friday);
    assert_eq!(Sunday.nth(6), Saturday);

    assert_eq!(Monday.nth(0), Monday);
    assert_eq!(Monday.nth(1), Tuesday);
    assert_eq!(Monday.nth(2), Wednesday);
    assert_eq!(Monday.nth(3), Thursday);
    assert_eq!(Monday.nth(4), Friday);
    assert_eq!(Monday.nth(5), Saturday);
    assert_eq!(Monday.nth(6), Sunday);

    assert_eq!(Sunday.nth(7), Sunday);
    assert_eq!(Sunday.nth(u8::MAX), Wednesday);
    assert_eq!(Monday.nth(7), Monday);
    assert_eq!(Monday.nth(u8::MAX), Thursday);
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

// Test operations that are expected to reverse each other.
mod round_trip {
    use super::*;

    #[quickcheck]
    fn number_from_monday(w: Weekday) -> bool {
        Monday.nth(w.number_from_monday() + 7 - 1) == w
    }

    #[quickcheck]
    fn number_from_sunday(w: Weekday) -> bool {
        Sunday.nth(w.number_from_sunday() + 7 - 1) == w
    }

    #[quickcheck]
    fn number_days_from_monday(w: Weekday) -> bool {
        Monday.nth(w.number_days_from_monday()) == w
    }

    #[quickcheck]
    fn number_days_from_sunday(w: Weekday) -> bool {
        Sunday.nth(w.number_days_from_sunday()) == w
    }
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
