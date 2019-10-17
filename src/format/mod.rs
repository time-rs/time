//! Formatting for `Date`, `Time`, `DateTime`, and `OffsetDateTime`.

mod date;
mod format;
mod offset;
mod time;

use crate::{Date, Time, UtcOffset};
use alloc::{string::String, vec::Vec};
use core::fmt::{self, Display, Formatter};
pub(crate) use format::parse_with_language;

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
///
// The list of supported languages is inherently non-exhaustive. Once
// `#[non_exhaustive]` is stabilized, that will be used.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// English
    en,
    /// Spanish
    es,

    #[doc(hidden)]
    __nonexhaustive,
}

/*
TODO
/// The type of padding to use when formatting.
enum Padding {
    /// No padding. Minimizes width.
    None,
    /// Pad to the requisite width using spaces.
    Space,
    /// Pad to the requisite width using zeros.
    Zero,
}
*/

/// Specifiers are similar to C's `strftime`, with some omissions.
///
/// Explicitly not implemented:
/// - `%h` - Prefer `%b`.
/// - `%n` - Prefer `\n`.
/// - `%t` - Prefer `\t`.
/// - `%x` - Date format can vary within locales.
/// - `%X` - Time format can vary within locales.
///
/// Currently not implemented, but will be in the future (additional internal
/// methods are needed):
/// - `%U` - Sunday-based week number from start of year (can be zero).
/// - `%W` - Monday-based week number from start of year (can be zero).
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Specifier {
    /// Abbreviated weekday name
    a {
        /// The language used for the formatting.
        language: Language,
    },
    /// Full weekday name
    A {
        /// The language used for the formatting.
        language: Language,
    },
    /// Abbreviated month name
    b {
        /// The language used for the formatting.
        language: Language,
    },
    /// Full month name
    B {
        /// The language used for the formatting.
        language: Language,
    },
    /// Date and time representation
    c {
        /// The language used for the formatting.
        language: Language,
    },
    /// Year divided by 100 and truncated to integer (`00`-`99`)
    C,
    /// Day of the month, zero-padded (`01`-`31`)
    d,
    /// Short MM/DD/YY date, equivalent to `%m/%d/%y`
    D,
    /// Day of the month, space-padded (` 1`-`31`)
    e,
    /// Short YYYY-MM-DD date, equivalent to `%Y-%m-%d`
    F,
    /// Week-based year, last two digits (`00`-`99`)
    g,
    /// Week-based year
    G,
    /// Hour in 24h format (`00`-`23`)
    H,
    /// Hour in 12h format (`01`-`12`)
    I,
    /// Day of the year (`001`-`366`)
    j,
    /// Month as a decimal number (`01`-`12`)
    m,
    /// Minute (`00`-`59`)
    M,
    /// `am` or `pm` designation
    p,
    /// `AM` or `PM` designation
    P,
    /// 12-hour clock time
    r,
    /// 24-hour HH:MM time, equivalent to `%H:%M`
    R,
    /// Second (`00`-`59`)
    S,
    /// ISO 8601 time format (HH:MM:SS), equivalent to `%H:%M:%S`
    T,
    /// ISO 8601 weekday as number with Monday as 1 (`1`-`7`)
    u,
    /// Week number with the first Sunday as the first day of week one (`00`-`53`)
    U,
    /// ISO 8601 week number (`01`-`53`)
    V,
    /// Weekday as a decimal number with Sunday as 0 (`0`-`6`)
    w,
    /// Week number with the first Monday as the first day of week one (`00`-`53`)
    W,
    /// Year, last two digits (`00`-`99`)
    y,
    /// Year
    Y,
    /// UTC offset
    z,
}

/// Given all the information necessary, write the provided specifier to the
/// formatter.
fn format_specifier(
    f: &mut Formatter<'_>,
    date: Option<Date>,
    time: Option<Time>,
    offset: Option<UtcOffset>,
    specifier: Specifier,
) -> fmt::Result {
    /// Push the provided specifier to the list of items. If an asterisk is
    /// present, the language will be provided to the method.
    macro_rules! specifier {
        ($type:ident, $specifier:ident) => {
            paste::expr! {
                $type::[<fmt_ $specifier>](
                    f,
                    $type.expect(concat!(
                        "Specifier `%",
                        stringify!($specifier),
                        "` requires a ",
                        stringify!($type),
                        " to be present."
                    ))
                )?
            }
        };

        ($type:ident, $specifier:ident *) => {
            paste::expr! {
                $type::[<fmt_ $specifier>](
                    f,
                    $type.expect(concat!(
                        "Specifier `%",
                        stringify!($specifier),
                        "` requires a ",
                        stringify!($type),
                        " to be present."
                    )),
                    language
                )?
            }
        };
    }

    macro_rules! literal {
        ($string:literal) => {
            f.write_str($string)?
        };
    }

    use Specifier::*;
    match specifier {
        a { language } => specifier!(date, a*),
        A { language } => specifier!(date, A*),
        b { language } => specifier!(date, b*),
        B { language } => specifier!(date, B*),
        c { language } => {
            specifier!(date, a*);
            literal!(" ");
            specifier!(date, b*);
            literal!(" ");
            specifier!(date, e); // TODO trim on left
            literal!(" ");
            specifier!(time, H);
            literal!(":");
            specifier!(time, M);
            literal!(":");
            specifier!(time, S);
            literal!(" ");
            specifier!(date, Y);
        }
        C => specifier!(date, C),
        d => specifier!(date, d),
        D => {
            specifier!(date, m);
            literal!("/");
            specifier!(date, d);
            literal!("/");
            specifier!(date, y);
        }
        e => specifier!(date, e),
        F => {
            specifier!(date, Y);
            literal!("-");
            specifier!(date, m);
            literal!("-");
            specifier!(date, d);
        }
        g => specifier!(date, g),
        G => specifier!(date, G),
        H => specifier!(time, H),
        I => specifier!(time, I),
        j => specifier!(date, j),
        m => specifier!(date, m),
        M => specifier!(time, M),
        p => specifier!(time, p),
        P => specifier!(time, P),
        r => {
            specifier!(time, I);
            literal!(":");
            specifier!(time, M);
            literal!(":");
            specifier!(time, S);
            literal!(" ");
            specifier!(time, p);
        }
        R => {
            specifier!(time, H);
            literal!(":");
            specifier!(time, M);
        }
        S => specifier!(time, S),
        T => {
            specifier!(time, H);
            literal!(":");
            specifier!(time, M);
            literal!(":");
            specifier!(time, S);
        }
        u => specifier!(date, u),
        U => unimplemented!(), // Week number, first Sunday is first day of week one (TODO)
        V => specifier!(date, V),
        w => specifier!(date, w),
        W => unimplemented!(), // Week number, first Monday is first day of week one (TODO)
        y => specifier!(date, y),
        Y => specifier!(date, Y),
        z => specifier!(offset, z),
    }

    Ok(())
}

/// An enum that can store both literals and specifiers.
#[allow(variant_size_differences, clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FormatItem {
    /// A value that should be printed as-is.
    Literal(String),
    /// A value that needs to be interpreted when formatting.
    Specifier(Specifier),
}

// TODO Eliminate `DeferredFormat` entirely. None of the exposed formatting
// methods are deferred.
/// A struct containing all the necessary information to display the inner type.
#[allow(clippy::module_name_repetitions)]
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct DeferredFormat {
    /// The `Date` to use for formatting.
    pub(crate) date: Option<Date>,
    /// The `Time` to use for formatting.
    pub(crate) time: Option<Time>,
    /// The `UtcOffset` to use for formatting.
    pub(crate) offset: Option<UtcOffset>,
    /// The list of items used to display the item.
    pub(crate) format: Vec<FormatItem>,
}

impl Display for DeferredFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in &self.format {
            match item {
                FormatItem::Literal(value) => write!(f, "{}", value)?,
                FormatItem::Specifier(specifier) => {
                    format_specifier(f, self.date, self.time, self.offset, *specifier)?
                }
            }
        }

        Ok(())
    }
}
