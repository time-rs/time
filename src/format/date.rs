//! Formatting helpers for a `Date`.

#![allow(non_snake_case)]

use crate::format::{Language, Padding};
use crate::Date;
use crate::Weekday::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};
use core::fmt::{self, Formatter};

macro_rules! nonexhaustive_locale {
    () => {
        unreachable!(
            "This is needed due to the explicit nonexhaustiveness of locales. \
             Once rust-lang/rust#44109 is resolved and `#[non_exhaustive]` is \
             stabilized, this will no longer be necessary."
        )
    };
}

/// Short day of the week
pub(crate) fn fmt_a(f: &mut Formatter<'_>, date: Date, locale: Language) -> fmt::Result {
    use Language::*;
    match (locale, date.weekday()) {
        (en, Monday) => f.write_str("Mon"),
        (en, Tuesday) => f.write_str("Tue"),
        (en, Wednesday) => f.write_str("Wed"),
        (en, Thursday) => f.write_str("Thu"),
        (en, Friday) => f.write_str("Fri"),
        (en, Saturday) => f.write_str("Sat"),
        (en, Sunday) => f.write_str("Sun"),

        (es, Monday) => f.write_str("Lu"),
        (es, Tuesday) => f.write_str("Ma"),
        (es, Wednesday) => f.write_str("Mi"),
        (es, Thursday) => f.write_str("Ju"),
        (es, Friday) => f.write_str("Vi"),
        (es, Saturday) => f.write_str("Sa"),
        (es, Sunday) => f.write_str("Do"),

        _ => nonexhaustive_locale!(),
    }
}

/// Day of the week
pub(crate) fn fmt_A(f: &mut Formatter<'_>, date: Date, locale: Language) -> fmt::Result {
    use Language::*;
    match (locale, date.weekday()) {
        (en, Monday) => f.write_str("Monday"),
        (en, Tuesday) => f.write_str("Tuesday"),
        (en, Wednesday) => f.write_str("Wednesday"),
        (en, Thursday) => f.write_str("Thursday"),
        (en, Friday) => f.write_str("Friday"),
        (en, Saturday) => f.write_str("Saturday"),
        (en, Sunday) => f.write_str("Sunday"),

        (es, Monday) => f.write_str("lunes"),
        (es, Tuesday) => f.write_str("martes"),
        (es, Wednesday) => f.write_str("miércoles"),
        (es, Thursday) => f.write_str("jueves"),
        (es, Friday) => f.write_str("viernes"),
        (es, Saturday) => f.write_str("sábado"),
        (es, Sunday) => f.write_str("domingo"),

        _ => nonexhaustive_locale!(),
    }
}

/// Short month name
///
/// References on localization
/// - [Yale](https://web.library.yale.edu/cataloging/months)
/// - [Princeton](https://library.princeton.edu/departments/tsd/katmandu/reference/months.html)
pub(crate) fn fmt_b(f: &mut Formatter<'_>, date: Date, locale: Language) -> fmt::Result {
    use Language::*;
    match (locale, date.month()) {
        (en, 1) => f.write_str("Jan"),
        (en, 2) => f.write_str("Feb"),
        (en, 3) => f.write_str("Mar"),
        (en, 4) => f.write_str("Apr"),
        (en, 5) => f.write_str("May"),
        (en, 6) => f.write_str("June"),
        (en, 7) => f.write_str("July"),
        (en, 8) => f.write_str("Aug"),
        (en, 9) => f.write_str("Sept"),
        (en, 10) => f.write_str("Oct"),
        (en, 11) => f.write_str("Nov"),
        (en, 12) => f.write_str("Dec"),

        (es, 1) => f.write_str("enero"),
        (es, 2) => f.write_str("feb"),
        (es, 3) => f.write_str("marzo"),
        (es, 4) => f.write_str("abr"),
        (es, 5) => f.write_str("mayo"),
        (es, 6) => f.write_str("jun"),
        (es, 7) => f.write_str("jul"),
        (es, 8) => f.write_str("agosto"),
        (es, 9) => f.write_str("set"),
        (es, 10) => f.write_str("oct"),
        (es, 11) => f.write_str("nov"),
        (es, 12) => f.write_str("dic"),

        _ => unreachable!("There are only 12 months in a year."),
    }
}

/// Month name
pub(crate) fn fmt_B(f: &mut Formatter<'_>, date: Date, locale: Language) -> fmt::Result {
    use Language::*;
    match (locale, date.month()) {
        (en, 1) => f.write_str("January"),
        (en, 2) => f.write_str("February"),
        (en, 3) => f.write_str("March"),
        (en, 4) => f.write_str("April"),
        (en, 5) => f.write_str("May"),
        (en, 6) => f.write_str("June"),
        (en, 7) => f.write_str("July"),
        (en, 8) => f.write_str("August"),
        (en, 9) => f.write_str("September"),
        (en, 10) => f.write_str("October"),
        (en, 11) => f.write_str("November"),
        (en, 12) => f.write_str("December"),

        (es, 1) => f.write_str("enero"),
        (es, 2) => f.write_str("febrero"),
        (es, 3) => f.write_str("marzo"),
        (es, 4) => f.write_str("abril"),
        (es, 5) => f.write_str("mayo"),
        (es, 6) => f.write_str("junio"),
        (es, 7) => f.write_str("julio"),
        (es, 8) => f.write_str("agosto"),
        (es, 9) => f.write_str("septiembre"),
        (es, 10) => f.write_str("octubre"),
        (es, 11) => f.write_str("noviembre"),
        (es, 12) => f.write_str("diciembre"),

        _ => unreachable!("There are only 12 months in a year."),
    }
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
