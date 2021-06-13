use time::{util, PrimitiveDateTime, Time};

#[test]
fn format_edge_cases() {
    let time = Time::midnight();
    assert_eq!(time.format("%H foo"), "00 foo"); // Trailing literal
    assert_eq!(time.format("%H%%"), "00%"); // Literal `%`
    assert!(util::validate_format_string("%").is_err()); // Standalone `%`
}

#[test]
fn issue_329() {
    let input = "2013-10-07 04:23:19.120";
    let result = PrimitiveDateTime::parse(input, "%F %T.%N");
    assert!(result.is_err());
}
