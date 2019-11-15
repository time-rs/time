//! The `Language` struct and its various methods.

/// Languages used in formatting. Follows [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
///
/// Additional languages may be added at any time. Contributions will be
/// accepted by native and highly fluent speakers of any living language.
///
/// All languages must have the following:
/// - Month names
/// - Short month names
/// - Weekday names
/// - Short weekday names
#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// English
    en,
    /// French
    fr,
    /// Spanish
    es,
}

#[allow(clippy::non_ascii_literal)]
impl Language {
    /// Get the month names for the given language.
    #[inline(always)]
    pub fn month_names(self) -> [&'static str; 12] {
        use Language::*;
        match self {
            en => [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ],
            fr => [
                "Janvier",
                "Février",
                "Mars",
                "Avril",
                "Mai",
                "Juin",
                "Juillet",
                "Août",
                "Septembre",
                "Octobre",
                "Novembre",
                "Décembre",
            ],
            es => [
                "enero",
                "febrero",
                "marzo",
                "abril",
                "mayo",
                "junio",
                "julio",
                "agosto",
                "septiembre",
                "octubre",
                "noviembre",
                "diciembre",
            ],
        }
    }

    /// Get the abbreviated month names for the given language.
    ///
    /// References on localization:
    /// [\[1\]](https://web.library.yale.edu/cataloging/months)
    /// [\[2\]](https://library.princeton.edu/departments/tsd/katmandu/reference/months.html)
    #[inline(always)]
    pub fn short_month_names(self) -> [&'static str; 12] {
        use Language::*;
        match self {
            en => [
                "Jan", "Feb", "Mar", "Apr", "May", "June", "July", "Aug", "Sept", "Oct", "Nov",
                "Dec",
            ],
            fr => [
                "Janv", "Févr", "Mars", "Avr", "Mai", "Juin", "Juil", "Août", "Sept", "Oct", "Nov",
                "Déc",
            ],
            es => [
                "enero", "feb", "marzo", "abr", "mayo", "jun", "jul", "agosto", "set", "oct",
                "nov", "dic",
            ],
        }
    }

    /// Get the names of days of the week for the given language. Starts with
    /// Monday.
    #[inline(always)]
    pub fn week_days(self) -> [&'static str; 7] {
        use Language::*;
        match self {
            en => [
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
                "Sunday",
            ],
            fr => [
                "Lundi",
                "Mardi",
                "Mercredi",
                "Jeudi",
                "Vendredi",
                "Samedi",
                "Dimanche",
            ],
            es => [
                "lunes",
                "martes",
                "miércoles",
                "jueves",
                "viernes",
                "sábado",
                "domingo",
            ],
        }
    }

    /// Get the abbreviated names of days of the week for the given language.
    /// Starts with Monday.
    #[inline(always)]
    pub fn short_week_days(self) -> [&'static str; 7] {
        use Language::*;
        match self {
            en => ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
            fr => ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"],
            es => ["Lu", "Ma", "Mi", "Ju", "Vi", "Sa", "Do"],
        }
    }
}
