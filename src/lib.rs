//! Simple time handling.
//!
//! ![MSRV 1.38.0](https://img.shields.io/badge/MSRV-1.38.0-red)
//!
//! # Feature gates
//!
//! ## `#![no_std]`
//!
//! Currently, all structs except `Instant` are useable wiht `#![no_std]`. As
//! support for the standard library is enabled by default, you muse use
//! `default_features = false` in your `Cargo.toml` to enable this.
//!
//! ```none
//! [dependencies]
//! time = { version = "0.2", default-features = false }
//! ```
//!
//! Of the structs that are useable, some methods may only be enabled due a
//! reliance on `Instant`. These will be documented alongside the method.
//!
//! ## Serde
//!
//! [Serde](https://github.com/serde-rs/serde) support is behind a feature flag.
//! To enable it, use the `serialization` feature. This is not enabled by
//! default. It _is_ compatible with `#![no_std]`, so long as an allocator is
//! present.
//!
//! ```none
//! [dependencies]
//! time = { version = "0.2", features = ["serialization"] }
//! ```
//!
//! # Formatting
//!
//! Time's implementation of formatting is based on `strftime` in C, though it
//! is explicitly no compatible. Specifiers may be missing, added, or have
//! different behavior than in C. As such, you should use the table below, which
//! is an up-to-date reference on what each specifier does.
//!
//! | Specifier | Replaced by                                                            | Example                  |
//! |-----------|------------------------------------------------------------------------|--------------------------|
//! | `%a`      | Abbreviated weekday name                                               | Thu                      |
//! | `%A`      | Full weekday name                                                      | Thursday                 |
//! | `%b`      | Abbreviated month name                                                 | Aug                      |
//! | `%B`      | Full month name                                                        | August                   |
//! | `%c`      | Date and time representation                                           | Thu Aug 23 14:55:02 2001 |
//! | `%C`      | Year divided by 100 and truncated to integer (00-99)                   | 20                       |
//! | `%d`      | Day of the month, zero-padded (01-31)                                  | 23                       |
//! | `%D`      | Short MM/DD/YY date, equivalent to %m/%d/%y                            | 08/23/01                 |
//! | `%e`      | Day of the month, space-padded ( 1-31)                                 | 23                       |
//! | `%F`      | Short YYYY-MM-DD date, equivalent to %Y-%m-%d                          | 2001-08-23               |
//! | `%g`      | Week-based year, last two digits (00-99)                               | 01                       |
//! | `%G`      | Week-based year                                                        | 2001                     |
//! | `%H`      | Hour in 24h format (00-23)                                             | 14                       |
//! | `%I`      | Hour in 12h format (01-12)                                             | 02                       |
//! | `%j`      | Day of the year (001-366)                                              | 235                      |
//! | `%m`      | Month as a decimal number (01-12)                                      | 08                       |
//! | `%M`      | Minute (00-59)                                                         | 55                       |
//! | `%p`      | `am` or `pm` designation                                               | pm                       |
//! | `%P`      | `AM` or `PM` designation                                               | PM                       |
//! | `%r`      | 12-hour clock time                                                     | 02:55:02 pm              |
//! | `%R`      | 24-hour HH:MM time, equivalent to %H:%M                                | 14:55                    |
//! | `%S`      | Second (00-59)                                                         | 02                       |
//! | `%T`      | ISO 8601 time format (HH:MM:SS), equivalent to %H:%M:%S                | 14:55:02                 |
//! | `%u`      | ISO 8601 weekday as number with Monday as 1 (1-7)                      | 4                        |
//! | `%U`      | Week number with the first Sunday as the first day of week one (00-53) | 33                       |
//! | `%V`      | ISO 8601 week number (01-53)                                           | 34                       |
//! | `%w`      | Weekday as a decimal number with Sunday as 0 (0-6)                     | 4                        |
//! | `%W`      | Week number with the first Monday as the first day of week one (00-53) | 34                       |
//! | `%y`      | Year, last two digits (00-99)                                          | 01                       |
//! | `%Y`      | Year                                                                   | 2001                     |
//! | `%z`      | ISO 8601 offset from UTC in timezone (+HHMM)                           | +0100                    |
//! | `%%`      | Literal `%`                                                            | %                        |
//!
//! ## Modifiers
//!
//! All specifiers that are strictly numerical have modifiers for formatting.
//! Adding a modifier to a non-supporting specifier is a no-op.
//!
//! | Modifier         | Behavior        | Example                       |
//! |------------------|-----------------|-------------------------------|
//! | `-` (dash)       | No padding      | `%-d` => `5` instead of `05`  |
//! | `_` (underscore) | Pad with spaces | `%_d` => ` 5` instead of `05` |
//! | `0`              | Pad with zeros  | `%0e` => `05` instead of ` 5` |

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    anonymous_parameters,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
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
    clippy::missing_docs_in_private_items
)]
#![allow(
    clippy::suspicious_arithmetic_impl,
    clippy::inline_always,
    // TODO Change to `warn` once rust-lang/rust-clippy#4605 is resolved.
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::non_ascii_literal,
    clippy::module_inception
)]

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

/// The `Date` struct and its associated `impl`s.
mod date;
/// The `DateTime` struct and its associated `impl`s.
mod date_time;
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
pub use date_time::DateTime;
pub use duration::Duration;
pub use format::{language::Language, DeferredFormat};
#[cfg(feature = "std")]
pub use instant::Instant;
pub use numerical_traits::{NumericalDuration, NumericalStdDuration};
pub use offset_date_time::OffsetDateTime;
pub(crate) use shim::NumberExt;
pub use sign::Sign;
pub use utc_offset::UtcOffset;
pub use weekday::Weekday;

/// A collection of traits (and possibly types, enums, etc.) that are useful to
/// import. Unlike the standard library, this must be explicitly included.
///
/// ```rust,no_run
/// use time::prelude::*;
/// ```
///
/// The prelude may grow in minor releases. Any removals will only occur in
/// major releases.
pub mod prelude {
    pub use crate::{NumericalDuration, NumericalStdDuration};
}

/// A stable alternative to [`alloc::v1::prelude`](https://doc.rust-lang.org/stable/alloc/prelude/v1/index.html).
/// Should be used anywhere `#![no_std]` is allowed.
#[cfg(not(feature = "std"))]
mod no_std_prelude {
    #![allow(unused_imports)]
    pub(crate) use alloc::borrow::ToOwned;
    pub(crate) use alloc::boxed::Box;
    pub(crate) use alloc::string::{String, ToString};
    pub(crate) use alloc::vec::Vec;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRangeError {
    /// Include zero-sized field so users can't construct this explicitly. This
    /// ensures forwards-compatibility, as anyone matching on this type has to
    /// explicitly discard the fields.
    unused: (),
}

impl OutOfRangeError {
    /// Create an new `OutOfRangeError`.
    pub(crate) const fn new() -> Self {
        Self { unused: () }
    }
}

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OutOfRangeError {}

// For some back-compatibility, we're also implementing some deprecated types.

#[cfg(feature = "std")]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(since = "0.2.0", note = "Use `Instant`")]
pub type PreciseTime = Instant;

#[cfg(feature = "std")]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(since = "0.2.0", note = "Use `Instant`")]
pub type SteadyTime = Instant;
