use time::{util, Month};

#[test]
fn days_in_year_month() {
    // Common year
    assert_eq!(util::days_in_year_month(2019, Month::January), 31);
    assert_eq!(util::days_in_year_month(2019, Month::February), 28);
    assert_eq!(util::days_in_year_month(2019, Month::March), 31);
    assert_eq!(util::days_in_year_month(2019, Month::April), 30);
    assert_eq!(util::days_in_year_month(2019, Month::May), 31);
    assert_eq!(util::days_in_year_month(2019, Month::June), 30);
    assert_eq!(util::days_in_year_month(2019, Month::July), 31);
    assert_eq!(util::days_in_year_month(2019, Month::August), 31);
    assert_eq!(util::days_in_year_month(2019, Month::September), 30);
    assert_eq!(util::days_in_year_month(2019, Month::October), 31);
    assert_eq!(util::days_in_year_month(2019, Month::November), 30);
    assert_eq!(util::days_in_year_month(2019, Month::December), 31);

    // Leap year
    assert_eq!(util::days_in_year_month(2020, Month::January), 31);
    assert_eq!(util::days_in_year_month(2020, Month::February), 29);
    assert_eq!(util::days_in_year_month(2020, Month::March), 31);
    assert_eq!(util::days_in_year_month(2020, Month::April), 30);
    assert_eq!(util::days_in_year_month(2020, Month::May), 31);
    assert_eq!(util::days_in_year_month(2020, Month::June), 30);
    assert_eq!(util::days_in_year_month(2020, Month::July), 31);
    assert_eq!(util::days_in_year_month(2020, Month::August), 31);
    assert_eq!(util::days_in_year_month(2020, Month::September), 30);
    assert_eq!(util::days_in_year_month(2020, Month::October), 31);
    assert_eq!(util::days_in_year_month(2020, Month::November), 30);
    assert_eq!(util::days_in_year_month(2020, Month::December), 31);
}

#[test]
fn is_leap_year() {
    assert!(!util::is_leap_year(1900));
    assert!(util::is_leap_year(2000));
    assert!(util::is_leap_year(2004));
    assert!(!util::is_leap_year(2005));
    assert!(!util::is_leap_year(2100));
}

#[test]
fn days_in_year() {
    assert_eq!(util::days_in_year(1900), 365);
    assert_eq!(util::days_in_year(2000), 366);
    assert_eq!(util::days_in_year(2004), 366);
    assert_eq!(util::days_in_year(2005), 365);
    assert_eq!(util::days_in_year(2100), 365);
}

#[test]
fn weeks_in_year() {
    let num_weeks_for_years = vec![
        52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52,
        52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52,
        52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52,
        52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52,
        53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52,
        53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52,
        53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53,
        52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53,
        52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52,
        52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52,
        52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52,
        52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52,
        52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 53, 52, 52, 52, 52, 52, 53, 52,
    ];

    for (year, &num_weeks) in (0..400).zip(&num_weeks_for_years) {
        assert_eq!(util::weeks_in_year(year), num_weeks);
    }
}
