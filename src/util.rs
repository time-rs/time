//! Utility functions.

use const_fn::const_fn;

/// The number of days in a month in both common and leap years.
const DAYS_IN_MONTH_COMMON_LEAP: [[u8; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

/// Get the number of days in the month of a given year.
#[const_fn("1.46")]
pub(crate) const fn days_in_year_month(year: i32, month: u8) -> u8 {
    DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1]
}

/// Returns if the provided year is a leap year in the proleptic Gregorian
/// calendar. Uses [astronomical year numbering](https://en.wikipedia.org/wiki/Astronomical_year_numbering).
///
/// ```rust
/// # use time::util::is_leap_year;
/// assert!(!is_leap_year(1900));
/// assert!(is_leap_year(2000));
/// assert!(is_leap_year(2004));
/// assert!(!is_leap_year(2005));
/// assert!(!is_leap_year(2100));
/// ```
///
/// This function is `const fn` when using rustc >= 1.46.
#[const_fn("1.46")]
pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Get the number of calendar days in a given year.
///
/// The returned value will always be either 365 or 366.
///
/// ```rust
/// # use time::util::days_in_year;
/// assert_eq!(days_in_year(1900), 365);
/// assert_eq!(days_in_year(2000), 366);
/// assert_eq!(days_in_year(2004), 366);
/// assert_eq!(days_in_year(2005), 365);
/// assert_eq!(days_in_year(2100), 365);
/// ```
///
/// This function is `const fn` when using rustc >= 1.46.
#[const_fn("1.46")]
pub const fn days_in_year(year: i32) -> u16 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

/// Get the number of weeks in the ISO year.
///
/// The returned value will always be either 52 or 53.
///
/// ```rust
/// # use time::util::weeks_in_year;
/// assert_eq!(weeks_in_year(2019), 52);
/// assert_eq!(weeks_in_year(2020), 53);
/// ```
///
/// This function is `const fn` when using rustc >= 1.46.
#[const_fn("1.46")]
pub const fn weeks_in_year(year: i32) -> u8 {
    let mut within_period = year % 400;
    if within_period < 0 {
        within_period += 400;
    }

    match within_period {
        4 | 9 | 15 | 20 | 26 | 32 | 37 | 43 | 48 | 54 | 60 | 65 | 71 | 76 | 82 | 88 | 93 | 99
        | 105 | 111 | 116 | 122 | 128 | 133 | 139 | 144 | 150 | 156 | 161 | 167 | 172 | 178
        | 184 | 189 | 195 | 201 | 207 | 212 | 218 | 224 | 229 | 235 | 240 | 246 | 252 | 257
        | 263 | 268 | 274 | 280 | 285 | 291 | 296 | 303 | 308 | 314 | 320 | 325 | 331 | 336
        | 342 | 348 | 353 | 359 | 364 | 370 | 376 | 381 | 387 | 392 | 398 => 53,
        _ => 52,
    }
}
