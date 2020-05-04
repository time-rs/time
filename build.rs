use std::env;
use version_check as rustc;

const MSRV: &str = "1.36.0";

macro_rules! cfg_emit {
    ($s:ident) => {
        println!(concat!("cargo:rustc-cfg=", stringify!($s)));
    };
}

macro_rules! cfg_aliases {
    ($($feature:literal => $alias:ident),* $(,)*) => {$(
        #[cfg(feature = $feature)]
        cfg_emit!($alias);
    )*};
}

macro_rules! warning {
    ($($s:tt)*) => {
        println!("cargo:warning={}", format_args!($($s)*));
    };
}

fn main() {
    // Alias `feature = "foo"`, allowing shorter usage and the possibility for
    // renaming.
    cfg_aliases! {
        "std" => std,
        "rand" => rand,
        "serde" => serde,
        "macros" => macros,
        "local-offset" => local_offset,
        "__doc" => docs,
    };

    // Are we compiling with `cargo web`?
    if env::var("COMPILING_UNDER_CARGO_WEB") == Ok("1".into()) {
        cfg_emit!(cargo_web);
    }

    // Warn if the version is below MSRV.
    if !rustc::is_min_version(MSRV).unwrap_or(false) {
        warning!(
            "The time crate has a minimum supported rust version of {}.",
            MSRV
        );
    }

    // Warn if the `__doc` feature is used on stable or beta.
    if !rustc::Channel::read().map_or(false, |channel| channel.supports_features()) {
        #[cfg(feature = "__doc")]
        warning!(
            "The `__doc` feature requires a nightly compiler, and is intended for internal usage \
             only."
        );
    }

    // ==== features that affect runtime directly ====

    // `#[non_exhaustive]` was stabilized in 1.40.0.
    if rustc::is_min_version("1.40.0").unwrap_or(false) {
        cfg_emit!(supports_non_exhaustive);
    }

    // `(-5).abs()` is `const`-capable beginning in 1.39.0.
    if rustc::is_min_version("1.39.0").unwrap_or(false) {
        cfg_emit!(const_num_abs);
    }
}
