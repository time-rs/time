//! Formatting for `Date`, `Time`, `DateTime`, and `OffsetDateTime`.

/// Pad a given value if requested.
macro_rules! pad {
    (None, $width:literal, $value:expr) => {
        paste::expr! {
            match [<padding>] {
                Padding::None | Padding::Default => write!([<f>], "{}", $value),
                Padding::Space => write!([<f>], concat!("{:", stringify!($width), "}"), $value),
                Padding::Zero => write!([<f>], concat!("{:0", stringify!($width), "}"), $value),
            }
        }
    };

    (Space, $width:literal, $value:expr) => {
        paste::expr! {
            match [<padding>] {
                Padding::None => write!([<f>], "{}", $value),
                Padding::Space | Padding::Default => write!([<f>], concat!("{:", stringify!($width), "}"), $value),
                Padding::Zero => write!([<f>], concat!("{:0", stringify!($width), "}"), $value),
            }
        }
    };

    (Zero, $width:literal, $value:expr) => {
        paste::expr! {
            match [<padding>] {
                Padding::None => write!([<f>], "{}", $value),
                Padding::Space => write!([<f>], concat!("{:", stringify!($width), "}"), $value),
                Padding::Zero | Padding::Default => write!([<f>], concat!("{:0", stringify!($width), "}"), $value),
            }
        }
    };
}

mod date;
mod format;
mod offset;
mod time;

#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{Date, Time, UtcOffset};
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

/// The type of padding to use when formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Padding {
    /// No padding. Minimizes width.
    None,
    /// Pad to the requisite width using spaces.
    Space,
    /// Pad to the requisite width using zeros.
    Zero,
    /// Use the default padding for the specifier.
    Default,
}

/// Specifiers are similar to C's `strftime`, with some omissions and changes.
#[allow(
    non_snake_case,
    non_camel_case_types,
    clippy::missing_docs_in_private_items
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Specifier {
    /// Abbreviated weekday name
    a { language: Language },
    /// Full weekday name
    A { language: Language },
    /// Abbreviated month name
    b { language: Language },
    /// Full month name
    B { language: Language },
    /// Date and time representation
    c { language: Language },
    /// Year divided by 100 and truncated to integer (`00`-`99`)
    C { padding: Padding },
    /// Day of the month, zero-padded (`01`-`31`)
    d { padding: Padding },
    /// Short MM/DD/YY date, equivalent to `%m/%d/%y`
    D,
    /// Day of the month, space-padded (` 1`-`31`)
    e { padding: Padding },
    /// Short YYYY-MM-DD date, equivalent to `%Y-%m-%d`
    F,
    /// Week-based year, last two digits (`00`-`99`)
    g { padding: Padding },
    /// Week-based year
    G { padding: Padding },
    /// Hour in 24h format (`00`-`23`)
    H { padding: Padding },
    /// Hour in 12h format (`01`-`12`)
    I { padding: Padding },
    /// Day of the year (`001`-`366`)
    j { padding: Padding },
    /// Month as a decimal number (`01`-`12`)
    m { padding: Padding },
    /// Minute (`00`-`59`)
    M { padding: Padding },
    /// `am` or `pm` designation
    p,
    /// `AM` or `PM` designation
    P,
    /// 12-hour clock time
    r,
    /// 24-hour HH:MM time, equivalent to `%H:%M`
    R,
    /// Second (`00`-`59`)
    S { padding: Padding },
    /// ISO 8601 time format (HH:MM:SS), equivalent to `%H:%M:%S`
    T,
    /// ISO 8601 weekday as number with Monday as 1 (`1`-`7`)
    u,
    /// Week number with the first Sunday as the first day of week one (`00`-`53`)
    U { padding: Padding },
    /// ISO 8601 week number (`01`-`53`)
    V { padding: Padding },
    /// Weekday as a decimal number with Sunday as 0 (`0`-`6`)
    w,
    /// Week number with the first Monday as the first day of week one (`00`-`53`)
    W { padding: Padding },
    /// Year, last two digits (`00`-`99`)
    y { padding: Padding },
    /// Year
    Y { padding: Padding },
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
    /// present, the language will be provided to the method. If a pound symbol
    /// is present, the padding will be provided.
    macro_rules! specifier {
        ($type:ident, $specifier:ident $(, $opt:ident)*) => {
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
                    $($opt),*
                )?
            }
        };
    }

    macro_rules! literal {
        ($string:literal) => {
            f.write_str($string)?
        };
    }

    // Identifiers to allow function-like macros.
    #[allow(clippy::missing_docs_in_private_items)]
    const DEFAULT_PADDING: Padding = Padding::Default;
    #[allow(clippy::missing_docs_in_private_items)]
    const NONE_PADDING: Padding = Padding::None;

    use Specifier::*;
    match specifier {
        a { language } => specifier!(date, a, language),
        A { language } => specifier!(date, A, language),
        b { language } => specifier!(date, b, language),
        B { language } => specifier!(date, B, language),
        c { language } => {
            specifier!(date, a, language);
            literal!(" ");
            specifier!(date, b, language);
            literal!(" ");
            specifier!(date, e, NONE_PADDING);
            literal!(" ");
            specifier!(time, H, DEFAULT_PADDING);
            literal!(":");
            specifier!(time, M, DEFAULT_PADDING);
            literal!(":");
            specifier!(time, S, DEFAULT_PADDING);
            literal!(" ");
            specifier!(date, Y, DEFAULT_PADDING);
        }
        C { padding } => specifier!(date, C, padding),
        d { padding } => specifier!(date, d, padding),
        D => {
            specifier!(date, m, DEFAULT_PADDING);
            literal!("/");
            specifier!(date, d, DEFAULT_PADDING);
            literal!("/");
            specifier!(date, y, DEFAULT_PADDING);
        }
        e { padding } => specifier!(date, e, padding),
        F => {
            specifier!(date, Y, DEFAULT_PADDING);
            literal!("-");
            specifier!(date, m, DEFAULT_PADDING);
            literal!("-");
            specifier!(date, d, DEFAULT_PADDING);
        }
        g { padding } => specifier!(date, g, padding),
        G { padding } => specifier!(date, G, padding),
        H { padding } => specifier!(time, H, padding),
        I { padding } => specifier!(time, I, padding),
        j { padding } => specifier!(date, j, padding),
        m { padding } => specifier!(date, m, padding),
        M { padding } => specifier!(time, M, padding),
        p => specifier!(time, p),
        P => specifier!(time, P),
        r => {
            specifier!(time, I, DEFAULT_PADDING);
            literal!(":");
            specifier!(time, M, DEFAULT_PADDING);
            literal!(":");
            specifier!(time, S, DEFAULT_PADDING);
            literal!(" ");
            specifier!(time, p);
        }
        R => {
            specifier!(time, H, DEFAULT_PADDING);
            literal!(":");
            specifier!(time, M, DEFAULT_PADDING);
        }
        S { padding } => specifier!(time, S, padding),
        T => {
            specifier!(time, H, DEFAULT_PADDING);
            literal!(":");
            specifier!(time, M, DEFAULT_PADDING);
            literal!(":");
            specifier!(time, S, DEFAULT_PADDING);
        }
        u => specifier!(date, u),
        U { .. } => unimplemented!(), // Week number, first Sunday is first day of week one (TODO)
        V { padding } => specifier!(date, V, padding),
        w => specifier!(date, w),
        W { .. } => unimplemented!(), // Week number, first Monday is first day of week one (TODO)
        y { padding } => specifier!(date, y, padding),
        Y { padding } => specifier!(date, Y, padding),
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
