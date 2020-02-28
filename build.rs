use std::env;

fn cargo_web() {
    if env::var("COMPILING_UNDER_CARGO_WEB") == Ok("1".into()) {
        println!("cargo:rustc-cfg=cargo_web")
    }
}

fn features(features: &[(&str, &str)]) {
    for feature in features {
        if env::var(format!("CARGO_FEATURE_{}", feature.0)).is_ok() {
            println!("cargo:rustc-cfg={}", feature.1);
        }
    }
}

fn no_std() {
    if env::var("CARGO_FEATURE_STD").is_err() {
        println!("cargo:rustc-cfg=no_std");
    }
}

fn main() {
    cargo_web();
    features(&vec![
        ("STD", "std"),
        ("DEPRECATED", "v01_deprecated_api"),
        ("PANICKING_API", "panicking_api"),
        ("RAND", "rand"),
        ("SERDE", "serde"),
        ("__DOC", "docs"),
    ]);
    no_std();
}
