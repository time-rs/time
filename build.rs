extern crate rustc_version;
use rustc_version::{version, Version};

fn main() {
    if version().unwrap() >= Version::parse("1.31.0").unwrap() {
        println!("cargo:rustc-cfg=feature=\"has_const_fn\"");
    }
}
