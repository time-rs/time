# Changelog

All notable changes to the time project will be documented in this file.

The format is based on [Keep a Changelog]. This project adheres to [Semantic
Versioning].

---

## Unreleased

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

- Documentation can now be built on stable. Some annotations will be missing if
  you do this.
- `NumericalDuration` has been implemented for `f32` and `f64`.
  `NumericalStdDuration` and `NumericalStdDurationShort` have been implemented
  for `f64` only.
- `UtcOffset::local_offset_at(OffsetDateTime)`, which will obtain the system's
  local offset at the provided moment in time.
  - `OffsetDateTime::now()` and `impl From<SystemTime> for OffsetDateTime` both
    return a value with the local offset.
  - `OffsetDateTime::now_utc()` is equivalent to calling
    `OffsetDateTime::now().to_offset(offset!(UTC))`.
  - `UtcOffset::current_local_offset()` will return the equivalent of
    `OffsetDateTime::now().offset()`.

### Changed

- All formatting and parsing methods now accept `impl AsRef<str>` as parameters,
  rather than just `&str`. `time::validate_format_string` does this as well.

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

## Pre-0.2.7

Prior to v0.2.7, changes were listed in GitHub releases.

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
