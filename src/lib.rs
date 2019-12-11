//! Simple time handling.
//!
//! ![rustc 1.40.0](https://img.shields.io/badge/rustc-1.40.0-blue)
//!
//! # Feature flags in Cargo
//!
//! ## `std`
//!
//! Currently, all structs except `Instant` can be used with `#![no_std]`. As
//! support for the standard library is enabled by default, you muse use
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
//! ## `deprecated`
//!
//! Using the `deprecated` feature allows using deprecated methods. Enabled by
//! default.
//!
//! Currently, the only benefit of disabling this feature is that the log crate
//! is a dependency if it is enabled.
//!
//! With the standard library, the normal `time = 0.2` will work as expected.
//!
//! With `#![no_std]` support:
//! ```toml
//! [dependencies]
//! time = { version = "0.2", default-features = false, features = ["deprecated"] }
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
//! | `%p`      | `am` or `pm` designation                                               | `pm`                       |
//! | `%P`      | `AM` or `PM` designation                                               | `PM`                       |
//! | `%r`      | 12-hour clock time, equivalent to `%-I:%M:%S %p`                       | `2:55:02 pm`               |
//! | `%R`      | 24-hour HH:MM time, equivalent to `%-H:%M`                             | `14:55`                    |
//! | `%S`      | Second (`00`-`59`)                                                     | `02`                       |
//! | `%T`      | ISO 8601 time format (HH:MM:SS), equivalent to `%-H:%M:%S`             | `14:55:02`                 |
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

#![cfg_attr(doc, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(
    anonymous_parameters,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub, // some known bugs that are overridden
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    clippy::all
)]
#![warn(
    unused_extern_crates,
    box_pointers,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_qualifications,
    variant_size_differences,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::option_unwrap_used,
    clippy::print_stdout,
    clippy::result_unwrap_used
)]
#![allow(
    clippy::suspicious_arithmetic_impl,
    clippy::inline_always,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::module_name_repetitions,
    clippy::must_use_candidate // rust-lang/rust-clippy#4779
)]
#![cfg_attr(test, allow(clippy::cognitive_complexity, clippy::too_many_lines))]
#![doc(html_favicon_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/55999857")]

#[macro_use]
extern crate alloc;

macro_rules! format_conditional {
    ($conditional:ident) => {
        format!(concat!(stringify!($conditional), "={}"), $conditional)
    };

    ($first_conditional:ident, $($conditional:ident),*) => {{
        let mut s = alloc::string::String::new();
        s.push_str(&format_conditional!($first_conditional));
        $(s.push_str(&format!(concat!(", ", stringify!($conditional), "={}"), $conditional));)*
        s
    }}
}

/// Panic if the value is not in range.
macro_rules! assert_value_in_range {
    ($value:ident in $start:expr => $end:expr) => {
        if !($start..=$end).contains(&$value) {
            panic!(
                concat!(stringify!($value), " must be in the range {}..={} (was {})"),
                $start,
                $end,
                $value,
            );
        }
    };

    ($value:ident in $start:expr => exclusive $end:expr) => {
        if !($start..$end).contains(&$value) {
            panic!(
                concat!(stringify!($value), " must be in the range {}..{} (was {})"),
                $start,
                $end,
                $value,
            );
        }
    };

    ($value:ident in $start:expr => $end:expr, given $($conditional:ident),+ $(,)?) => {
        if !($start..=$end).contains(&$value) {
            panic!(
                concat!(stringify!($value), " must be in the range {}..={} given{} (was {})"),
                $start,
                $end,
                &format_conditional!($($conditional),+),
                $value,
            );
        };
    };
}

/// Returns `None` if the value is not in range.
macro_rules! ensure_value_in_range {
    ($value:ident in $start:expr => $end:expr) => {
        if !($start..=$end).contains(&$value) {
            return None;
        }
    };

    ($value:ident in $start:expr => exclusive $end:expr) => {
        if !($start..$end).contains(&$value) {
            return None;
        }
    };

    ($value:ident in $start:expr => $end:expr,given $($conditional:ident),+ $(,)?) => {
        if !($start..=$end).contains(&$value) {
            return None;
        };
    };
}

#[cfg(all(test, feature = "std"))]
macro_rules! assert_panics {
    ($e:expr $(, $message:literal)?) => {
        #[allow(box_pointers)]
        {
            if std::panic::catch_unwind(move || $e).is_ok() {
                panic!(concat!(
                    "assertion failed: expected `",
                    stringify!($e),
                    "` to panic",
                    $(concat!(" (", $message, ")"))?
                ));
            }
        }
    };
}

/// The `Date` struct and its associated `impl`s.
mod date;
/// The `Duration` struct and its associated `impl`s.
mod duration;
mod format;
/// The `Instant` struct and its associated `impl`s.
#[cfg(feature = "std")]
mod instant;
/// A collection of traits extending built-in numerical types.
mod numerical_traits;
/// The `OffsetDateTime` struct and its associated `impl`s.
mod offset_date_time;
/// The `PrimitiveDateTime` struct and its associated `impl`s.
mod primitive_date_time;
/// Ensure certain methods are present on all types.
mod shim;
/// The `Sign` struct and its associated `impl`s.
mod sign;
/// The `Time` struct and its associated `impl`s.
mod time;
/// The `UtcOffset` struct and its associated `impl`s.
mod utc_offset;
/// Days of the week.
mod weekday;

pub use self::time::Time;
use core::fmt;
pub use date::{days_in_year, is_leap_year, weeks_in_year, Date};
pub use duration::Duration;
pub(crate) use format::DeferredFormat;
#[allow(unreachable_pub)] // rust-lang/rust#64762
pub use format::ParseError;
#[cfg(feature = "std")]
pub use instant::Instant;
pub use numerical_traits::{NumericalDuration, NumericalStdDuration, NumericalStdDurationShort};
pub use offset_date_time::OffsetDateTime;
pub use primitive_date_time::PrimitiveDateTime;
pub(crate) use shim::NumberExt;
pub use sign::Sign;
pub use utc_offset::UtcOffset;
pub use weekday::Weekday;

/// A collection of traits that are widely useful. Unlike the standard library,
/// this must be explicitly imported:
///
/// ```rust,no_run
/// use time::prelude::*;
/// ```
///
/// The prelude may grow in minor releases. Any removals will only occur in
/// major releases.
pub mod prelude {
    // Rename to `_` to avoid any potential name conflicts.
    pub use crate::{NumericalDuration as _, NumericalStdDuration as _};
}

/// A stable alternative to [`alloc::v1::prelude`](https://doc.rust-lang.org/stable/alloc/prelude/v1/index.html).
/// Useful anywhere `#![no_std]` is allowed.
#[cfg(not(feature = "std"))]
mod no_std_prelude {
    #![allow(unused_imports)]
    pub(crate) use alloc::{
        borrow::ToOwned,
        boxed::Box,
        string::{String, ToString},
        vec::Vec,
    };
}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
///
/// ```rust
/// # use time::{Duration, OutOfRangeError};
/// # use core::time::Duration as StdDuration;
/// # use core::{any::Any, convert::TryFrom};
/// // "Construct" an `OutOfRangeError`.
/// let error = StdDuration::try_from(Duration::seconds(-1)).unwrap_err();
/// assert!(Any::is::<OutOfRangeError>(&error));
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OutOfRangeError {}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(not(feature = "std"))]
    use crate::no_std_prelude::*;

    #[test]
    fn out_of_range_error_format() {
        assert_eq!(
            OutOfRangeError.to_string(),
            "Source value is out of range for the target type",
        );
    }
}

// For some back-compatibility, we're also implementing some deprecated types
// and methods. They will be removed completely in 0.3.

#[cfg(all(feature = "std", feature = "deprecated"))]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(since = "0.2.0", note = "Use `Instant`")]
pub type PreciseTime = Instant;

#[cfg(all(feature = "std", feature = "deprecated"))]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(since = "0.2.0", note = "Use `Instant`")]
pub type SteadyTime = Instant;

#[cfg(all(feature = "std", feature = "deprecated"))]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(
    since = "0.2.0",
    note = "Use `PrimitiveDateTime::now() - PrimitiveDateTime::unix_epoch()` to get a `Duration` \
            since a known epoch."
)]
#[inline]
pub fn precise_time_ns() -> u64 {
    use core::convert::TryInto;
    (PrimitiveDateTime::now() - PrimitiveDateTime::unix_epoch())
        .whole_nanoseconds()
        .try_into()
        .expect("You really shouldn't be using this in the year 2554...")
}

#[cfg(all(feature = "std", feature = "deprecated"))]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(
    since = "0.2.0",
    note = "Use `PrimitiveDateTime::now() - PrimitiveDateTime::unix_epoch()` to get a `Duration` \
            since a known epoch."
)]
#[inline]
pub fn precise_time_s() -> f64 {
    (PrimitiveDateTime::now() - PrimitiveDateTime::unix_epoch()).as_seconds_f64()
}
