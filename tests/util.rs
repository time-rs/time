use time::util;

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
    assert_eq!(util::weeks_in_year(2019), 52);
    assert_eq!(util::weeks_in_year(2020), 53);
}

#[test]
#[cfg(feature = "alloc")]
fn validate_format_string() {
    assert!(util::validate_format_string("%H foo").is_ok());
    assert!(util::validate_format_string("%H%%").is_ok());
    assert!(util::validate_format_string("%").is_err());
}
