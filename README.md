# time

![build status](https://github.com/time-rs/time/workflows/Build/badge.svg)
[![documentation status](https://github.com/time-rs/time/workflows/Documentation/badge.svg)](https://time-rs.github.io/time/time/index.html)
<br>
![downloads](https://img.shields.io/crates/d/time)
![license](https://img.shields.io/badge/license-MIT%20or%20Apache--2-blue)
![version](https://img.shields.io/crates/v/time)
![MSRV 1.38.0](https://img.shields.io/badge/MSRV-1.38.0-red)

## Features

### `#![no_std]`

Currently, all structs except `Instant` can be used with `#![no_std]`. As
support for the standard library is enabled by default, you muse use
`default_features = false` in your `Cargo.toml` to enable this.

```toml
[dependencies]
time = { version = "0.2", default-features = false }
```

Of the structs that are usable, some methods may only be enabled due a reliance
on `Instant`. These will be documented alongside the method.

### Serde

[Serde](https://github.com/serde-rs/serde) support is behind a feature flag. To
enable it, use the `serialization` feature. This is not enabled by default. It
_is_ compatible with `#![no_std]`, so long as an allocator is present.

```toml
[dependencies]
time = { version = "0.2", features = ["serialization"] }
```

### License

This project is licensed under either of

- [Apache License, Version 2.0](https://github.com/time-rs/time/blob/master/LICENSE-Apache)
- [MIT license](https://github.com/time-rs/time/blob/master/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in time by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
