# Changelog

All notable changes to the time project will be documented in this file.

The format is based on [Keep a Changelog]. This project adheres to [Semantic
Versioning].

---

## Unreleased

None.

## 0.2.9 [2020-03-13]

### Fixed

`cfg-if` now has a mandatory minimum of 0.1.10, rather than just 0.1. This is
because compilation fails when using 0.1.9.

## 0.2.8 [2020-03-12]

### Added

- `cargo_web` support has been added for getting a local offset. A general
  catch-all defaulting to UTC has also been added.
- `Error::source` has been implemented for the wrapper `time::Error`.
- `UtcOffset::try_local_offset`, `UtcOffset::try_current_local_offset`,
  `OffsetDateTime::try_now_local()` provide fallible alternatives when the
  default of UTC is not desired. To facilitate this change,
  `IndeterminateOffsetError` has been added.
- Support for parsing and formatting subsecond nanoseconds.

### Changed

- `#[non_exhaustive]` is simulated on compilers prior to 1.40.0.

## 0.2.7 [2020-02-22]

### Added

- `Display` has been implemented for `Date`, `OffsetDateTime`,
  `PrimitiveDateTime`, `Time`, `UtcOffset`, and `Weekday`.
- `Hash` is now derived for `Duration`.
- `SystemTime` can be converted to and from `OffsetDateTime`. The following
  trait implementations have been made for interoperability:
  - `impl Sub<SystemTime> for OffsetDateTime`
  - `impl Sub<OffsetDateTime> for SystemTime`
  - `impl PartialEq<SystemTime> for OffsetDateTime`
  - `impl PartialEq<OffsetDateTime> for SystemTime`
  - `impl PartialOrd<SystemTime> for OffsetDateTime`
  - `impl PartialOrd<OffsetDateTime> for SystemTime`
  - `impl From<SystemTime> for OffsetDateTime`
  - `impl From<OffsetDateTime> for SystemTime`
- All structs now `impl Duration<T> for Standard`, allowing usage with the
  `rand` crate. This is gated behind the `rand` feature flag.

- Documentation can now be built on stable. Some annotations will be missing if
  you do this.
- `NumericalDuration` has been implemented for `f32` and `f64`.
  `NumericalStdDuration` and `NumericalStdDurationShort` have been implemented
  for `f64` only.
- `UtcOffset::local_offset_at(OffsetDateTime)`, which will obtain the system's
  local offset at the provided moment in time.
  - `OffsetDateTime::now_local()` is equivalent to calling
    `OffsetDateTime::now().to_offset(UtcOffset::local_offset_at(OffsetDateTime::now()))`
    (but more efficient).
  - `UtcOffset::current_local_offset()` will return the equivalent of
    `OffsetDateTime::now_local().offset()`.

### Changed

- All formatting and parsing methods now accept `impl AsRef<str>` as parameters,
  rather than just `&str`. `time::validate_format_string` does this as well.
- The requirement of a `Date` being between the years -100,000 and +100,000
  (inclusive) is now strictly enforced.
- Overflow checks for `Duration` are now enabled by default. This behavior is
  the identical to what the standard library does.
- The `time`, `date`, and `offset` macros have been added to the prelude.

### Deprecated

- `Sign` has been deprecated in its entirety, along with `Duration::sign`.

  To obtain the sign of a `Duration`, you can use the `Sign::is_positive`,
  `Sign::is_negative`, and `Sign::is_zero` methods.

- A number of functions and trait implementations that implicitly assumed a
  timezone (generally UTC) have been deprecated. These are:
  - `Date::today`
  - `Time::now`
  - `PrimitiveDateTime::now`
  - `PrimitiveDateTime::unix_epoch`
  - `PrimitiveDateTime::from_unix_timestamp`
  - `PrimitiveDateTime::timestamp`
  - `impl Sub<SystemTime> for PrimitiveDateTime`
  - `impl Sub<PrimitiveDateTime> for SystemTime`
  - `impl PartialEq<SystemTime> for PrimitiveDateTime`
  - `impl PartialEq<PrimitiveDateTime> for SystemTime>`
  - `impl PartialOrd<SystemTime> for PrimitiveDateTime`
  - `impl PartialOrd<PrimitiveDateTime> for SystemTime>`
  - `impl From<SystemTime> for PrimitiveDateTime`
  - `impl From<PrimitiveDateTime> for SystemTime`

### Fixed

- Avoid panics when parsing an empty string (#215).
- The nanoseconds component of a `Duration` is now always in range. Previously,
  it was possible (via addition and/or subtraction) to obtain a value that was
  not internally consistent.
- `Time::parse` erroneously returned an `InvalidMinute` error when it was
  actually the second that was invalid.
- `Date::parse("0000-01-01", "%Y-%m-%d")` incorrectly returned an `Err` (#221).

## Pre-0.2.7

Prior to v0.2.7, changes were listed in GitHub releases.

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
