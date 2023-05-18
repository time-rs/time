#![deny(
    anonymous_parameters,
    clippy::all,
    clippy::undocumented_unsafe_blocks,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::print_stdout,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::clone_on_copy,
    clippy::cmp_owned,
    clippy::cognitive_complexity,
    clippy::missing_const_for_fn,
    clippy::unwrap_used
)]

#[cfg(not(all(
    feature = "default",
    feature = "alloc",
    feature = "formatting",
    feature = "large-dates",
    feature = "local-offset",
    feature = "macros",
    feature = "parsing",
    feature = "quickcheck",
    feature = "serde-human-readable",
    feature = "serde-well-known",
    feature = "std",
    feature = "rand",
    feature = "serde",
)))]
#[test]
fn run_with_all_features() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    struct Error(std::process::ExitStatus);

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl std::error::Error for Error {}

    let status = std::process::Command::new("cargo")
        .args(&["test", "--all-features"])
        .status()?;

    return if status.success() {
        Ok(())
    } else {
        Err(Box::new(Error(status)))
    };

    // Intentionally unreachable. This is to show the user a warning when they don't provide
    // `--all-features`.
    "Tests must be run with `--all-features`. Because the flag was not provided, `cargo test \
     --all-features` is run.";
}

macro_rules! require_all_features {
    ($($x:item)*) => {$(
        #[cfg(all(
            feature = "default",
            feature = "alloc",
            feature = "formatting",
            feature = "large-dates",
            feature = "local-offset",
            feature = "macros",
            feature = "parsing",
            feature = "quickcheck",
            feature = "serde-human-readable",
            feature = "serde-well-known",
            feature = "std",
            feature = "rand",
            feature = "serde",
        ))]
        $x
    )*};
}

require_all_features! {
    use std::sync::Mutex;

    /// A lock to ensure that certain tests don't run in parallel, which could lead to a test
    /// unexpectedly failing.
    static SOUNDNESS_LOCK: Mutex<()> = Mutex::new(());

    /// Construct a non-exhaustive modifier.
    macro_rules! modifier {
        ($name:ident {
            $($field:ident $(: $value:expr)?),+ $(,)?
        }) => {{
            let mut value = ::time::format_description::modifier::$name::default();
            $(value.$field = modifier!(@value $field $($value)?);)+
            value
        }};

        (@value $field:ident) => ($field);
        (@value $field:ident $value:expr) => ($value);
    }

    /// Assert that the given expression panics.
    macro_rules! assert_panic {
        ($($x:tt)*) => {
            assert!(std::panic::catch_unwind(|| {
                $($x)*
            })
            .is_err())
        }
    }

    mod date;
    mod derives;
    mod duration;
    mod error;
    mod ext;
    mod format_description;
    mod formatting;
    mod instant;
    mod macros;
    mod meta;
    mod month;
    mod offset_date_time;
    mod parse_format_description;
    mod parsed;
    mod parsing;
    mod primitive_date_time;
    #[path = "quickcheck.rs"]
    mod quickcheck_mod;
    mod rand;
    mod serde;
    mod serde_helpers;
    mod time;
    mod utc_offset;
    mod util;
    mod weekday;

    #[cfg(__ui_tests)]
    #[test]
    fn compile_fail() {
        let tests = trybuild::TestCases::new();
        // Path is relative from `time/Cargo.toml`.
        tests.compile_fail("../tests/compile-fail/*.rs");
    }
}
