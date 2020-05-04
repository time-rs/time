//! Simple time handling.
//!
//! ![rustc 1.36.0](https://img.shields.io/badge/rustc-1.36.0-blue)
//!
//! # Feature flags in Cargo
//!
//! ## `std`
//!
//! Currently, all structs except `Instant` can be used with `#![no_std]`. As
//! support for the standard library is enabled by default, you must use
//! `default_features = false` in your `Cargo.toml` to enable this.
//!
//! ```toml
//! [dependencies]
//! time = { version = "0.2", default-features = false }
//! ```
//!
//! Of the structs that are usable, some methods may only be enabled due a
//! reliance on `Instant`. These will be indicated in the documentation.
//!
//! ## `serde`
//!
//! [Serde](https://github.com/serde-rs/serde) support is behind a feature flag.
//! To enable it, use the `serde` feature. This is not enabled by default. It
//! _is_ compatible with `#![no_std]`, so long as an allocator is present.
//!
//! With the standard library:
//! ```toml
//! [dependencies]
//! time = { version = "0.2", features = ["serde"] }
//! ```
//!
//! With `#![no_std]` support:
//! ```toml
//! [dependencies]
//! time = { version = "0.2", default-features = false, features = ["serde"] }
//! ```
//!
//! **Please note that data _is not verified_ when deserializing. If invalid
//! data is provided, the resulting struct will not be in a valid state. It is,
//! however, guaranteed that a round-trip serialize-deserialize will result in a
//! valid state.
//!
//! ## `rand`
//!
//! [Rand](https://github.com/rust-random/rand) support is behind a feature
//! flag. To enable it, use the `rand` feature. This is not enabled by default.
//! Usage is compatible with `#![no_std]`.
//!
//! With the standard library:
//! ```toml
//! [dependencies]
//! time = { version = "0.2", features = ["rand"] }
//! ```
//!
//! With `#![no_std]` support:
//! ```toml
//! [dependencies]
//! time = { version = "0.2", default-features = false, features = ["rand"] }
//! ```
//!
//! # Formatting
//!
//! Time's formatting behavior is based on `strftime` in C, though it is
//! explicitly _not_ compatible. Specifiers may be missing, added, or have
//! different behavior than in C. As such, you should use the table below, which
//! is an up-to-date reference on what each specifier does.
//!
//! | Specifier | Replaced by                                                            | Example                    |
//! |-----------|------------------------------------------------------------------------|----------------------------|
//! | `%a`      | Abbreviated weekday name                                               | `Thu`                      |
//! | `%A`      | Full weekday name                                                      | `Thursday`                 |
//! | `%b`      | Abbreviated month name                                                 | `Aug`                      |
//! | `%B`      | Full month name                                                        | `August`                   |
//! | `%c`      | Date and time representation, equivalent to `%a %b %-d %-H:%M:%S %-Y`  | `Thu Aug 23 14:55:02 2001` |
//! | `%C`      | Year divided by 100 and truncated to integer (`00`-`99`)               | `20`                       |
//! | `%d`      | Day of the month, zero-padded (`01`-`31`)                              | `23`                       |
//! | `%D`      | Short MM/DD/YY date, equivalent to `%-m/%d/%y`                         | `8/23/01`                  |
//! | `%F`      | Short YYYY-MM-DD date, equivalent to `%-Y-%m-%d`                       | `2001-08-23`               |
//! | `%g`      | Week-based year, last two digits (`00`-`99`)                           | `01`                       |
//! | `%G`      | Week-based year                                                        | `2001`                     |
//! | `%H`      | Hour in 24h format (`00`-`23`)                                         | `14`                       |
//! | `%I`      | Hour in 12h format (`01`-`12`)                                         | `02`                       |
//! | `%j`      | Day of the year (`001`-`366`)                                          | `235`                      |
//! | `%m`      | Month as a decimal number (`01`-`12`)                                  | `08`                       |
//! | `%M`      | Minute (`00`-`59`)                                                     | `55`                       |
//! | `%N`      | Subsecond nanoseconds. Always 9 digits                                 | `012345678`                |
//! | `%p`      | `am` or `pm` designation                                               | `pm`                       |
//! | `%P`      | `AM` or `PM` designation                                               | `PM`                       |
//! | `%r`      | 12-hour clock time, equivalent to `%-I:%M:%S %p`                       | `2:55:02 pm`               |
//! | `%R`      | 24-hour HH:MM time, equivalent to `%-H:%M`                             | `14:55`                    |
//! | `%S`      | Second (`00`-`59`)                                                     | `02`                       |
//! | `%T`      | 24-hour clock time with seconds, equivalent to `%-H:%M:%S`             | `14:55:02`                 |
//! | `%u`      | ISO 8601 weekday as number with Monday as 1 (`1`-`7`)                  | `4`                        |
//! | `%U`      | Week number with the first Sunday as the start of week one (`00`-`53`) | `33`                       |
//! | `%V`      | ISO 8601 week number (`01`-`53`)                                       | `34`                       |
//! | `%w`      | Weekday as a decimal number with Sunday as 0 (`0`-`6`)                 | `4`                        |
//! | `%W`      | Week number with the first Monday as the start of week one (`00`-`53`) | `34`                       |
//! | `%y`      | Year, last two digits (`00`-`99`)                                      | `01`                       |
//! | `%Y`      | Full year, including `+` if ≥10,000                                    | `2001`                     |
//! | `%z`      | ISO 8601 offset from UTC in timezone (+HHMM)                           | `+0100`                    |
//! | `%%`      | Literal `%`                                                            | `%`                        |
//!
//! ## Modifiers
//!
//! All specifiers that are strictly numerical have modifiers for formatting.
//! Adding a modifier to a non-supporting specifier is a no-op.
//!
//! <!-- rust-lang/rust#65613 -->
//! <style>.docblock code { white-space: pre-wrap; }</style>
//!
//! | Modifier         | Behavior        | Example       |
//! |------------------|-----------------|---------------|
//! | `-` (dash)       | No padding      | `%-d` => `5`  |
//! | `_` (underscore) | Pad with spaces | `%_d` => ` 5` |
//! | `0`              | Pad with zeros  | `%0d` => `05` |

#![cfg_attr(docs, feature(doc_cfg))]
#![cfg_attr(not(std), no_std)]
#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::option_unwrap_used,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::result_unwrap_used,
    clippy::todo,
    clippy::unimplemented,
    clippy::use_debug,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::enum_glob_use,
    clippy::inline_always,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::suspicious_arithmetic_impl,
    clippy::use_self,
    clippy::wildcard_imports,
    clippy::zero_prefixed_literal,
    unstable_name_collisions
)]
#![cfg_attr(
    test,
    allow(
        clippy::cognitive_complexity,
        clippy::too_many_lines,
        clippy::neg_multiply
    )
)]
#![doc(html_favicon_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(test(attr(deny(warnings))))]

extern crate alloc;

/// Returns `Err(ComponentRangeError)` if the value is not in range.
macro_rules! ensure_value_in_range {
    ($value:ident in $start:expr => $end:expr) => {
        if !($start..=$end).contains(&$value) {
            return Err(ComponentRangeError {
                name: stringify!($value),
                minimum: i64::from($start),
                maximum: i64::from($end),
                value: i64::from($value),
                given: Vec::new(),
            });
        }
    };

    ($value:ident in $start:expr => $end:expr, given $($conditional:ident),+ $(,)?) => {
        if !($start..=$end).contains(&$value) {
            return Err(ComponentRangeError {
                name: stringify!($value),
                minimum: i64::from($start),
                maximum: i64::from($end),
                value: i64::from($value),
                given: vec![$((stringify!($conditional), i64::from($conditional))),+],
            });
        }
    };
}

/// A macro to generate `Time`s at runtime, usable for tests.
#[cfg(test)]
macro_rules! time {
    ($hour:literal : $minute:literal) => {
        crate::Time::from_hms($hour, $minute, 0)?
    };
    ($hour:literal : $minute:literal : $second:literal) => {
        crate::Time::from_hms($hour, $minute, $second)?
    };
    ($hour:literal : $minute:literal : $second:literal : $nanosecond:literal) => {
        crate::Time::from_hms_nano($hour, $minute, $second, $nanosecond)?
    };
}

/// A macro to generate `UtcOffset`s with *no data verification*, usable for
/// tests.
#[cfg(test)]
macro_rules! offset {
    (UTC) => {
        crate::UtcOffset::UTC
    };
    ($(+)? $hour:literal) => {
        crate::internals::UtcOffset::seconds($hour * 3_600)
    };
    (+ $hour:literal : $minute:literal) => {
        crate::internals::UtcOffset::seconds($hour * 3_600 + $minute * 60)
    };
    (+ $hour:literal : $minute:literal : $second:literal) => {
        crate::internals::UtcOffset::seconds($hour * 3_600 + $minute * 60 + $second)
    };
    (- $hour:literal : $minute:literal) => {
        crate::internals::UtcOffset::seconds($hour * -3_600 - $minute * 60)
    };
    (- $hour:literal : $minute:literal : $second:literal) => {
        crate::internals::UtcOffset::seconds($hour * -3_600 - $minute * 60 - $second)
    };
}

/// A macro to generate `Date`s at runtime, usable for tests.
#[cfg(test)]
macro_rules! date {
    ($(+)? $year:literal - $ordinal:literal) => {
        crate::Date::from_yo($year, $ordinal)?
    };
    ($(+)? $year:literal - $month:literal - $day:literal) => {
        crate::Date::from_ymd($year, $month, $day)?
    };
}

/// The `Date` struct and its associated `impl`s.
mod date;
/// The `Duration` struct and its associated `impl`s.
mod duration;
/// Various error types returned by methods in the time crate.
mod error;
mod format;
/// The `Instant` struct and its associated `impl`s.
#[cfg(std)]
mod instant;
pub mod internals;
/// A collection of traits extending built-in numerical types.
mod numerical_traits;
/// The `OffsetDateTime` struct and its associated `impl`s.
mod offset_date_time;
/// The `PrimitiveDateTime` struct and its associated `impl`s.
mod primitive_date_time;
#[cfg(rand)]
mod rand;
#[cfg(serde)]
#[allow(missing_copy_implementations, missing_debug_implementations)]
mod serde;
/// The `Time` struct and its associated `impl`s.
mod time_mod;
/// The `UtcOffset` struct and its associated `impl`s.
mod utc_offset;
/// Days of the week.
mod weekday;

pub use date::{days_in_year, is_leap_year, weeks_in_year, Date};
pub use duration::Duration;
pub use error::{
    ComponentRangeError, ConversionRangeError, Error, FormatError, IndeterminateOffsetError,
};
pub(crate) use format::DeferredFormat;
pub use format::{validate_format_string, Format, ParseError};
#[cfg(std)]
pub use instant::Instant;
use internal_prelude::*;
pub use numerical_traits::{NumericalDuration, NumericalStdDuration, NumericalStdDurationShort};
pub use offset_date_time::OffsetDateTime;
pub use primitive_date_time::PrimitiveDateTime;
/// Construct a [`Date`] with a statically known value.
///
/// The resulting expression can be used in `const` or `static` declarations.
///
/// Three formats are supported: year-week-weekday, year-ordinal, and
/// year-month-day.
///
/// ```rust
/// # use time::{Date, date, Weekday::*};
/// assert_eq!(date!(2020-W01-3), Date::from_iso_ywd(2020, 1, Wednesday)?);
/// assert_eq!(date!(2020-001), Date::from_yo(2020, 1)?);
/// assert_eq!(date!(2020-01-01), Date::from_ymd(2020, 1, 1)?);
/// # Ok::<_, time::Error>(())
/// ```
#[cfg(macros)]
pub use time_macros::date;
/// Construct a [`UtcOffset`] with a statically known value.
///
/// The resulting expression can be used in `const` or `static` declarations.
///
/// A sign and the hour must be provided; minutes and seconds default to zero.
/// `UTC` (both uppercase and lowercase) is also allowed.
///
/// ```rust
/// # use time::{offset, UtcOffset};
/// assert_eq!(offset!(UTC), UtcOffset::hours(0)?);
/// assert_eq!(offset!(utc), UtcOffset::hours(0)?);
/// assert_eq!(offset!(+0), UtcOffset::hours(0)?);
/// assert_eq!(offset!(+1), UtcOffset::hours(1)?);
/// assert_eq!(offset!(-1), UtcOffset::hours(-1)?);
/// assert_eq!(offset!(+1:30), UtcOffset::minutes(90)?);
/// assert_eq!(offset!(-1:30), UtcOffset::minutes(-90)?);
/// assert_eq!(offset!(+1:30:59), UtcOffset::seconds(5459)?);
/// assert_eq!(offset!(-1:30:59), UtcOffset::seconds(-5459)?);
/// assert_eq!(offset!(+23:59:59), UtcOffset::seconds(86_399)?);
/// assert_eq!(offset!(-23:59:59), UtcOffset::seconds(-86_399)?);
/// # Ok::<_, time::Error>(())
/// ```
#[cfg(macros)]
pub use time_macros::offset;
/// Construct a [`Time`] with a statically known value.
///
/// The resulting expression can be used in `const` or `static` declarations.
///
/// Hours and minutes must be provided, while seconds defaults to zero. AM/PM is
/// allowed (either uppercase or lowercase). Any number of subsecond digits may
/// be provided (though any past nine will be discarded).
///
/// All components are validated at compile-time. An error will be raised if any
/// value is invalid.
///
/// ```rust
/// # use time::{Time, time};
/// assert_eq!(time!(0:00), Time::from_hms(0, 0, 0)?);
/// assert_eq!(time!(1:02:03), Time::from_hms(1, 2, 3)?);
/// assert_eq!(time!(1:02:03.004_005_006), Time::from_hms_nano(1, 2, 3, 4_005_006)?);
/// assert_eq!(time!(12:00 am), Time::from_hms(0, 0, 0)?);
/// assert_eq!(time!(1:02:03 am), Time::from_hms(1, 2, 3)?);
/// assert_eq!(time!(1:02:03.004_005_006 am), Time::from_hms_nano(1, 2, 3, 4_005_006)?);
/// assert_eq!(time!(12:00 pm), Time::from_hms(12, 0, 0)?);
/// assert_eq!(time!(1:02:03 pm), Time::from_hms(13, 2, 3)?);
/// assert_eq!(time!(1:02:03.004_005_006 pm), Time::from_hms_nano(13, 2, 3, 4_005_006)?);
/// # Ok::<_, time::Error>(())
/// ```
#[cfg(macros)]
pub use time_macros::time;
pub use time_mod::Time;
pub use utc_offset::UtcOffset;
pub use weekday::Weekday;

/// An alias for `Result` with a generic error from the time crate.
pub type Result<T> = core::result::Result<T, Error>;

/// A collection of imports that are widely useful.
///
/// Unlike the standard library, this must be explicitly imported:
///
/// ```rust,no_run
/// # #[allow(unused_imports)]
/// use time::prelude::*;
/// ```
///
/// The prelude may grow in minor releases. Any removals will only occur in
/// major releases.
pub mod prelude {
    #[cfg(macros)]
    pub use crate::{date, offset, time};
    pub use crate::{NumericalDuration as _, NumericalStdDuration as _};
}

/// Items generally useful in any file in the time crate.
mod internal_prelude {
    #![allow(unused_imports)]

    #[cfg(std)]
    pub(crate) use crate::Instant;
    pub(crate) use crate::{
        format::{ParseError, ParseResult},
        ComponentRangeError, ConversionRangeError, Date, DeferredFormat, Duration, Format,
        FormatError, IndeterminateOffsetError, NumericalDuration, NumericalStdDuration,
        OffsetDateTime, PrimitiveDateTime, Time, UtcOffset,
        Weekday::{self, Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday},
    };
    pub(crate) use alloc::{
        borrow::{Cow, ToOwned},
        boxed::Box,
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
    pub(crate) use core::convert::{TryFrom, TryInto};
    pub(crate) use standback::prelude::*;
}

#[allow(clippy::missing_docs_in_private_items)]
mod private {
    use super::*;

    macro_rules! parsable {
        ($($type:ty),* $(,)?) => {$(
            impl Parsable for $type {
                fn parse<'a>(
                    s: impl Into<Cow<'a, str>>,
                    format: impl Into<Format<'a>>,
                ) -> ParseResult<Self> {
                    Self::parse(s, format)
                }
            }
        )*};
    }

    pub trait Parsable: Sized {
        fn parse<'a>(
            s: impl Into<Cow<'a, str>>,
            format: impl Into<Format<'a>>,
        ) -> ParseResult<Self>;
    }

    parsable![Time, Date, UtcOffset, PrimitiveDateTime, OffsetDateTime];
}

/// Parse any parsable type from the time crate.
///
/// This is identical to calling `T::parse(s, format)`, but allows the use of
/// type inference where possible.
///
/// ```rust,no_run
/// use time::Time;
///
/// #[derive(Debug)]
/// struct Foo(Time);
///
/// fn main() -> time::Result<()> {
///     // We don't need to tell the compiler what type we need!
///     let foo = Foo(time::parse("14:55:02", "%T")?);
///     println!("{:?}", foo);
///     Ok(())
/// }
/// ```
#[inline(always)]
pub fn parse<'a, T: private::Parsable>(
    s: impl Into<Cow<'a, str>>,
    format: impl Into<Cow<'a, str>>,
) -> ParseResult<T> {
    private::Parsable::parse(s, format)
}
