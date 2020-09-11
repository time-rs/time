use std::env;
use version_check as rustc;

const MSRV: &str = "1.32.0";
const NO_STD_MSRV: &str = "1.36.0";

macro_rules! cfg_emit {
    ($s:ident) => {
        println!(concat!("cargo:rustc-cfg=", stringify!($s)));
    };
}

macro_rules! warning {
    ($($s:tt)*) => {
        println!("cargo:warning={}", format_args!($($s)*));
    };
}

fn main() {
    println!("cargo:rerun-if-env-changed=COMPILING_UNDER_CARGO_WEB");

    // Are we compiling with `cargo web`?
    if env::var("COMPILING_UNDER_CARGO_WEB") == Ok("1".into()) {
        cfg_emit!(__time_02_cargo_web);
    }

    // Warn if the version is below MSRV.
    if !rustc::is_min_version(MSRV).unwrap_or(false) {
        warning!(
            "The time crate has a minimum supported rust version of {}.",
            MSRV
        );
    }

    // Warn if the version is below `#![no_std]` MSRV.
    if !rustc::is_min_version(NO_STD_MSRV).unwrap_or(false) {
        #[cfg(not(feature = "std"))]
        warning!(
            "Using the time crate without the standard library enabled requires a global \
             allocator. This was stabilized in Rust {}. You can either upgrade or enable the \
             standard library.",
            NO_STD_MSRV
        );
    }

    // Warn if the `__doc` feature is used on stable or beta.
    if !rustc::Channel::read().map_or(false, |channel| channel.supports_features()) {
        #[cfg(__time_02_docs)]
        warning!(
            "`--cfg __time_02_docs` requires a nightly compiler, and is intended for internal \
             usage only."
        );
    }

    // ==== features that affect runtime directly ====

    // `#[non_exhaustive]` was stabilized in 1.40.0.
    if rustc::is_min_version("1.40.0").unwrap_or(false) {
        cfg_emit!(__time_02_supports_non_exhaustive);
    }

    // `Instant::checked_add` and `Instant::checked_sub` were added in 1.34.0.
    // `NonZeroI*` was stabilized in 1.34.0.
    if rustc::is_min_version("1.34.0").unwrap_or(false) {
        cfg_emit!(__time_02_instant_checked_ops);
        cfg_emit!(__time_02_nonzero_signed);
    }

    // `use <trait> as _;` was stabilized in 1.33.0.
    if rustc::is_min_version("1.33.0").unwrap_or(false) {
        cfg_emit!(__time_02_use_trait_as_underscore);
    }
}
