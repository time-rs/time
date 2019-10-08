use crate::Weekday::{self, Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};
use crate::{DateTime, Duration, Time};
use core::cmp::{Ord, Ordering, PartialOrd};
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration as StdDuration;

// Some methods could be `const fn` due to the internal structure of `Date`, but
// are explicitly not (and have linting disabled) as it could lead to
// compatibility issues down the road if the internal structure is changed.

/// The number of days in a month in a non-leap year.
const DAYS_IN_MONTH: [u16; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// The number of days in a month in a leap year.
const DAYS_IN_MONTH_LEAP: [u16; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Get the number of days in the month of a given year.
fn days_in_year_month(year: i32, month: u8) -> u8 {
    if is_leap_year(year) {
        #[allow(clippy::cast_possible_truncation)]
        {
            DAYS_IN_MONTH_LEAP[(month - 1) as usize] as u8
        }
    } else {
        #[allow(clippy::cast_possible_truncation)]
        {
            DAYS_IN_MONTH[(month - 1) as usize] as u8
        }
    }
}

/// Returns if the provided year is a leap year in the proleptic Gregorian
/// calendar. Uses [astronomical year numbering](https://en.wikipedia.org/wiki/Astronomical_year_numbering).
///
/// ```rust
/// # use time::is_leap_year;
/// assert!(!is_leap_year(1900));
/// assert!(is_leap_year(2000));
/// assert!(is_leap_year(2004));
/// assert!(!is_leap_year(2005));
/// assert!(!is_leap_year(2100));
/// ```
pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) & ((year % 100 != 0) | (year % 400 == 0))
}

/// Get the number of calendar days in a given year, either 365 or 366.
///
/// ```rust
/// # use time::days_in_year;
/// assert_eq!(days_in_year(1900), 365);
/// assert_eq!(days_in_year(2000), 366);
/// assert_eq!(days_in_year(2004), 366);
/// assert_eq!(days_in_year(2005), 365);
/// assert_eq!(days_in_year(2100), 365);
/// ```
pub const fn days_in_year(year: i32) -> u16 {
    365 + is_leap_year(year) as u16
}

/// Get the number of weeks in the ISO year.
///
/// The returned value will always be either 52 or 53.
///
/// ```rust
/// # use time::weeks_in_year;
/// assert_eq!(weeks_in_year(2019), 52);
/// assert_eq!(weeks_in_year(2020), 53);
/// ```
pub fn weeks_in_year(year: i32) -> u8 {
    let weekday = Date::from_yo(year, 1).weekday();

    if (weekday == Thursday) || (weekday == Wednesday && is_leap_year(year)) {
        53
    } else {
        52
    }
}

/// Calendar date. All reasonable proleptic Gregorian dates are able to be
/// stored.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) year: i32,
    #[allow(clippy::missing_docs_in_private_items)]
    pub(crate) ordinal: u16,
}

impl Date {
    /// Create a `Date` from the year, month, and day.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1), Date::from_yo(2019, 1));
    /// assert_eq!(Date::from_ymd(2019, 12, 31), Date::from_yo(2019, 365));
    /// ```
    ///
    /// Panics if the date is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Date;
    /// Date::from_ymd(2019, 2, 29); // 2019 isn't a leap year.
    /// ```
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Self {
        assert_value_in_range!(month in 1 => 12);
        assert_value_in_range!(day in 1 => days_in_year_month(year, month), given year, month);

        let ordinal: u16 = if is_leap_year(year) {
            DAYS_IN_MONTH_LEAP[..month as usize - 1].iter().sum()
        } else {
            DAYS_IN_MONTH[..month as usize - 1].iter().sum()
        };

        Self {
            year,
            ordinal: ordinal + day as u16,
        }
    }

    /// Create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_yo(2019, 1), Date::from_ymd(2019, 1, 1));
    /// assert_eq!(Date::from_yo(2019, 365), Date::from_ymd(2019, 12, 31));
    /// ```
    ///
    /// Panics if the date is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::Date;
    /// Date::from_yo(2019, 366); // 2019 isn't a leap year.
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn from_yo(year: i32, ordinal: u16) -> Self {
        assert_value_in_range!(ordinal in 1 => days_in_year(year), given year);
        Self { year, ordinal }
    }

    /// Create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert_eq!(Date::from_iso_ywd(2019, 1, Monday), Date::from_ymd(2018, 12, 31));
    /// assert_eq!(Date::from_iso_ywd(2019, 1, Tuesday), Date::from_ymd(2019, 1, 1));
    /// assert_eq!(Date::from_iso_ywd(2020, 53, Friday), Date::from_ymd(2021, 1, 1));
    /// ```
    ///
    /// Panics if the week is not valid.
    ///
    /// ```rust,should_panic
    /// # use time::{Date, Weekday::*};
    /// Date::from_iso_ywd(2019, 53, Monday); // 2019 doesn't have 53 weeks.
    /// ```
    pub fn from_iso_ywd(year: i32, week: u8, weekday: Weekday) -> Self {
        assert_value_in_range!(week in 1 => weeks_in_year(year), given year);

        let ordinal = week as u16 * 7 + weekday.iso_weekday_number() as u16
            - (Self::from_yo(year, 4).weekday().iso_weekday_number() as u16 + 3);

        if ordinal < 1 {
            return Self::from_yo(year - 1, ordinal + days_in_year(year - 1));
        }

        let days_in_cur_year = days_in_year(year);
        if ordinal > days_in_cur_year {
            Self::from_yo(year + 1, ordinal - days_in_cur_year)
        } else {
            Self::from_yo(year, ordinal)
        }
    }

    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).year(), 2019);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).year(), 2019);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).year(), 2020);
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn year(self) -> i32 {
        self.year
    }

    /// Get the month of the date. If fetching both the month and day, use
    /// [`Date::date`](Date::date) instead.
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).month(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).month(), 12);
    /// ```
    pub fn month(self) -> u8 {
        self.month_day().0
    }

    /// Get the day of the date. If fetching both the month and day, use
    /// [`Date::date`](Date::date) instead.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).day(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).day(), 31);
    /// ```
    pub fn day(self) -> u8 {
        self.month_day().1
    }

    /// Get the month and day of the date.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).month_day(), (1, 1));
    /// assert_eq!(Date::from_ymd(2019, 12, 31).month_day(), (12, 31));
    /// ```
    pub fn month_day(self) -> (u8, u8) {
        let mut ordinal = self.ordinal;

        let days = if is_leap_year(self.year) {
            DAYS_IN_MONTH_LEAP
        } else {
            DAYS_IN_MONTH
        };

        let mut month = 0;
        let month = loop {
            if ordinal <= days[month] {
                break month;
            }
            ordinal -= days[month];
            month += 1;
        };

        #[allow(clippy::cast_possible_truncation)]
        (month as u8 + 1, ordinal as u8)
    }

    /// Get the day of the year of the date.
    ///
    /// The returned value will always be in the range `1..=366`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).ordinal(), 1);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).ordinal(), 365);
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn ordinal(self) -> u16 {
        self.ordinal
    }

    /// Get the ISO 8601 year and week number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).iso_year_week(), (2019, 1));
    /// assert_eq!(Date::from_ymd(2019, 10, 4).iso_year_week(), (2019, 40));
    /// assert_eq!(Date::from_ymd(2020, 1, 1).iso_year_week(), (2020, 1));
    /// assert_eq!(Date::from_ymd(2020, 12, 31).iso_year_week(), (2020, 53));
    /// assert_eq!(Date::from_ymd(2021, 1, 1).iso_year_week(), (2020, 53));
    /// ```
    pub fn iso_year_week(self) -> (i32, u8) {
        let weekday = self.weekday();
        #[allow(clippy::cast_possible_truncation)]
        let week = ((self.ordinal + 10 - weekday.iso_weekday_number() as u16) / 7) as u8;

        match week {
            0 => (self.year - 1, weeks_in_year(self.year - 1)),
            53 if weeks_in_year(self.year) == 52 => (self.year + 1, 1),
            _ => (self.year, week),
        }
    }

    /// Get the ISO week number of the date.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).week(), 1);
    /// assert_eq!(Date::from_ymd(2019, 10, 4).week(), 40);
    /// assert_eq!(Date::from_ymd(2020, 1, 1).week(), 1);
    /// assert_eq!(Date::from_ymd(2020, 12, 31).week(), 53);
    /// assert_eq!(Date::from_ymd(2021, 1, 1).week(), 53);
    /// ```
    pub fn week(self) -> u8 {
        self.iso_year_week().1
    }

    /// Get the year, month, and day of the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).as_ymd(), (2019, 1, 1));
    /// ```
    pub fn as_ymd(self) -> (i32, u8, u8) {
        let (month, day) = self.month_day();
        (self.year, month, day)
    }

    /// Get the year and ordinal day number of the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).as_yo(), (2019, 1));
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_yo(self) -> (i32, u16) {
        (self.year, self.ordinal)
    }

    /// Get the weekday of the date.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert_eq!(Date::from_ymd(2019, 1, 1).weekday(), Tuesday);
    /// assert_eq!(Date::from_ymd(2019, 2, 1).weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 3, 1).weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 4, 1).weekday(), Monday);
    /// assert_eq!(Date::from_ymd(2019, 5, 1).weekday(), Wednesday);
    /// assert_eq!(Date::from_ymd(2019, 6, 1).weekday(), Saturday);
    /// assert_eq!(Date::from_ymd(2019, 7, 1).weekday(), Monday);
    /// assert_eq!(Date::from_ymd(2019, 8, 1).weekday(), Thursday);
    /// assert_eq!(Date::from_ymd(2019, 9, 1).weekday(), Sunday);
    /// assert_eq!(Date::from_ymd(2019, 10, 1).weekday(), Tuesday);
    /// assert_eq!(Date::from_ymd(2019, 11, 1).weekday(), Friday);
    /// assert_eq!(Date::from_ymd(2019, 12, 1).weekday(), Sunday);
    /// ```
    pub fn weekday(self) -> Weekday {
        // Don't recalculate the value every time.
        let (mut month, day) = self.month_day();

        let adjusted_year = if month < 3 {
            month += 12;
            self.year - 1
        } else {
            self.year
        };

        match (day as i32 + (13 * (month as i32 + 1)) / 5 + adjusted_year + adjusted_year / 4
            - adjusted_year / 100
            + adjusted_year / 400)
            % 7
        {
            0 => Saturday,
            1 => Sunday,
            2 => Monday,
            3 => Tuesday,
            4 => Wednesday,
            5 => Thursday,
            6 => Friday,
            _ => unreachable!("A value mod 7 is always in the range 0..7"),
        }
    }

    /// Get the next calendar date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 1).next_day(), Date::from_ymd(2019, 1, 2));
    /// assert_eq!(Date::from_ymd(2019, 1, 31).next_day(), Date::from_ymd(2019, 2, 1));
    /// assert_eq!(Date::from_ymd(2019, 12, 31).next_day(), Date::from_ymd(2020, 1, 1));
    /// ```
    pub fn next_day(mut self) -> Self {
        self.ordinal += 1;

        if self.ordinal > days_in_year(self.year) {
            self.year += 1;
            self.ordinal = 1;
        }

        self
    }

    /// Get the previous calendar date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(2019, 1, 2).previous_day(), Date::from_ymd(2019, 1, 1));
    /// assert_eq!(Date::from_ymd(2019, 2, 1).previous_day(), Date::from_ymd(2019, 1, 31));
    /// assert_eq!(Date::from_ymd(2020, 1, 1).previous_day(), Date::from_ymd(2019, 12, 31));
    /// ```
    pub fn previous_day(mut self) -> Self {
        self.ordinal -= 1;

        if self.ordinal == 0 {
            self.year -= 1;
            self.ordinal = days_in_year(self.year);
        }

        self
    }

    /// Get the Julian day for the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_ymd(-4713, 11, 24).julian_day(), 0);
    /// assert_eq!(Date::from_ymd(2000, 1, 1).julian_day(), 2_451_545);
    /// assert_eq!(Date::from_ymd(2019, 1, 1).julian_day(), 2_458_485);
    /// assert_eq!(Date::from_ymd(2019, 12, 31).julian_day(), 2_458_849);
    /// ```
    pub fn julian_day(self) -> i64 {
        let year = self.year as i64;
        let (month, day) = self.month_day();
        let month = month as i64;
        let day = day as i64;
        (1_461 * (year + 4_800 + (month - 14) / 12)) / 4
            + (367 * (month - 2 - 12 * ((month - 14) / 12))) / 12
            - (3 * ((year + 4_900 + (month - 14) / 12) / 100)) / 4
            + day
            - 32_075
    }

    /// Create a `Date` from the Julian day.
    ///
    /// The algorithm to perform this conversion comes from E.G. Richards; it is
    /// available in [section 15.11.3](https://aa.usno.navy.mil/publications/docs/c15_usb_online.pdf),
    /// courtesy of the United States Naval Observatory.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert_eq!(Date::from_julian_day(0), Date::from_ymd(-4713, 11, 24));
    /// assert_eq!(Date::from_julian_day(2_451_545), Date::from_ymd(2000, 1, 1));
    /// assert_eq!(Date::from_julian_day(2_458_485), Date::from_ymd(2019, 1, 1));
    /// assert_eq!(Date::from_julian_day(2_458_849), Date::from_ymd(2019, 12, 31));
    /// ```
    pub fn from_julian_day(julian_day: i64) -> Self {
        #![allow(clippy::missing_docs_in_private_items)]
        const Y: i64 = 4_716;
        const J: i64 = 1_401;
        const M: i64 = 2;
        const N: i64 = 12;
        const R: i64 = 4;
        const P: i64 = 1_461;
        const V: i64 = 3;
        const U: i64 = 5;
        const S: i64 = 153;
        const W: i64 = 2;
        const B: i64 = 274_277;
        const C: i64 = -38;

        let f = julian_day + J + (((4 * julian_day + B) / 146_097) * 3) / 4 + C;
        let e = R * f + V;
        let g = e.rem_euclid(P) / R;
        let h = U * g + W;
        let day = h.rem_euclid(S) / U + 1;
        let month = (h / S + M).rem_euclid(N) + 1;
        let year = (e / P) - Y + (N + M - month) / N;

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Self::from_ymd(year as i32, month as u8, day as u8)
    }
}

/// Methods to add a `Time` component, resulting in a `DateTime`.
impl Date {
    /// Create a `DateTime` using the existing date. The `Time` component will
    /// be set to midnight.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(Date::from_ymd(1970, 1, 1).midnight(), DateTime::unix_epoch());
    /// ```
    pub const fn midnight(self) -> DateTime {
        DateTime::new(self, Time::midnight())
    }

    /// Create a `DateTime` using the existing date and the provided `Time`.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms(0, 0, 0)),
    ///     Date::from_ymd(1970, 1, 1).midnight(),
    /// );
    /// ```
    pub const fn with_time(self, time: Time) -> DateTime {
        DateTime::new(self, time)
    }

    /// Create a `DateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_hms(0, 0, 0),
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms(0, 0, 0)),
    /// );
    /// ```
    pub fn with_hms(self, hour: u8, minute: u8, second: u8) -> DateTime {
        DateTime::new(self, Time::from_hms(hour, minute, second))
    }

    /// Create a `DateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_hms_milli(0, 0, 0, 0),
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms_milli(0, 0, 0, 0)),
    /// );
    /// ```
    pub fn with_hms_milli(self, hour: u8, minute: u8, second: u8, millisecond: u16) -> DateTime {
        DateTime::new(
            self,
            Time::from_hms_milli(hour, minute, second, millisecond),
        )
    }

    /// Create a `DateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_hms_micro(0, 0, 0, 0),
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms_micro(0, 0, 0, 0)),
    /// );
    /// ```
    pub fn with_hms_micro(self, hour: u8, minute: u8, second: u8, microsecond: u32) -> DateTime {
        DateTime::new(
            self,
            Time::from_hms_micro(hour, minute, second, microsecond),
        )
    }

    /// Create a `DateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::{Date, DateTime, Time};
    /// assert_eq!(
    ///     Date::from_ymd(1970, 1, 1).with_hms_nano(0, 0, 0, 0),
    ///     Date::from_ymd(1970, 1, 1).with_time(Time::from_hms_nano(0, 0, 0, 0)),
    /// );
    /// ```
    pub fn with_hms_nano(self, hour: u8, minute: u8, second: u8, nanosecond: u32) -> DateTime {
        DateTime::new(self, Time::from_hms_nano(hour, minute, second, nanosecond))
    }
}

impl Add<Duration> for Date {
    type Output = Self;

    /// Add the whole number of days of the `Duration` to the date.
    ///
    /// ```rust
    /// # use time::{Date, Duration};
    /// assert_eq!(Date::from_ymd(2019, 1, 1) + Duration::days(5), Date::from_ymd(2019, 1, 6));
    /// assert_eq!(Date::from_ymd(2019, 12, 31) + Duration::day(), Date::from_ymd(2020, 1, 1));
    /// ```
    fn add(self, duration: Duration) -> Self::Output {
        Self::from_julian_day(self.julian_day() + duration.whole_days())
    }
}

impl Add<StdDuration> for Date {
    type Output = Self;

    /// Add the whole number of days of the `std::time::Duration` to the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// # use core::time::Duration;
    /// assert_eq!(Date::from_ymd(2019, 1, 1) + Duration::from_secs(5 * 86_400), Date::from_ymd(2019, 1, 6));
    /// assert_eq!(Date::from_ymd(2019, 12, 31) + Duration::from_secs(86_400), Date::from_ymd(2020, 1, 1));
    /// ```
    fn add(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.julian_day() + Duration::from(duration).whole_days())
    }
}

impl AddAssign<Duration> for Date {
    /// Add the whole number of days of the `Duration` to the date.
    ///
    /// ```rust
    /// # use time::{Date, Duration};
    /// let mut date = Date::from_ymd(2019, 12, 31);
    /// date += Duration::day();
    /// assert_eq!(date, Date::from_ymd(2020, 1, 1));
    /// ```
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for Date {
    /// Add the whole number of days of the `std::time::Duration` to the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// # use core::time::Duration;
    /// let mut date = Date::from_ymd(2019, 12, 31);
    /// date += Duration::from_secs(86_400);
    /// assert_eq!(date, Date::from_ymd(2020, 1, 1));
    /// ```
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Date {
    type Output = Self;

    /// Subtract the whole number of days of the `Duration` from the date.
    ///
    /// ```rust
    /// # use time::{Date, Duration};
    /// assert_eq!(Date::from_ymd(2019, 1, 6) - Duration::days(5), Date::from_ymd(2019, 1, 1));
    /// assert_eq!(Date::from_ymd(2020, 1, 1) - Duration::day(), Date::from_ymd(2019, 12, 31));
    /// ```
    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl Sub<StdDuration> for Date {
    type Output = Self;

    /// Subtract the whole number of days of the `std::time::Duration` from the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// # use core::time::Duration;
    /// assert_eq!(Date::from_ymd(2019, 1, 6) - Duration::from_secs(5 * 86_400), Date::from_ymd(2019, 1, 1));
    /// assert_eq!(Date::from_ymd(2020, 1, 1) - Duration::from_secs(86_400), Date::from_ymd(2019, 12, 31));
    /// ```
    fn sub(self, duration: StdDuration) -> Self::Output {
        self + -Duration::from(duration)
    }
}

impl SubAssign<Duration> for Date {
    /// Subtract the whole number of days of the `Duration` from the date.
    ///
    /// ```rust
    /// # use time::{Date, Duration};
    /// let mut date = Date::from_ymd(2020, 1, 1);
    /// date -= Duration::day();
    /// assert_eq!(date, Date::from_ymd(2019, 12, 31));
    /// ```
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for Date {
    /// Subtract the whole number of days of the `std::time::Duration` from the date.
    ///
    /// ```rust
    /// # use time::Date;
    /// # use core::time::Duration;
    /// let mut date = Date::from_ymd(2020, 1, 1);
    /// date -= Duration::from_secs(86_400);
    /// assert_eq!(date, Date::from_ymd(2019, 12, 31));
    /// ```
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<Date> for Date {
    type Output = Duration;

    /// Subtrace two `Date`s, returning the number of days between.
    ///
    /// ```rust
    /// # use time::{Date, Duration};
    /// assert_eq!(Date::from_ymd(2019, 1, 6) - Date::from_ymd(2019, 1, 1), Duration::days(5));
    /// assert_eq!(Date::from_ymd(2020, 1, 1) - Date::from_ymd(2019, 12, 31), Duration::day());
    /// ```
    fn sub(self, other: Self) -> Self::Output {
        Duration::days(self.julian_day() - other.julian_day())
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.ordinal.cmp(&other.ordinal),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn weeks_in_year_exhaustive() {
        extern crate alloc;
        let mut years_with_53 = alloc::collections::btree_set::BTreeSet::new();
        for year in [
            4, 9, 15, 20, 26, 32, 37, 43, 48, 54, 60, 65, 71, 76, 82, 88, 93, 99, 105, 111, 116,
            122, 128, 133, 139, 144, 150, 156, 161, 167, 172, 178, 184, 189, 195, 201, 207, 212,
            218, 224, 229, 235, 240, 246, 252, 257, 263, 268, 274, 280, 285, 291, 296, 303, 308,
            314, 320, 325, 331, 336, 342, 348, 353, 359, 364, 370, 376, 381, 387, 392, 398,
        ]
        .iter()
        {
            years_with_53.insert(year);
        }

        for year in 0..400 {
            assert_eq!(
                weeks_in_year(year),
                if years_with_53.contains(&year) {
                    53
                } else {
                    52
                }
            );
        }
    }
}
