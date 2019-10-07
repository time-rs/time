# time

![build status](https://github.com/time-rs/time/workflows/Build/badge.svg)
![downloads](https://img.shields.io/crates/d/time)
![license](https://img.shields.io/badge/license-MIT%20or%20Apache--2-blue)
![version](https://img.shields.io/crates/v/time)
![MSRV 1.38.0](https://img.shields.io/badge/MSRV-1.38.0-red)

## Features

### `#![no_std]`

Currently, all structs except `Instant` are useable wiht `#![no_std]`. As
support for the standard library is enabled be default, you muse use
`default_features = false` in your `Cargo.toml` to enable this.

```none
[dependencies]
time = { version = "0.2", default-features = false }
```

Of the structs that are useable, some methods may only be enabled due a reliance
on `Instant`. These will be documented alongside the method.

### Serde

[Serde](https://github.com/serde-rs/serde) support is behind a feature flag. To
enable it, use the `serialization` feature. This is not enabled by default. It
_is_ compatible with `#![no_std]`, so long as an allocator is present.

```none
[dependencies]
time = { version = "0.2", features = ["serialization"] }
```
