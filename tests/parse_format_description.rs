use core::num::NonZeroU16;

use time::error::InvalidFormatDescription;
use time::format_description::modifier::*;
use time::format_description::{self, Component, FormatItem, OwnedFormatItem};

mod iterator {
    use super::*;

    pub(super) fn padding() -> impl Iterator<Item = (Padding, &'static str)> {
        [
            (Padding::Space, "padding:space"),
            (Padding::Zero, "padding:zero"),
            (Padding::None, "padding:none"),
        ]
        .iter()
        .copied()
    }

    pub(super) fn hour_is_12_hour_clock() -> impl Iterator<Item = (bool, &'static str)> {
        [(false, "repr:24"), (true, "repr:12")].iter().copied()
    }

    pub(super) fn period_is_uppercase() -> impl Iterator<Item = (bool, &'static str)> {
        [(true, "case:upper"), (false, "case:lower")]
            .iter()
            .copied()
    }

    pub(super) fn month_repr() -> impl Iterator<Item = (MonthRepr, &'static str)> {
        [
            (MonthRepr::Numerical, "repr:numerical"),
            (MonthRepr::Long, "repr:long"),
            (MonthRepr::Short, "repr:short"),
        ]
        .iter()
        .copied()
    }

    pub(super) fn subsecond_digits() -> impl Iterator<Item = (SubsecondDigits, &'static str)> {
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
        .iter()
        .copied()
    }

    pub(super) fn weekday_repr() -> impl Iterator<Item = (WeekdayRepr, &'static str)> {
        [
            (WeekdayRepr::Short, "repr:short"),
            (WeekdayRepr::Long, "repr:long"),
            (WeekdayRepr::Sunday, "repr:sunday"),
            (WeekdayRepr::Monday, "repr:monday"),
        ]
        .iter()
        .copied()
    }

    pub(super) fn week_number_repr() -> impl Iterator<Item = (WeekNumberRepr, &'static str)> {
        [
            (WeekNumberRepr::Iso, "repr:iso"),
            (WeekNumberRepr::Sunday, "repr:sunday"),
            (WeekNumberRepr::Monday, "repr:monday"),
        ]
        .iter()
        .copied()
    }

    pub(super) fn year_repr() -> impl Iterator<Item = (YearRepr, &'static str)> {
        [
            (YearRepr::Full, "repr:full"),
            (YearRepr::LastTwo, "repr:last_two"),
        ]
        .iter()
        .copied()
    }

    pub(super) fn year_is_iso_week_based() -> impl Iterator<Item = (bool, &'static str)> {
        [(false, "base:calendar"), (true, "base:iso_week")]
            .iter()
            .copied()
    }

    pub(super) fn sign_is_mandatory() -> impl Iterator<Item = (bool, &'static str)> {
        [(false, "sign:automatic"), (true, "sign:mandatory")]
            .iter()
            .copied()
    }

    pub(super) fn weekday_is_one_indexed() -> impl Iterator<Item = (bool, &'static str)> {
        [(true, "one_indexed:true"), (false, "one_indexed:false")]
            .iter()
            .copied()
    }

    pub(super) fn case_sensitive() -> impl Iterator<Item = (bool, &'static str)> {
        [
            (true, "case_sensitive:true"),
            (false, "case_sensitive:false"),
        ]
        .iter()
        .copied()
    }

    pub(super) fn ignore_count() -> impl Iterator<Item = (NonZeroU16, &'static str)> {
        [
            (1, "count:1"),
            (2, "count:2"),
            (3, "count:3"),
            (10, "count:10"),
            (100, "count:100"),
            (1_000, "count:1000"),
        ]
        .into_iter()
        .map(|(count, name)| {
            (
                NonZeroU16::new(count).expect("number should not be zero"),
                name,
            )
        })
    }

    pub(super) fn unix_timestamp_precision()
    -> impl Iterator<Item = (UnixTimestampPrecision, &'static str)> {
        [
            (UnixTimestampPrecision::Second, "precision:second"),
            (UnixTimestampPrecision::Millisecond, "precision:millisecond"),
            (UnixTimestampPrecision::Microsecond, "precision:microsecond"),
            (UnixTimestampPrecision::Nanosecond, "precision:nanosecond"),
        ]
        .into_iter()
    }
}

#[test]
fn empty() {
    assert_eq!(format_description::parse_borrowed::<2>(""), Ok(vec![]));
    assert_eq!(
        format_description::parse_owned::<2>(""),
        Ok(OwnedFormatItem::Compound(Box::new([])))
    );
}

#[test]
fn only_literal() {
    assert_eq!(
        format_description::parse("foo bar"),
        Ok(vec![FormatItem::Literal(b"foo bar")])
    );
    assert_eq!(
        format_description::parse("  leading spaces"),
        Ok(vec![FormatItem::Literal(b"  leading spaces")])
    );
    assert_eq!(
        format_description::parse("trailing spaces  "),
        Ok(vec![FormatItem::Literal(b"trailing spaces  ")])
    );
    assert_eq!(
        format_description::parse("     "),
        Ok(vec![FormatItem::Literal(b"     ")])
    );
    assert_eq!(
        format_description::parse("[["),
        Ok(vec![FormatItem::Literal(b"[")])
    );
    assert_eq!(
        format_description::parse("foo[[bar"),
        Ok(vec![
            FormatItem::Literal(b"foo"),
            FormatItem::Literal(b"["),
            FormatItem::Literal(b"bar")
        ])
    );
}

#[test]
fn simple_component() {
    assert_eq!(
        format_description::parse("[day]"),
        Ok(vec![FormatItem::Component(Component::Day(modifier!(
            Day {
                padding: Padding::Zero
            }
        )))])
    );
    assert_eq!(
        format_description::parse("[hour]"),
        Ok(vec![FormatItem::Component(Component::Hour(modifier!(
            Hour {
                padding: Padding::Zero,
                is_12_hour_clock: false
            }
        )))])
    );
    assert_eq!(
        format_description::parse("[minute]"),
        Ok(vec![FormatItem::Component(Component::Minute(modifier!(
            Minute {
                padding: Padding::Zero
            }
        )))])
    );
    assert_eq!(
        format_description::parse("[month]"),
        Ok(vec![FormatItem::Component(Component::Month(modifier!(
            Month {
                padding: Padding::Zero,
                repr: MonthRepr::Numerical
            }
        )))])
    );
    assert_eq!(
        format_description::parse("[offset_hour]"),
        Ok(vec![FormatItem::Component(Component::OffsetHour(
            modifier!(OffsetHour {
                sign_is_mandatory: false,
                padding: Padding::Zero
            })
        ))])
    );
    assert_eq!(
        format_description::parse("[offset_minute]"),
        Ok(vec![FormatItem::Component(Component::OffsetMinute(
            modifier!(OffsetMinute {
                padding: Padding::Zero
            })
        ))])
    );
    assert_eq!(
        format_description::parse("[offset_second]"),
        Ok(vec![FormatItem::Component(Component::OffsetSecond(
            modifier!(OffsetSecond {
                padding: Padding::Zero
            })
        ))])
    );
    assert_eq!(
        format_description::parse("[ordinal]"),
        Ok(vec![FormatItem::Component(Component::Ordinal(modifier!(
            Ordinal {
                padding: Padding::Zero
            }
        )))])
    );
    assert_eq!(
        format_description::parse("[period]"),
        Ok(vec![FormatItem::Component(Component::Period(modifier!(
            Period { is_uppercase: true }
        )))])
    );
    assert_eq!(
        format_description::parse("[second]"),
        Ok(vec![FormatItem::Component(Component::Second(modifier!(
            Second {
                padding: Padding::Zero
            }
        )))])
    );
    assert_eq!(
        format_description::parse("[subsecond]"),
        Ok(vec![FormatItem::Component(Component::Subsecond(
            modifier!(Subsecond {
                digits: SubsecondDigits::OneOrMore
            })
        ))])
    );
    assert_eq!(
        format_description::parse("[unix_timestamp]"),
        Ok(vec![FormatItem::Component(Component::UnixTimestamp(
            modifier!(UnixTimestamp {
                precision: UnixTimestampPrecision::Second,
                sign_is_mandatory: false,
            })
        ))])
    );
    assert_eq!(
        format_description::parse("[weekday]"),
        Ok(vec![FormatItem::Component(Component::Weekday(modifier!(
            Weekday {
                repr: WeekdayRepr::Long,
                one_indexed: true,
            }
        )))])
    );
    assert_eq!(
        format_description::parse("[week_number]"),
        Ok(vec![FormatItem::Component(Component::WeekNumber(
            modifier!(WeekNumber {
                padding: Padding::Zero,
                repr: WeekNumberRepr::Iso
            })
        ))])
    );
    assert_eq!(
        format_description::parse("[year]"),
        Ok(vec![FormatItem::Component(Component::Year(modifier!(
            Year {
                padding: Padding::Zero,
                repr: YearRepr::Full,
                iso_week_based: false,
                sign_is_mandatory: false
            }
        )))])
    );
}

#[test]
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

#[test]
fn component_with_modifiers() {
    for (padding, padding_str) in iterator::padding() {
        assert_eq!(
            format_description::parse(&format!("[day {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::Day(modifier!(
                Day { padding }
            )))])
        );
        assert_eq!(
            format_description::parse(&format!("[minute {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::Minute(modifier!(
                Minute { padding }
            )))])
        );
        assert_eq!(
            format_description::parse(&format!("[offset_minute {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::OffsetMinute(
                modifier!(OffsetMinute { padding })
            ))])
        );
        assert_eq!(
            format_description::parse(&format!("[offset_second {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::OffsetSecond(
                modifier!(OffsetSecond { padding })
            ))])
        );
        assert_eq!(
            format_description::parse(&format!("[ordinal {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::Ordinal(modifier!(
                Ordinal { padding }
            )))])
        );
        assert_eq!(
            format_description::parse(&format!("[second {padding_str}]")),
            Ok(vec![FormatItem::Component(Component::Second(modifier!(
                Second { padding }
            )))])
        );

        for (is_12_hour_clock, is_12_hour_clock_str) in iterator::hour_is_12_hour_clock() {
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
        for (case_sensitive, case_sensitive_repr) in iterator::case_sensitive() {
            for (repr, repr_str) in iterator::month_repr() {
                assert_eq!(
                    format_description::parse(&format!(
                        "[month {padding_str} {case_sensitive_repr} {repr_str}]"
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
            for (is_uppercase, is_uppercase_str) in iterator::period_is_uppercase() {
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
            for (repr, repr_str) in iterator::weekday_repr() {
                for (one_indexed, one_indexed_str) in iterator::weekday_is_one_indexed() {
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
                    );
                }
            }
        }
        for (repr, repr_str) in iterator::week_number_repr() {
            assert_eq!(
                format_description::parse(&format!("[week_number {padding_str} {repr_str}]")),
                Ok(vec![FormatItem::Component(Component::WeekNumber(
                    modifier!(WeekNumber { padding, repr })
                ))])
            );
        }
        for (sign_is_mandatory, sign_is_mandatory_str) in iterator::sign_is_mandatory() {
            assert_eq!(
                format_description::parse(&format!(
                    "[offset_hour {padding_str} {sign_is_mandatory_str}]"
                )),
                Ok(vec![FormatItem::Component(Component::OffsetHour(
                    modifier!(OffsetHour {
                        sign_is_mandatory,
                        padding
                    })
                ))])
            );

            for (repr, repr_str) in iterator::year_repr() {
                for (iso_week_based, iso_week_based_str) in iterator::year_is_iso_week_based() {
                    assert_eq!(
                        format_description::parse(&format!(
                            "[year {padding_str} {repr_str} {iso_week_based_str} \
                             \n{sign_is_mandatory_str}]",
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
        }
    }

    for (sign_is_mandatory, sign_is_mandatory_str) in iterator::sign_is_mandatory() {
        for (unix_timestamp_precision, unix_timestamp_precision_str) in
            iterator::unix_timestamp_precision()
        {
            assert_eq!(
                format_description::parse(&format!(
                    "[unix_timestamp {sign_is_mandatory_str} {unix_timestamp_precision_str}]"
                )),
                Ok(vec![FormatItem::Component(Component::UnixTimestamp(
                    modifier!(UnixTimestamp {
                        sign_is_mandatory,
                        precision: unix_timestamp_precision
                    })
                ))])
            );
        }
    }

    for (digits, digits_str) in iterator::subsecond_digits() {
        assert_eq!(
            format_description::parse(&format!("[subsecond {digits_str}]")),
            Ok(vec![FormatItem::Component(Component::Subsecond(
                modifier!(Subsecond { digits })
            ))])
        );
    }

    for (count, count_str) in iterator::ignore_count() {
        assert_eq!(
            format_description::parse(&format!("[ignore {count_str}]")),
            Ok(vec![FormatItem::Component(Component::Ignore(
                Ignore::count(count)
            ))])
        );
    }
}

#[test]
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

#[test]
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

#[test]
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

#[test]
fn backslash_escape_error() {
    assert!(matches!(
        format_description::parse_owned::<2>(r"\a"),
        Err(InvalidFormatDescription::Expected {
            what: "valid escape sequence",
            index: 1,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_owned::<2>(r"\"),
        Err(InvalidFormatDescription::Expected {
            what: "valid escape sequence",
            index: 0,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_borrowed::<2>(r"\a"),
        Err(InvalidFormatDescription::Expected {
            what: "valid escape sequence",
            index: 1,
            ..
        })
    ));
    assert!(matches!(
        format_description::parse_borrowed::<2>(r"\"),
        Err(InvalidFormatDescription::Expected {
            what: "valid escape sequence",
            index: 0,
            ..
        })
    ));
}

#[test]
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

#[test]
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

#[test]
fn error_display() {
    assert_eq!(
        format_description::parse("[").unwrap_err().to_string(),
        "missing component name at byte index 0"
    );
    assert_eq!(
        format_description::parse("[foo").unwrap_err().to_string(),
        "unclosed opening bracket at byte index 0"
    );
    assert_eq!(
        format_description::parse("[foo]").unwrap_err().to_string(),
        "invalid component name `foo` at byte index 1"
    );
    assert_eq!(
        format_description::parse("[day bar]")
            .unwrap_err()
            .to_string(),
        "invalid modifier `bar` at byte index 5"
    );
    assert_eq!(
        format_description::parse("[]").unwrap_err().to_string(),
        "missing component name at byte index 0"
    );
    assert_eq!(
        format_description::parse_owned::<2>("[optional ")
            .unwrap_err()
            .to_string(),
        "expected opening bracket at byte index 9"
    );
    assert_eq!(
        format_description::parse("[optional []]")
            .unwrap_err()
            .to_string(),
        "optional item is not supported in runtime-parsed format descriptions at byte index 0"
    );
    assert_eq!(
        format_description::parse("[ignore]")
            .unwrap_err()
            .to_string(),
        "missing required modifier `count` for component at byte index 1"
    );
}

#[test]
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
