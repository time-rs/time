use std::env;
use version_check as rustc;

const MSRV: &str = "1.36.0";

fn main() {
    println!("cargo:rerun-if-env-changed=COMPILING_UNDER_CARGO_WEB");

    // Are we compiling with `cargo web`?
    if env::var("COMPILING_UNDER_CARGO_WEB") == Ok("1".into()) {
        println!("cargo:rustc-cfg=__time_02_cargo_web");
    }

    // Warn if the version is below MSRV.
    if !rustc::is_min_version(MSRV).unwrap_or(false) {
        println!(
            "cargo:warning=The time crate has a minimum supported rust version of {}.",
            MSRV
        );
    }

    // Warn if the `__doc` feature is used on stable or beta.
    if !rustc::Channel::read().map_or(false, |channel| channel.supports_features()) {
        #[cfg(__time_02_docs)]
        println!(
            "cargo:warning=`--cfg __time_02_docs` requires a nightly compiler, and is intended \
             for internal usage only."
        );
    }

    // `#[non_exhaustive]` was stabilized in 1.40.0.
    if rustc::is_min_version("1.40.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=__time_02_supports_non_exhaustive");
    }
}
