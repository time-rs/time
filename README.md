# time

![build status](https://github.com/time-rs/time/workflows/Build/badge.svg)
![downloads](https://img.shields.io/crates/d/time)
![license](https://img.shields.io/badge/license-MIT%20or%20Apache--2-blue)
![version](https://img.shields.io/crates/v/time)
![MSRV 1.38.0](https://img.shields.io/badge/MSRV-1.38.0-red)

## Features

The following structs are currently useable with `#![no_std]`.

- `Duration`
- `Weekday`
- `Sign`
- `Date`
- `Time`
- `DateTime`

To enable this, you must use `default_features = false` in your `Cargo.toml`.

`Instant` is not useable with `#![no_std]`. This will not happen unless
`std::time::Instant` moves to `core`.

Of the structs that _are_ useable, some method may only be enabled due a
reliance on `Instant`. These will be documented alongside the method.
