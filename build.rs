extern crate gcc;

use std::default::Default;

fn main() {
    gcc::compile_library("libtime_helpers.a",
                         &Default::default(),
                         &["src/time_helpers.c"]);
}
