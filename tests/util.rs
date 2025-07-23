use rstest::rstest;
use time::Month::*;
use time::{util, Month};

#[rstest]
#[case(2019, January, 31)]
#[case(2019, February, 28)]
#[case(2019, March, 31)]
#[case(2019, April, 30)]
#[case(2019, May, 31)]
#[case(2019, June, 30)]
#[case(2019, July, 31)]
#[case(2019, August, 31)]
#[case(2019, September, 30)]
#[case(2019, October, 31)]
#[case(2019, November, 30)]
#[case(2019, December, 31)]
#[case(2020, January, 31)]
#[case(2020, February, 29)]
#[case(2020, March, 31)]
#[case(2020, April, 30)]
#[case(2020, May, 31)]
#[case(2020, June, 30)]
#[case(2020, July, 31)]
#[case(2020, August, 31)]
#[case(2020, September, 30)]
#[case(2020, October, 31)]
#[case(2020, November, 30)]
#[case(2020, December, 31)]
fn days_in_year_month(#[case] year: i32, #[case] month: Month, #[case] expected: u8) {
    #[expect(deprecated)]
    {
        assert_eq!(util::days_in_year_month(year, month), expected);
    }
}

#[rstest]
#[case(1900, false)]
#[case(2000, true)]
#[case(2004, true)]
#[case(2005, false)]
#[case(2100, false)]
fn is_leap_year(#[case] year: i32, #[case] expected: bool) {
    assert_eq!(util::is_leap_year(year), expected);
}

#[rstest]
#[case(1900, 365)]
#[case(2000, 366)]
#[case(2004, 366)]
#[case(2005, 365)]
#[case(2100, 365)]
fn days_in_year(#[case] year: i32, #[case] expected: u16) {
    assert_eq!(util::days_in_year(year), expected);
}

#[rstest]
fn weeks_in_year() {
    let num_weeks_for_years = [
        52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52,
        52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52,
        52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52,
        52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52,
        53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52,
        53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52,
        53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53,
        52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53,
        52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52,
        52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52,
        52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52,
        52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52,
        52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52, 52, 53, 52, 52, 52, 52, 52, 53, 52, 52, 52,
        52, 53, 52, 52, 52, 52, 52, 53, 52,
    ];

    for (year, &num_weeks) in (0..400).zip(&num_weeks_for_years) {
        assert_eq!(util::weeks_in_year(year), num_weeks);
    }
}

#[rstest]
#[expect(deprecated)]
fn local_offset_soundness() {
    use time::util::local_offset::*;

    // These functions no longer do anything so they always return `Sound`.
    assert_eq!(get_soundness(), Soundness::Sound);
    // Safety: This no longer has any safety requirements.
    unsafe { set_soundness(Soundness::Unsound) };
    assert_eq!(get_soundness(), Soundness::Sound);
    // Safety: See above.
    unsafe { set_soundness(Soundness::Sound) };
    assert_eq!(get_soundness(), Soundness::Sound);
}
