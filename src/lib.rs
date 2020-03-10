//! Simple time handling.
//!
//! ![rustc 1.34.0](https://img.shields.io/badge/rustc-1.34.0-blue)
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
//! ## `deprecated`
//!
//! Using the `deprecated` feature allows using deprecated v0.1 methods. Enabled
//! by default.
//!
//! With the standard library, the normal `time = 0.2` will work as expected.
//!
//! With `#![no_std]` support:
//! ```toml
//! [dependencies]
//! time = { version = "0.2", default-features = false, features = ["deprecated"] }
//! ```
//!
//! ## `panicking-api`
//!
//! Non-panicking APIs are provided, and should generally be preferred. However,
//! there are some situations where avoiding `.unwrap()` may be desired. To
//! enable these APIs, you need to use the `panicking-api` feature in your
//! `Cargo.toml`, which is not enabled by default.
//!
//! Library authors should avoid using this feature.
//!
//! This feature will be removed in a future release, as there are provided
//! macros to perform the equivalent calculations at compile-time.
//!
//! ```toml
//! [dependencies]
//! time = { version = "0.2", features = ["panicking-api"] }
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
#![cfg_attr(no_std, no_std)]
#![deny(
    unsafe_code, // Used when interacting with system APIs
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
    unstable_name_collisions,
    clippy::suspicious_arithmetic_impl,
    clippy::inline_always,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::use_self, // Not supported in some situations in older compilers.
)]
#![cfg_attr(test, allow(clippy::cognitive_complexity, clippy::too_many_lines))]
#![doc(html_favicon_url = "https://avatars0.githubusercontent.com/u/55999857")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/55999857")]
// Because we have a macro named `time`, this can cause conflicts. MSRV
// guarantees that edition 2018 is available.
#![doc(test(no_crate_inject))]

// This is necessary to allow our proc macros to work.
// See rust-lang/rust#54647 for details.
// Unfortunately, this also means we can't have a `time` mod.
extern crate self as time;

#[cfg(docs)]
#[rustversion::not(nightly)]
compile_error!("The `__doc` feature requires a nightly compiler, and is for internal usage only.");

#[rustversion::before(1.34.0)]
compile_error!("The time crate has a minimum supported rust version of 1.34.0.");

#[cfg(no_std)]
#[rustversion::before(1.36.0)]
compile_error!(
    "Using the time crate without the standard library enabled requires a global allocator. This \
     was stabilized in Rust 1.36.0. You can either upgrade or enable the standard library."
);

#[cfg(panicking_api)]
#[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
macro_rules! format_conditional {
    ($conditional:ident) => {
        format!(concat!(stringify!($conditional), "={}"), $conditional)
    };

    ($first_conditional:ident, $($conditional:ident),*) => {{
        #[cfg(no_std)]
        let mut s = alloc::string::String::new();
        #[cfg(std)]
        let mut s = String::new();
        s.push_str(&format_conditional!($first_conditional));
        $(s.push_str(&format!(concat!(", ", stringify!($conditional), "={}"), $conditional));)*
        s
    }}
}

/// Panic if the value is not in range.
#[cfg(panicking_api)]
#[cfg_attr(docs, doc(cfg(feature = "panicking-api")))]
macro_rules! assert_value_in_range {
    ($value:ident in $start:expr => $end:expr) => {
        #[allow(unused_comparisons)]
        {
            if $value < $start || $value > $end {
                panic!(
                    concat!(stringify!($value), " must be in the range {}..={} (was {})"),
                    $start,
                    $end,
                    $value,
                );
            }
        }
    };

    ($value:ident in $start:expr => $end:expr, given $($conditional:ident),+ $(,)?) => {
        #[allow(unused_comparisons)]
        {
            if $value < $start || $value > $end {
                panic!(
                    concat!(stringify!($value), " must be in the range {}..={} given{} (was {})"),
                    $start,
                    $end,
                    &format_conditional!($($conditional),+),
                    $value,
                );
            };
        }
    };
}

// TODO Some of the formatting can likely be performed at compile-time.
/// Returns `None` if the value is not in range.
macro_rules! ensure_value_in_range {
    ($value:ident in $start:expr => $end:expr) => {
        #[allow(unused_comparisons)]
        {
            if $value < $start || $value > $end {
                return Err(ComponentRangeError {
                    name: stringify!($value),
                    minimum: i64::from($start),
                    maximum: i64::from($end),
                    value: i64::from($value),
                    given: Vec::new(),
                });
            }
        }
    };

    ($value:ident in $start:expr => $end:expr, given $($conditional:ident),+ $(,)?) => {
        #[allow(unused_comparisons)]
        {
            if $value < $start || $value > $end {
                return Err(ComponentRangeError {
                    name: stringify!($value),
                    minimum: i64::from($start),
                    maximum: i64::from($end),
                    value: i64::from($value),
                    given: vec![$((stringify!($conditional), i64::from($conditional))),+],
                });
            };
        }
    };
}

#[cfg(all(test, std))]
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
/// The `Sign` struct and its associated `impl`s.
mod sign;
/// The `Time` struct and its associated `impl`s.
mod time_mod;
/// The `UtcOffset` struct and its associated `impl`s.
mod utc_offset;
/// Days of the week.
mod weekday;

pub use date::{days_in_year, is_leap_year, weeks_in_year, Date};
pub use duration::Duration;
pub use error::{ComponentRangeError, ConversionRangeError, Error, IndeterminateOffsetError};
pub(crate) use format::DeferredFormat;
use format::ParseResult;
pub use format::{validate_format_string, ParseError};
#[cfg(std)]
pub use instant::Instant;
pub use numerical_traits::{NumericalDuration, NumericalStdDuration, NumericalStdDurationShort};
pub use offset_date_time::OffsetDateTime;
pub use primitive_date_time::PrimitiveDateTime;
#[allow(deprecated)]
pub use sign::Sign;
/// Construct a [`Date`] with a statically known value.
///
/// The resulting expression can be used in `const` or `static` declarations.
///
/// Three formats are supported: year-week-weekday, year-ordinal, and
/// year-month-day.
///
/// ```rust
/// # use time::{Date, date, Weekday::*};
/// # fn main() -> time::Result<()> {
/// assert_eq!(date!(2020-W01-3), Date::try_from_iso_ywd(2020, 1, Wednesday)?);
/// assert_eq!(date!(2020-001), Date::try_from_yo(2020, 1)?);
/// assert_eq!(date!(2020-01-01), Date::try_from_ymd(2020, 1, 1)?);
/// # Ok(())
/// # }
/// ```
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
/// assert_eq!(offset!(UTC), UtcOffset::hours(0));
/// assert_eq!(offset!(utc), UtcOffset::hours(0));
/// assert_eq!(offset!(+0), UtcOffset::hours(0));
/// assert_eq!(offset!(+1), UtcOffset::hours(1));
/// assert_eq!(offset!(-1), UtcOffset::hours(-1));
/// assert_eq!(offset!(+1:30), UtcOffset::minutes(90));
/// assert_eq!(offset!(-1:30), UtcOffset::minutes(-90));
/// assert_eq!(offset!(+1:30:59), UtcOffset::seconds(5459));
/// assert_eq!(offset!(-1:30:59), UtcOffset::seconds(-5459));
/// assert_eq!(offset!(+23:59:59), UtcOffset::seconds(86_399));
/// assert_eq!(offset!(-23:59:59), UtcOffset::seconds(-86_399));
/// ```
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
/// # fn main() -> time::Result<()> {
/// assert_eq!(time!(0:00), Time::try_from_hms(0, 0, 0)?);
/// assert_eq!(time!(1:02:03), Time::try_from_hms(1, 2, 3)?);
/// assert_eq!(time!(1:02:03.004_005_006), Time::try_from_hms_nano(1, 2, 3, 4_005_006)?);
/// assert_eq!(time!(12:00 am), Time::try_from_hms(0, 0, 0)?);
/// assert_eq!(time!(1:02:03 am), Time::try_from_hms(1, 2, 3)?);
/// assert_eq!(time!(1:02:03.004_005_006 am), Time::try_from_hms_nano(1, 2, 3, 4_005_006)?);
/// assert_eq!(time!(12:00 pm), Time::try_from_hms(12, 0, 0)?);
/// assert_eq!(time!(1:02:03 pm), Time::try_from_hms(13, 2, 3)?);
/// assert_eq!(time!(1:02:03.004_005_006 pm), Time::try_from_hms_nano(13, 2, 3, 4_005_006)?);
/// # Ok(())
/// # }
/// ```
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
/// use time::prelude::*;
/// ```
///
/// The prelude may grow in minor releases. Any removals will only occur in
/// major releases.
pub mod prelude {
    // Rename traits to `_` to avoid any potential name conflicts.
    pub use crate::{NumericalDuration as _, NumericalStdDuration as _};
    // We need to re-export from the macros crate again (and not just do
    // `crate::foo`) because of the way name resolution works in Rust. It's not
    // currently possible to import _only_ the macro, so doing `use crate::time`
    // also pulls in the `time` _crate_ (due to `extern crate self as time`).
    //
    // As a side note, doing `use crate::time` causes a stack overflow in
    // rustc <= 1.37.0.
    pub use time_macros::{date, offset, time};
}

/// Items generally useful in any file in the time crate.
mod internal_prelude {
    #![allow(unused_imports)]

    #[cfg(no_std)]
    extern crate alloc;

    #[cfg(std)]
    pub(crate) use crate::Instant;
    pub(crate) use crate::{
        format::{ParseError, ParseResult},
        ComponentRangeError, ConversionRangeError, Date, DeferredFormat, Duration,
        IndeterminateOffsetError, NumericalDuration, NumericalStdDuration, OffsetDateTime,
        PrimitiveDateTime, Time, UtcOffset,
        Weekday::{self, Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday},
    };
    #[cfg(no_std)]
    pub(crate) use alloc::{
        borrow::ToOwned,
        boxed::Box,
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
    pub(crate) use standback::prelude::*;
    pub(crate) use time_macros::{date, offset, time};
}

#[allow(clippy::missing_docs_in_private_items)]
mod private {
    use super::*;

    macro_rules! parsable {
        ($($type:ty),* $(,)?) => {
            $(
                impl Parsable for $type {
                    fn parse(s: impl AsRef<str>, format: impl AsRef<str>) -> ParseResult<Self> {
                        Self::parse(s, format)
                    }
                }
            )*
        };
    }

    pub trait Parsable: Sized {
        fn parse(s: impl AsRef<str>, format: impl AsRef<str>) -> ParseResult<Self>;
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
pub fn parse<T: private::Parsable>(s: impl AsRef<str>, format: impl AsRef<str>) -> ParseResult<T> {
    private::Parsable::parse(s, format)
}

// For some back-compatibility, we're also implementing some deprecated types
// and methods. They will be removed completely in 0.3.

#[cfg(all(std, v01_deprecated))]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(since = "0.2.0", note = "Use `Instant`")]
pub type PreciseTime = Instant;

#[cfg(all(std, v01_deprecated))]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(since = "0.2.0", note = "Use `Instant`")]
pub type SteadyTime = Instant;

#[cfg(all(std, v01_deprecated))]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(
    since = "0.2.0",
    note = "Use `OffsetDateTime::now() - OffsetDateTime::unix_epoch()` to get a `Duration` since \
            a known epoch."
)]
#[inline]
pub fn precise_time_ns() -> u64 {
    use core::convert::TryInto;
    use std::time::SystemTime;

    (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH))
        .expect("System clock was before 1970.")
        .as_nanos()
        .try_into()
        .expect("This function will be removed long before this is an issue.")
}

#[cfg(all(std, v01_deprecated))]
#[cfg_attr(tarpaulin, skip)]
#[allow(clippy::missing_docs_in_private_items)]
#[deprecated(
    since = "0.2.0",
    note = "Use `OffsetDateTime::now() - OffsetDateTime::unix_epoch()` to get a `Duration` since \
            a known epoch."
)]
#[inline]
pub fn precise_time_s() -> f64 {
    use std::time::SystemTime;

    (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH))
        .expect("System clock was before 1970.")
        .as_secs_f64()
}
