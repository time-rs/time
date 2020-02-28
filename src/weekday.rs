use crate::internal_prelude::*;
use core::fmt::{self, Display};

/// Days of the week.
///
/// As order is dependent on context (Sunday could be either
/// two days after or five days before Friday), this type does not implement
/// `PartialOrd` or `Ord`.
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    serde,
    serde(try_from = "crate::serde::Weekday", into = "crate::serde::Weekday")
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    #[allow(clippy::missing_docs_in_private_items)]
    Monday,
    #[allow(clippy::missing_docs_in_private_items)]
    Tuesday,
    #[allow(clippy::missing_docs_in_private_items)]
    Wednesday,
    #[allow(clippy::missing_docs_in_private_items)]
    Thursday,
    #[allow(clippy::missing_docs_in_private_items)]
    Friday,
    #[allow(clippy::missing_docs_in_private_items)]
    Saturday,
    #[allow(clippy::missing_docs_in_private_items)]
    Sunday,
}

impl Weekday {
    /// Get the previous weekday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Tuesday.previous(), Weekday::Monday);
    /// ```
    #[inline(always)]
    pub fn previous(self) -> Self {
        match self {
            Monday => Sunday,
            Tuesday => Monday,
            Wednesday => Tuesday,
            Thursday => Wednesday,
            Friday => Thursday,
            Saturday => Friday,
            Sunday => Saturday,
        }
    }

    /// Get the next weekday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.next(), Weekday::Tuesday);
    /// ```
    #[inline(always)]
    pub fn next(self) -> Self {
        match self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }

    /// Get the ISO 8601 weekday number. Equivalent to
    /// [`Weekday::number_from_monday`].
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.iso_weekday_number(), 1);
    /// ```
    #[inline(always)]
    pub const fn iso_weekday_number(self) -> u8 {
        self.number_from_monday()
    }

    /// Get the one-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_from_monday(), 1);
    /// ```
    #[inline(always)]
    pub const fn number_from_monday(self) -> u8 {
        self.number_days_from_monday() + 1
    }

    /// Get the one-indexed number of days from Sunday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_from_sunday(), 2);
    /// ```
    #[inline(always)]
    pub const fn number_from_sunday(self) -> u8 {
        self.number_days_from_sunday() + 1
    }

    /// Get the zero-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_days_from_monday(), 0);
    /// ```
    #[inline(always)]
    pub const fn number_days_from_monday(self) -> u8 {
        self as u8
    }

    /// Get the zero-indexed number of days from Sunday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_days_from_sunday(), 1);
    /// ```
    #[inline(always)]
    pub const fn number_days_from_sunday(self) -> u8 {
        (self as u8 + 1) % 7
    }
}

impl Display for Weekday {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday",
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn iso_weekday_number() {
        assert_eq!(Monday.iso_weekday_number(), 1);
        assert_eq!(Tuesday.iso_weekday_number(), 2);
        assert_eq!(Wednesday.iso_weekday_number(), 3);
        assert_eq!(Thursday.iso_weekday_number(), 4);
        assert_eq!(Friday.iso_weekday_number(), 5);
        assert_eq!(Saturday.iso_weekday_number(), 6);
        assert_eq!(Sunday.iso_weekday_number(), 7);
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
}
