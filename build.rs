use std::env;

fn main() {
    if env::var("COMPILING_UNDER_CARGO_WEB").is_ok() {
        println!("cargo:rustc-cfg=cargoweb")
    }
}
