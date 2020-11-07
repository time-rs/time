#![cfg(feature = "alloc")]

use time::{util, Format, Time};

#[test]
fn format_edge_cases() {
    let time = Time::midnight();
    assert_eq!(time.format("%H foo"), "00 foo"); // Trailing literal
    assert_eq!(time.format("%H%%"), "00%"); // Literal `%`
    assert!(util::validate_format_string("%").is_err()); // Standalone `%`
}

#[test]
#[should_panic]
fn insufficient_information() {
    Time::midnight().format(Format::Rfc3339);
}
