//! Formatting helpers for a `Date`.

#![allow(non_snake_case)]

use crate::format::{Language, Padding};
use crate::Date;
use core::fmt::{self, Formatter};

/// Short day of the week
pub(crate) fn fmt_a(f: &mut Formatter<'_>, date: Date, language: Language) -> fmt::Result {
    f.write_str(language.short_week_days()[date.weekday().number_days_from_monday() as usize])
}

/// Day of the week
pub(crate) fn fmt_A(f: &mut Formatter<'_>, date: Date, language: Language) -> fmt::Result {
    f.write_str(language.week_days()[date.weekday().number_days_from_monday() as usize])
}

/// Short month name
///
/// References on localization
/// - [Yale](https://web.library.yale.edu/cataloging/months)
/// - [Princeton](https://library.princeton.edu/departments/tsd/katmandu/reference/months.html)
pub(crate) fn fmt_b(f: &mut Formatter<'_>, date: Date, language: Language) -> fmt::Result {
    f.write_str(language.short_month_names()[date.month() as usize - 1])
}

/// Month name
pub(crate) fn fmt_B(f: &mut Formatter<'_>, date: Date, language: Language) -> fmt::Result {
    f.write_str(language.month_names()[date.month() as usize - 1])
}

/// Year divided by 100 and truncated to integer (`00`-`99`)
pub(crate) fn fmt_C(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, date.year() / 100)
}

/// Day of the month, zero-padded (`01`-`31`)
pub(crate) fn fmt_d(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, date.day())
}

/// Day of the month, space-padded (` 1`-`31`)
pub(crate) fn fmt_e(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Space, 2, date.day())
}

/// Week-based year, last two digits (`00`-`99`)
pub(crate) fn fmt_g(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, date.iso_year_week().0.rem_euclid(100))
}

/// Week-based year
pub(crate) fn fmt_G(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, date.iso_year_week().0)
}

/// Day of the year, zero-padded to width 3 (`000`-`366`)
pub(crate) fn fmt_j(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Zero, 3, date.ordinal())
}

/// Month of the year, zero-padded (`01`-`12`)
pub(crate) fn fmt_m(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, date.month())
}

/// ISO weekday (Monday = 1, Sunday = 7)
pub(crate) fn fmt_u(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    write!(f, "{}", date.weekday().iso_weekday_number())
}

/// ISO week number, zero-padded (`00`-`53`)
pub(crate) fn fmt_V(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, date.week())
}

/// Weekday number (Sunday = 0, Saturday = 6)
pub(crate) fn fmt_w(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    write!(f, "{}", date.weekday().number_days_from_sunday())
}

/// Last two digits of year (`00`-`99`)
pub(crate) fn fmt_y(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(Zero, 2, date.year().rem_euclid(100))
}

/// Full year
pub(crate) fn fmt_Y(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    let year = date.year();

    if year >= 10_000 {
        f.write_str("+")?;
    }

    pad!(Zero, 4, year)
}
