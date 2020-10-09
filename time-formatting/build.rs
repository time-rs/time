use version_check as rustc;

fn main() {
    // `#[non_exhaustive]` was stabilized in 1.40.0.
    if rustc::is_min_version("1.40.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=__time_formatting_01_supports_non_exhaustive");
    }
}
