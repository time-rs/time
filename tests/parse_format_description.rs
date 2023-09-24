use core::num::NonZeroU16;

use itertools::iproduct;
use rstest::{fixture, rstest};
use time::error::InvalidFormatDescription;
use time::format_description::modifier::*;
use time::format_description::{self, Component, FormatItem, OwnedFormatItem};

// Use type aliases to avoid writing out the full type in every test.
type PaddingIter = [(Padding, &'static str); 3];
type HourIs12HourClockIter = [(bool, &'static str); 2];
type PeriodIsUppercaseIter = [(bool, &'static str); 2];
type MonthReprIter = [(MonthRepr, &'static str); 3];
type SubsecondDigitsIter = [(SubsecondDigits, &'static str); 10];
type WeekdayReprIter = [(WeekdayRepr, &'static str); 4];
type WeekNumberReprIter = [(WeekNumberRepr, &'static str); 3];
type YearReprIter = [(YearRepr, &'static str); 2];
type YearIsIsoWeekBasedIter = [(bool, &'static str); 2];
type SignIsMandatoryIter = [(bool, &'static str); 2];
type WeekdayIsOneIndexedIter = [(bool, &'static str); 2];
type CaseSensitiveIter = [(bool, &'static str); 2];
type IgnoreCountIter = [(NonZeroU16, &'static str); 6];
type UnixTimestampPrecisionIter = [(UnixTimestampPrecision, &'static str); 4];

// region: fixtures
#[fixture]
fn padding() -> PaddingIter {
    [
        (Padding::Space, "padding:space"),
        (Padding::Zero, "padding:zero"),
        (Padding::None, "padding:none"),
    ]
}

#[fixture]
fn hour_is_12_hour_clock() -> HourIs12HourClockIter {
    [(false, "repr:24"), (true, "repr:12")]
}

#[fixture]
fn period_is_uppercase() -> PeriodIsUppercaseIter {
    [(true, "case:upper"), (false, "case:lower")]
}

#[fixture]
fn month_repr() -> MonthReprIter {
    [
        (MonthRepr::Numerical, "repr:numerical"),
        (MonthRepr::Long, "repr:long"),
        (MonthRepr::Short, "repr:short"),
    ]
}

#[fixture]
fn subsecond_digits() -> SubsecondDigitsIter {
    [
        (SubsecondDigits::One, "digits:1"),
        (SubsecondDigits::Two, "digits:2"),
        (SubsecondDigits::Three, "digits:3"),
        (SubsecondDigits::Four, "digits:4"),
        (SubsecondDigits::Five, "digits:5"),
        (SubsecondDigits::Six, "digits:6"),
        (SubsecondDigits::Seven, "digits:7"),
        (SubsecondDigits::Eight, "digits:8"),
        (SubsecondDigits::Nine, "digits:9"),
        (SubsecondDigits::OneOrMore, "digits:1+"),
    ]
}

#[fixture]
fn weekday_repr() -> WeekdayReprIter {
    [
        (WeekdayRepr::Short, "repr:short"),
        (WeekdayRepr::Long, "repr:long"),
        (WeekdayRepr::Sunday, "repr:sunday"),
        (WeekdayRepr::Monday, "repr:monday"),
    ]
}

#[fixture]
fn week_number_repr() -> WeekNumberReprIter {
    [
        (WeekNumberRepr::Iso, "repr:iso"),
        (WeekNumberRepr::Sunday, "repr:sunday"),
        (WeekNumberRepr::Monday, "repr:monday"),
    ]
}

#[fixture]
fn year_repr() -> YearReprIter {
    [
        (YearRepr::Full, "repr:full"),
        (YearRepr::LastTwo, "repr:last_two"),
    ]
}

#[fixture]
fn year_is_iso_week_based() -> YearIsIsoWeekBasedIter {
    [(false, "base:calendar"), (true, "base:iso_week")]
}

#[fixture]
fn sign_is_mandatory() -> SignIsMandatoryIter {
    [(false, "sign:automatic"), (true, "sign:mandatory")]
}

#[fixture]
fn weekday_is_one_indexed() -> WeekdayIsOneIndexedIter {
    [(true, "one_indexed:true"), (false, "one_indexed:false")]
}

#[fixture]
fn case_sensitive() -> CaseSensitiveIter {
    [
        (true, "case_sensitive:true"),
        (false, "case_sensitive:false"),
    ]
}

#[fixture]
#[allow(clippy::unwrap_used)] // all values are valid
fn ignore_count() -> IgnoreCountIter {
    [
        (NonZeroU16::new(1).unwrap(), "count:1"),
        (NonZeroU16::new(2).unwrap(), "count:2"),
        (NonZeroU16::new(3).unwrap(), "count:3"),
        (NonZeroU16::new(10).unwrap(), "count:10"),
        (NonZeroU16::new(100).unwrap(), "count:100"),
        (NonZeroU16::new(1_000).unwrap(), "count:1000"),
    ]
}

#[fixture]
fn unix_timestamp_precision() -> UnixTimestampPrecisionIter {
    [
        (UnixTimestampPrecision::Second, "precision:second"),
        (UnixTimestampPrecision::Millisecond, "precision:millisecond"),
        (UnixTimestampPrecision::Microsecond, "precision:microsecond"),
        (UnixTimestampPrecision::Nanosecond, "precision:nanosecond"),
    ]
}
// endregion fixtures

#[rstest]
fn empty() {
    assert_eq!(format_description::parse_borrowed::<2>(""), Ok(vec![]));
    assert_eq!(
        format_description::parse_owned::<2>(""),
        Ok(OwnedFormatItem::Compound(Box::new([])))
    );
}

#[rstest]
#[case("foo bar", [b"foo bar".as_slice()])]
#[case("  leading spaces", [b"  leading spaces".as_slice()])]
#[case("trailing spaces  ", [b"trailing spaces  ".as_slice()])]
#[case("     ", [b"     ".as_slice()])]
#[case("[[", [b"[".as_slice()])]
#[case("foo[[bar", [b"foo".as_slice(), b"[".as_slice(), b"bar".as_slice()])]
fn only_literal<const N: usize>(#[case] format_description: &str, #[case] expected: [&[u8]; N]) {
    assert_eq!(
        format_description::parse(format_description),
        Ok(expected.into_iter().map(FormatItem::Literal).collect())
    );
}

#[rstest]
#[case("[day]", Component::Day(modifier!(Day)))]
#[case("[end]", Component::End(modifier!(End)))]
#[case("[hour]", Component::Hour(modifier!(Hour)))]
#[case("[minute]", Component::Minute(modifier!(Minute)))]
#[case("[month]", Component::Month(modifier!(Month)))]
#[case("[offset_hour]", Component::OffsetHour(modifier!(OffsetHour)))]
#[case("[offset_minute]", Component::OffsetMinute(modifier!(OffsetMinute)))]
#[case("[offset_second]", Component::OffsetSecond(modifier!(OffsetSecond)))]
#[case("[ordinal]", Component::Ordinal(modifier!(Ordinal)))]
#[case("[period]", Component::Period(modifier!(Period)))]
#[case("[second]", Component::Second(modifier!(Second)))]
#[case("[subsecond]", Component::Subsecond(modifier!(Subsecond)))]
#[case("[unix_timestamp]", Component::UnixTimestamp(modifier!(UnixTimestamp)))]
#[case("[weekday]", Component::Weekday(modifier!(Weekday)))]
#[case("[week_number]", Component::WeekNumber(modifier!(WeekNumber)))]
#[case("[year]", Component::Year(modifier!(Year)))]
fn simple_component(#[case] format_description: &str, #[case] component: Component) {
    assert_eq!(
        format_description::parse(format_description),
        Ok(vec![FormatItem::Component(component)])
    );
}

#[allow(clippy::cognitive_complexity)] // all test the same thing
#[rstest]
fn errors() {
    use InvalidFormatDescription::*;

    macro_rules! assert_errs {
        ($($format_description:literal, $error:pat $(if $condition:expr)?,)*) => {$(
            assert!(matches!(
                format_description::parse($format_description),
                Err($error) $(if $condition)?
            ));
            assert!(matches!(
                format_description::parse_owned::<2>($format_description),
                Err($error) $(if $condition)?
            ));
        )*};
    }

    assert_errs! {
        "[ invalid ]", InvalidComponentName { name, index: 2, .. } if name == "invalid",
        "[", MissingComponentName { index: 0, .. },
        "[ ", MissingComponentName { index: 1, .. },
        "[]", MissingComponentName { index: 0, .. },
        "[day sign:mandatory]", InvalidModifier { value, index: 5, .. } if value == "sign",
        "[day sign:]", InvalidModifier { value, index: 9,.. } if value.is_empty(),
        "[day :mandatory]", InvalidModifier { value, index: 5,.. } if value.is_empty(),
        "[day sign:mandatory", UnclosedOpeningBracket { index: 0, .. },
        "[day padding:invalid]", InvalidModifier { value, index: 13, .. } if value == "invalid",
        "[ignore]", MissingRequiredModifier { name: "count", index: 1, .. },
        "[ignore count:70000]", InvalidModifier { value, index: 14, .. } if value == "70000",
    }
}

// region: individual components
#[rstest]
fn day_component(padding: PaddingIter) {
    for (padding, padding_str) in padding {
        assert_eq!(
            format_description::parse(&format!("[day {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::Day(modifier!(
                Day { padding }
            )))])
        );
    }
}

#[rstest]
fn minute_component(padding: PaddingIter) {
    for (padding, padding_str) in padding {
        assert_eq!(
            format_description::parse(&format!("[minute {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::Minute(modifier!(
                Minute { padding }
            )))])
        );
    }
}

#[rstest]
fn offset_minute_component(padding: PaddingIter) {
    for (padding, padding_str) in padding {
        assert_eq!(
            format_description::parse(&format!("[offset_minute {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::OffsetMinute(
                modifier!(OffsetMinute { padding })
            ))])
        );
    }
}

#[rstest]
fn offset_second_component(padding: PaddingIter) {
    for (padding, padding_str) in padding {
        assert_eq!(
            format_description::parse(&format!("[offset_second {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::OffsetSecond(
                modifier!(OffsetSecond { padding })
            ))])
        );
    }
}

#[rstest]
fn ordinal_component(padding: PaddingIter) {
    for (padding, padding_str) in padding {
        assert_eq!(
            format_description::parse(&format!("[ordinal {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::Ordinal(modifier!(
                Ordinal { padding }
            )))])
        );
    }
}

#[rstest]
fn second_component(padding: PaddingIter) {
    for (padding, padding_str) in padding {
        assert_eq!(
            format_description::parse(&format!("[second {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::Second(modifier!(
                Second { padding }
            )))])
        );
    }
}

#[rstest]
fn hour_component(padding: PaddingIter, hour_is_12_hour_clock: HourIs12HourClockIter) {
    for ((padding, padding_str), (is_12_hour_clock, is_12_hour_clock_str)) in
        iproduct!(padding, hour_is_12_hour_clock)
    {
        assert_eq!(
            format_description::parse(&format!("[hour {padding_str} {is_12_hour_clock_str}]")),
            Ok(vec![FormatItem::Component(Component::Hour(modifier!(
                Hour {
                    padding,
                    is_12_hour_clock
                }
            )))])
        );
    }
}

#[rstest]
fn month_component(
    padding: PaddingIter,
    case_sensitive: CaseSensitiveIter,
    month_repr: MonthReprIter,
) {
    for ((padding, padding_str), (repr, repr_str), (case_sensitive, case_sensitive_str)) in
        iproduct!(padding, month_repr, case_sensitive)
    {
        assert_eq!(
            format_description::parse(&format!(
                "[month {padding_str} {case_sensitive_str} {repr_str}]"
            )),
            Ok(vec![FormatItem::Component(Component::Month(modifier!(
                Month {
                    padding,
                    repr,
                    case_sensitive
                }
            )))])
        );
    }
}

#[rstest]
fn period_component(case_sensitive: CaseSensitiveIter, period_is_uppercase: PeriodIsUppercaseIter) {
    for ((case_sensitive, case_sensitive_repr), (is_uppercase, is_uppercase_str)) in
        iproduct!(case_sensitive, period_is_uppercase)
    {
        assert_eq!(
            format_description::parse(&format!(
                "[period {is_uppercase_str} {case_sensitive_repr}]"
            )),
            Ok(vec![FormatItem::Component(Component::Period(modifier!(
                Period {
                    is_uppercase,
                    case_sensitive
                }
            )))])
        );
    }
}

#[rstest]
fn weekday_component(
    case_sensitive: CaseSensitiveIter,
    weekday_is_one_indexed: WeekdayIsOneIndexedIter,
    weekday_repr: WeekdayReprIter,
) {
    for ((case_sensitive, case_sensitive_repr), (one_indexed, one_indexed_str), (repr, repr_str)) in
        iproduct!(case_sensitive, weekday_is_one_indexed, weekday_repr)
    {
        assert_eq!(
            format_description::parse(&format!(
                "[weekday {repr_str} {one_indexed_str} {case_sensitive_repr} ]"
            )),
            Ok(vec![FormatItem::Component(Component::Weekday(modifier!(
                Weekday {
                    repr,
                    one_indexed,
                    case_sensitive
                }
            )))])
        )
    }
}

#[rstest]
fn week_number_component(padding: PaddingIter, week_number_repr: WeekNumberReprIter) {
    for ((padding, padding_str), (repr, repr_str)) in iproduct!(padding, week_number_repr) {
        assert_eq!(
            format_description::parse(&format!("[week_number {padding_str} {repr_str}]")),
            Ok(vec![FormatItem::Component(Component::WeekNumber(
                modifier!(WeekNumber { padding, repr })
            ))])
        );
    }
}

#[rstest]
fn offset_hour_component(padding: PaddingIter, sign_is_mandatory: SignIsMandatoryIter) {
    for ((padding, padding_str), (sign_is_mandatory, sign_is_mandatory_str)) in
        iproduct!(padding, sign_is_mandatory)
    {
        assert_eq!(
            format_description::parse(&format!(
                "[offset_hour {padding_str} {sign_is_mandatory_str}]"
            )),
            Ok(vec![FormatItem::Component(Component::OffsetHour(
                modifier!(OffsetHour {
                    padding,
                    sign_is_mandatory
                })
            ))])
        );
    }
}

#[rstest]
fn year_component(
    padding: PaddingIter,
    year_repr: YearReprIter,
    year_is_iso_week_based: YearIsIsoWeekBasedIter,
    sign_is_mandatory: SignIsMandatoryIter,
) {
    for (
        (padding, padding_str),
        (repr, repr_str),
        (iso_week_based, iso_week_based_str),
        (sign_is_mandatory, sign_is_mandatory_str),
    ) in iproduct!(
        padding,
        year_repr,
        year_is_iso_week_based,
        sign_is_mandatory
    ) {
        assert_eq!(
            format_description::parse(&format!(
                "[year {padding_str} {repr_str} {iso_week_based_str} {sign_is_mandatory_str}]"
            )),
            Ok(vec![FormatItem::Component(Component::Year(modifier!(
                Year {
                    padding,
                    repr,
                    iso_week_based,
                    sign_is_mandatory
                }
            )))])
        );
    }
}

#[rstest]
fn unix_timestamp_component(
    sign_is_mandatory: SignIsMandatoryIter,
    unix_timestamp_precision: UnixTimestampPrecisionIter,
) {
    for ((sign_is_mandatory, sign_is_mandatory_str), (precision, precision_str)) in
        iproduct!(sign_is_mandatory, unix_timestamp_precision)
    {
        assert_eq!(
            format_description::parse(&format!(
                "[unix_timestamp {sign_is_mandatory_str} {precision_str}]"
            )),
            Ok(vec![FormatItem::Component(Component::UnixTimestamp(
                modifier!(UnixTimestamp {
                    sign_is_mandatory,
                    precision
                })
            ))])
        );
    }
}

#[rstest]
fn subsecond_component(subsecond_digits: SubsecondDigitsIter) {
    for (digits, digits_str) in subsecond_digits {
        assert_eq!(
            format_description::parse(&format!("[subsecond {digits_str}]")),
            Ok(vec![FormatItem::Component(Component::Subsecond(
                modifier!(Subsecond { digits })
            ))]),
        );
    }
}

#[rstest]
fn ignore_component(ignore_count: IgnoreCountIter) {
    for (count, count_str) in ignore_count {
        assert_eq!(
            format_description::parse(&format!("[ignore {count_str}]")),
            Ok(vec![FormatItem::Component(Component::Ignore(
                Ignore::count(count)
            ))])
        );
    }
}
// endregion individual components

#[rstest]
fn optional() {
    assert_eq!(
        format_description::parse_owned::<2>("[optional [:[year]]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Compound(Box::new([
                OwnedFormatItem::Literal(Box::new(*b":")),
                OwnedFormatItem::Component(Component::Year(Default::default()))
            ]))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>("[optional [[year]]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Component(Component::Year(Default::default()))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [\[]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Literal(Box::new(*b"["))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [ \[ ]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Compound(Box::new([
                OwnedFormatItem::Literal(Box::new(*b" ")),
                OwnedFormatItem::Literal(Box::new(*b"[")),
                OwnedFormatItem::Literal(Box::new(*b" ")),
            ]))
        )))
    );
}

#[rstest]
fn first() {
    assert_eq!(
        format_description::parse_owned::<2>("[first [a]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a"))
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>("[first [a] [b]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a")),
            OwnedFormatItem::Literal(Box::new(*b"b")),
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>("[first [a][b]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a")),
            OwnedFormatItem::Literal(Box::new(*b"b")),
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[first [a][\[]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a")),
            OwnedFormatItem::Literal(Box::new(*b"[")),
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[first [a][\[\[]]"),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"a")),
            OwnedFormatItem::Compound(Box::new([
                OwnedFormatItem::Literal(Box::new(*b"[")),
                OwnedFormatItem::Literal(Box::new(*b"[")),
            ]))
        ])))
    );
    assert_eq!(
        format_description::parse_owned::<2>(
            "[first [[period case:upper]] [[period case:lower]] ]"
        ),
        Ok(OwnedFormatItem::First(Box::new([
            OwnedFormatItem::Component(Component::Period(modifier!(Period {
                is_uppercase: true,
                case_sensitive: true,
            }))),
            OwnedFormatItem::Component(Component::Period(modifier!(Period {
                is_uppercase: false,
                case_sensitive: true,
            }))),
        ])))
    );
}

#[rstest]
fn backslash_escape() {
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [\]]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Literal(Box::new(*b"]"))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [\[]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Literal(Box::new(*b"["))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"[optional [\\]]"),
        Ok(OwnedFormatItem::Optional(Box::new(
            OwnedFormatItem::Literal(Box::new(*br"\"))
        )))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"\\"),
        Ok(OwnedFormatItem::Literal(Box::new(*br"\")))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"\["),
        Ok(OwnedFormatItem::Literal(Box::new(*br"[")))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"\]"),
        Ok(OwnedFormatItem::Literal(Box::new(*br"]")))
    );
    assert_eq!(
        format_description::parse_owned::<2>(r"foo\\"),
        Ok(OwnedFormatItem::Compound(Box::new([
            OwnedFormatItem::Literal(Box::new(*b"foo")),
            OwnedFormatItem::Literal(Box::new(*br"\")),
        ])))
    );
    assert_eq!(
        format_description::parse_borrowed::<2>(r"\\"),
        Ok(vec![FormatItem::Literal(br"\")])
    );
    assert_eq!(
        format_description::parse_borrowed::<2>(r"\["),
        Ok(vec![FormatItem::Literal(br"[")])
    );
    assert_eq!(
        format_description::parse_borrowed::<2>(r"\]"),
        Ok(vec![FormatItem::Literal(br"]")])
    );
    assert_eq!(
        format_description::parse_borrowed::<2>(r"foo\\"),
        Ok(vec![
            FormatItem::Literal(b"foo"),
            FormatItem::Literal(br"\"),
        ])
    );
}

#[rstest]
#[case(r"\a", 1)]
#[case(r"\", 0)]
fn backslash_escape_error(#[case] format_description: &str, #[case] expected_index: usize) {
    assert!(matches!(
        format_description::parse_owned::<2>(format_description),
        Err(InvalidFormatDescription::Expected {
            what: "valid escape sequence",
            index,
            ..
        }) if index == expected_index
    ));
    assert!(matches!(
        format_description::parse_borrowed::<2>(format_description),
        Err(InvalidFormatDescription::Expected {
            what: "valid escape sequence",
            index,
            ..
        }) if index == expected_index
    ));
}

#[rstest]
fn nested_v1_error() {
    assert!(matches!(
        format_description::parse_owned::<2>("[optional [[[]]"),
        Err(InvalidFormatDescription::MissingComponentName { index: 11, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional [ [[ ]]"),
        Err(InvalidFormatDescription::MissingComponentName { index: 12, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[first [a][[[]]"),
        Err(InvalidFormatDescription::UnclosedOpeningBracket { index: 0, .. })
    ));
}

#[rstest]
fn nested_error() {
    use InvalidFormatDescription::*;

    assert!(matches!(
        format_description::parse("[optional []]"),
        Err(NotSupported {
            what: "optional item",
            context: "runtime-parsed format descriptions",
            index: 0,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse("[first []]"),
        Err(NotSupported {
            what: "'first' item",
            context: "runtime-parsed format descriptions",
            index: 0,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[year [month]]"),
        Err(InvalidModifier { value, index: 6, .. }) if value == "["
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional[]]"),
        Err(Expected {
            what: "whitespace after `optional`",
            index: 8,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[first[]]"),
        Err(Expected {
            what: "whitespace after `first`",
            index: 5,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional []"),
        Err(UnclosedOpeningBracket { index: 0, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[first []"),
        Err(UnclosedOpeningBracket { index: 0, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional ["),
        Err(UnclosedOpeningBracket { index: 10, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional [[year"),
        Err(UnclosedOpeningBracket { index: 11, .. })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>("[optional "),
        Err(Expected {
            what: "opening bracket",
            index: 9,
            ..
        })
    ));
}

#[rstest]
#[case("[", "missing component name at byte index 0")]
#[case("[foo", "unclosed opening bracket at byte index 0")]
#[case("[foo]", "invalid component name `foo` at byte index 1")]
#[case("[day bar]", "invalid modifier `bar` at byte index 5")]
#[case("[]", "missing component name at byte index 0")]
#[case(
    "[optional []]",
    "optional item is not supported in runtime-parsed format descriptions at byte index 0"
)]
#[case(
    "[ignore]",
    "missing required modifier `count` for component at byte index 1"
)]
fn error_display(#[case] format_description: &str, #[case] error: &str) {
    // la10736/rstest#217
    #[allow(clippy::unwrap_used)] // It's the point of the test.
    let test = || {
        assert_eq!(
            format_description::parse(format_description)
                .unwrap_err()
                .to_string(),
            error
        );
    };

    test();
}

#[rstest]
#[case("[optional ", "expected opening bracket at byte index 9")]
fn error_display_owned(#[case] format_description: &str, #[case] error: &str) {
    // la10736/rstest#217
    #[allow(clippy::unwrap_used)] // It's the point of the test.
    let test = || {
        assert_eq!(
            format_description::parse_owned::<2>(format_description)
                .unwrap_err()
                .to_string(),
            error
        )
    };

    test();
}

#[rstest]
fn rfc_3339() {
    assert_eq!(
        format_description::parse(
            "[year]-[month repr:numerical]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour \
             sign:mandatory]:[offset_minute]"
        ),
        Ok(vec![
            FormatItem::Component(Component::Year(modifier!(Year {
                padding: Padding::Zero,
                repr: YearRepr::Full,
                iso_week_based: false,
                sign_is_mandatory: false
            }))),
            FormatItem::Literal(b"-"),
            FormatItem::Component(Component::Month(modifier!(Month {
                padding: Padding::Zero,
                repr: MonthRepr::Numerical
            }))),
            FormatItem::Literal(b"-"),
            FormatItem::Component(Component::Day(modifier!(Day {
                padding: Padding::Zero
            }))),
            FormatItem::Literal(b"T"),
            FormatItem::Component(Component::Hour(modifier!(Hour {
                padding: Padding::Zero,
                is_12_hour_clock: false
            }))),
            FormatItem::Literal(b":"),
            FormatItem::Component(Component::Minute(modifier!(Minute {
                padding: Padding::Zero
            }))),
            FormatItem::Literal(b":"),
            FormatItem::Component(Component::Second(modifier!(Second {
                padding: Padding::Zero
            }))),
            FormatItem::Literal(b"."),
            FormatItem::Component(Component::Subsecond(modifier!(Subsecond {
                digits: SubsecondDigits::OneOrMore
            }))),
            FormatItem::Component(Component::OffsetHour(modifier!(OffsetHour {
                padding: Padding::Zero,
                sign_is_mandatory: true
            }))),
            FormatItem::Literal(b":"),
            FormatItem::Component(Component::OffsetMinute(modifier!(OffsetMinute {
                padding: Padding::Zero
            })))
        ])
    );
}
