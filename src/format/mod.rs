//! Parsing and formatting various types.

/// Pad a given value if requested.
macro_rules! pad {
    ($f:ident, $padding:ident(None), $width:literal, $value:expr) => {
        match $padding {
            Padding::None | Padding::Default => write!($f, "{}", $value),
            Padding::Space => write!($f, concat!("{:", stringify!($width), "}"), $value),
            Padding::Zero => write!($f, concat!("{:0", stringify!($width), "}"), $value),
        }
    };

    ($f:ident, $padding:ident(Space), $width:literal, $value:expr) => {
        match $padding {
            Padding::None => write!($f, "{}", $value),
            Padding::Space | Padding::Default => {
                write!($f, concat!("{:", stringify!($width), "}"), $value)
            }
            Padding::Zero => write!($f, concat!("{:0", stringify!($width), "}"), $value),
        }
    };

    ($f:ident, $padding:ident(Zero), $width:literal, $value:expr) => {
        match $padding {
            Padding::None => write!($f, "{}", $value),
            Padding::Space => write!($f, concat!("{:", stringify!($width), "}"), $value),
            Padding::Zero | Padding::Default => {
                write!($f, concat!("{:0", stringify!($width), "}"), $value)
            }
        }
    };
}

pub(crate) mod date;
pub(crate) mod offset;
pub(crate) mod parse;
pub(crate) mod parse_items;
pub(crate) mod time;

use crate::internal_prelude::*;
use core::fmt::{self, Display, Formatter};
#[allow(unreachable_pub)] // rust-lang/rust#64762
pub use parse::ParseError;
pub(crate) use parse::{parse, ParseResult, ParsedItems};
pub(crate) use parse_items::{parse_fmt_string, try_parse_fmt_string};

/// Checks if a user-provided formatting string is valid. If it isn't, a
/// description of the error is returned.
#[inline(always)]
pub fn validate_format_string(s: impl AsRef<str>) -> Result<(), String> {
    try_parse_fmt_string(s.as_ref()).map(|_| ())
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
    /// Use the default padding for the specifier. Varies by specifier.
    Default,
}

impl Padding {
    /// Map the default value to a provided alternative.
    #[inline(always)]
    pub(crate) fn default_to(self, value: Self) -> Self {
        match self {
            Padding::Default => value,
            _ => self,
        }
    }
}

/// Specifiers are similar to C's `strftime`, with some omissions and changes.
///
/// See the table in `lib.rs` for a description of each specifier (and
/// equivalences for combination specifiers).
#[allow(
    non_snake_case,
    non_camel_case_types,
    clippy::missing_docs_in_private_items
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Specifier {
    a,
    A,
    b,
    B,
    c,
    C { padding: Padding },
    d { padding: Padding },
    D,
    F,
    g { padding: Padding },
    G { padding: Padding },
    H { padding: Padding },
    I { padding: Padding },
    j { padding: Padding },
    m { padding: Padding },
    M { padding: Padding },
    N,
    p,
    P,
    r,
    R,
    S { padding: Padding },
    T,
    u,
    U { padding: Padding },
    V { padding: Padding },
    w,
    W { padding: Padding },
    y { padding: Padding },
    Y { padding: Padding },
    z,
}

/// Given all the information necessary, write the provided specifier to the
/// formatter.
#[inline]
fn format_specifier(
    f: &mut Formatter<'_>,
    date: Option<Date>,
    time: Option<Time>,
    offset: Option<UtcOffset>,
    specifier: Specifier,
) -> fmt::Result {
    /// Push the provided specifier to the list of items.
    macro_rules! specifier {
        ($type:ident :: $specifier_fn:ident ( $specifier:ident $(, $param:expr)? )) => {
            $type::$specifier_fn(
                f,
                $type.expect(concat!(
                    "Specifier `%",
                    stringify!($specifier),
                    "` requires a ",
                    stringify!($type),
                    " to be present."
                )),
                $($param)?
            )?
        };
    }

    macro_rules! literal {
        ($string:literal) => {
            f.write_str($string)?
        };
    }

    use Specifier::*;
    match specifier {
        a => specifier!(date::fmt_a(a)),
        A => specifier!(date::fmt_A(A)),
        b => specifier!(date::fmt_b(b)),
        B => specifier!(date::fmt_B(B)),
        c => {
            specifier!(date::fmt_a(a));
            literal!(" ");
            specifier!(date::fmt_b(b));
            literal!(" ");
            specifier!(date::fmt_d(d, Padding::None));
            literal!(" ");
            specifier!(time::fmt_H(H, Padding::None));
            literal!(":");
            specifier!(time::fmt_M(M, Padding::Default));
            literal!(":");
            specifier!(time::fmt_S(S, Padding::Default));
            literal!(" ");
            specifier!(date::fmt_Y(Y, Padding::None));
        }
        C { padding } => specifier!(date::fmt_C(C, padding)),
        d { padding } => specifier!(date::fmt_d(d, padding)),
        D => {
            specifier!(date::fmt_m(m, Padding::None));
            literal!("/");
            specifier!(date::fmt_d(d, Padding::Default));
            literal!("/");
            specifier!(date::fmt_y(y, Padding::Default));
        }
        F => {
            specifier!(date::fmt_Y(Y, Padding::None));
            literal!("-");
            specifier!(date::fmt_m(m, Padding::Default));
            literal!("-");
            specifier!(date::fmt_d(d, Padding::Default));
        }
        g { padding } => specifier!(date::fmt_g(g, padding)),
        G { padding } => specifier!(date::fmt_G(G, padding)),
        H { padding } => specifier!(time::fmt_H(H, padding)),
        I { padding } => specifier!(time::fmt_I(I, padding)),
        j { padding } => specifier!(date::fmt_j(j, padding)),
        m { padding } => specifier!(date::fmt_m(m, padding)),
        M { padding } => specifier!(time::fmt_M(M, padding)),
        N => specifier!(time::fmt_N(N)),
        p => specifier!(time::fmt_p(p)),
        P => specifier!(time::fmt_P(P)),
        r => {
            specifier!(time::fmt_I(I, Padding::None));
            literal!(":");
            specifier!(time::fmt_M(M, Padding::Default));
            literal!(":");
            specifier!(time::fmt_S(S, Padding::Default));
            literal!(" ");
            specifier!(time::fmt_p(p));
        }
        R => {
            specifier!(time::fmt_H(H, Padding::None));
            literal!(":");
            specifier!(time::fmt_M(M, Padding::Default));
        }
        S { padding } => specifier!(time::fmt_S(S, padding)),
        T => {
            specifier!(time::fmt_H(H, Padding::None));
            literal!(":");
            specifier!(time::fmt_M(M, Padding::Default));
            literal!(":");
            specifier!(time::fmt_S(S, Padding::Default));
        }
        u => specifier!(date::fmt_u(u)),
        U { padding } => specifier!(date::fmt_U(U, padding)),
        V { padding } => specifier!(date::fmt_V(V, padding)),
        w => specifier!(date::fmt_w(w)),
        W { padding } => specifier!(date::fmt_W(W, padding)),
        y { padding } => specifier!(date::fmt_y(y, padding)),
        Y { padding } => specifier!(date::fmt_Y(Y, padding)),
        z => specifier!(offset::fmt_z(z)),
    }

    Ok(())
}

/// An enum that can store both literals and specifiers.
#[allow(variant_size_differences, single_use_lifetimes)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FormatItem<'a> {
    /// A value that should be printed as-is.
    Literal(&'a str),
    /// A value that needs to be interpreted when formatting.
    Specifier(Specifier),
}

/// A struct containing all the necessary information to display the inner type.
#[allow(single_use_lifetimes)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct DeferredFormat<'a> {
    /// The `Date` to use for formatting.
    date: Option<Date>,
    /// The `Time` to use for formatting.
    time: Option<Time>,
    /// The `UtcOffset` to use for formatting.
    offset: Option<UtcOffset>,
    /// The list of items used to display the item.
    format: Vec<FormatItem<'a>>,
}

impl<'a> DeferredFormat<'a> {
    /// Create a new `DeferredFormat` with the provided formatting string.
    pub(crate) fn new(format: &'a str) -> Self {
        Self {
            date: None,
            time: None,
            offset: None,
            format: parse_fmt_string(format),
        }
    }

    /// Provide the `Date` component.
    pub(crate) fn with_date(self, date: Date) -> Self {
        Self {
            date: Some(date),
            ..self
        }
    }

    /// Provide the `Time` component.
    pub(crate) fn with_time(self, time: Time) -> Self {
        Self {
            time: Some(time),
            ..self
        }
    }

    /// Provide the `UtCOffset` component.
    pub(crate) fn with_offset(self, offset: UtcOffset) -> Self {
        Self {
            offset: Some(offset),
            ..self
        }
    }
}

impl Display for DeferredFormat<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in &self.format {
            match item {
                FormatItem::Literal(value) => f.write_str(value)?,
                FormatItem::Specifier(specifier) => {
                    format_specifier(f, self.date, self.time, self.offset, *specifier)?
                }
            }
        }

        Ok(())
    }
}
