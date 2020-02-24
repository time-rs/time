use std::env;

fn main() {
    if env::var("COMPILING_UNDER_CARGO_WEB") == Ok("1".into()) {
        println!("cargo:rustc-cfg=cargo_web")
    }
}
