#[test]
fn from_hms_nanos_unchecked() {
    assert_eq!(
        Ok(time::internals::Time::from_hms_nanos_unchecked(0, 1, 2, 3)),
        time::Time::try_from_hms_nano(0, 1, 2, 3)
    );
}
