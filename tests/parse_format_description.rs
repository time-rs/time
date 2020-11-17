#![cfg(feature = "alloc")]

mod iterator {
    use time::formatting::modifier::{
        MonthRepr, Padding, SubsecondDigits, WeekNumberRepr, WeekdayRepr, YearRepr,
    };

    pub(super) fn padding() -> Vec<(Padding, &'static str)> {
        vec![
            (Padding::Space, "padding:space"),
            (Padding::Zero, "padding:zero"),
            (Padding::None, "padding:none"),
        ]
    }

    pub(super) fn hour_is_12_hour_clock() -> Vec<(bool, &'static str)> {
        vec![(false, "repr:24"), (true, "repr:12")]
    }

    pub(super) fn period_is_uppercase() -> Vec<(bool, &'static str)> {
        vec![(true, "case:upper"), (false, "case:lower")]
    }

    pub(super) fn month_repr() -> Vec<(MonthRepr, &'static str)> {
        vec![
            (MonthRepr::Numerical, "repr:numerical"),
            (MonthRepr::Long, "repr:long"),
            (MonthRepr::Short, "repr:short"),
        ]
    }

    pub(super) fn subsecond_digits() -> Vec<(SubsecondDigits, &'static str)> {
        vec![
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

    pub(super) fn weekday_repr() -> Vec<(WeekdayRepr, &'static str)> {
        vec![
            (WeekdayRepr::Short, "repr:short"),
            (WeekdayRepr::Long, "repr:long"),
            (WeekdayRepr::Sunday, "repr:sunday"),
            (WeekdayRepr::Monday, "repr:monday"),
        ]
    }

    pub(super) fn week_number_repr() -> Vec<(WeekNumberRepr, &'static str)> {
        vec![
            (WeekNumberRepr::Iso, "repr:iso"),
            (WeekNumberRepr::Sunday, "repr:sunday"),
            (WeekNumberRepr::Monday, "repr:monday"),
        ]
    }

    pub(super) fn year_repr() -> Vec<(YearRepr, &'static str)> {
        vec![
            (YearRepr::Full, "repr:full"),
            (YearRepr::Century, "repr:century"),
            (YearRepr::LastTwo, "repr:last_two"),
        ]
    }

    pub(super) fn year_is_iso_week_based() -> Vec<(bool, &'static str)> {
        vec![(false, "base:calendar"), (true, "base:iso_week")]
    }

    pub(super) fn sign_is_mandatory() -> Vec<(bool, &'static str)> {
        vec![(false, "sign:automatic"), (true, "sign:mandatory")]
    }

    pub(super) fn weekday_is_one_indexed() -> Vec<(bool, &'static str)> {
        vec![(true, "one_indexed:true"), (false, "one_indexed:false")]
    }
}

use time::formatting::{
    error::InvalidFormatDescription,
    modifier::{MonthRepr, Padding, SubsecondDigits, WeekNumberRepr, WeekdayRepr, YearRepr},
    parse_format_description, Component, FormatDescription,
};

#[test]
fn empty() {
    assert_eq!(parse_format_description(""), Ok(vec![]));
}

#[test]
fn only_literal() {
    assert_eq!(
        parse_format_description("foo bar"),
        Ok(vec![FormatDescription::Literal("foo bar")])
    );
    assert_eq!(
        parse_format_description("  leading spaces"),
        Ok(vec![FormatDescription::Literal("  leading spaces")])
    );
    assert_eq!(
        parse_format_description("trailing spaces  "),
        Ok(vec![FormatDescription::Literal("trailing spaces  ")])
    );
    assert_eq!(
        parse_format_description("[["),
        Ok(vec![FormatDescription::Literal("[")])
    );
    assert_eq!(
        parse_format_description("foo[[bar"),
        Ok(vec![
            FormatDescription::Literal("foo"),
            FormatDescription::Literal("["),
            FormatDescription::Literal("bar")
        ])
    );
}

#[test]
fn simple_component() {
    assert_eq!(
        parse_format_description("[day]"),
        Ok(vec![FormatDescription::Component(Component::Day {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[hour]"),
        Ok(vec![FormatDescription::Component(Component::Hour {
            padding: Padding::Zero,
            is_12_hour_clock: false
        })])
    );
    assert_eq!(
        parse_format_description("[minute]"),
        Ok(vec![FormatDescription::Component(Component::Minute {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[month]"),
        Ok(vec![FormatDescription::Component(Component::Month {
            padding: Padding::Zero,
            repr: MonthRepr::Long
        })])
    );
    assert_eq!(
        parse_format_description("[offset_hour]"),
        Ok(vec![FormatDescription::Component(Component::OffsetHour {
            sign_is_mandatory: false,
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[offset_minute]"),
        Ok(vec![FormatDescription::Component(
            Component::OffsetMinute {
                padding: Padding::Zero
            }
        )])
    );
    assert_eq!(
        parse_format_description("[offset_second]"),
        Ok(vec![FormatDescription::Component(
            Component::OffsetSecond {
                padding: Padding::Zero
            }
        )])
    );
    assert_eq!(
        parse_format_description("[ordinal]"),
        Ok(vec![FormatDescription::Component(Component::Ordinal {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[period]"),
        Ok(vec![FormatDescription::Component(Component::Period {
            is_uppercase: true
        })])
    );
    assert_eq!(
        parse_format_description("[second]"),
        Ok(vec![FormatDescription::Component(Component::Second {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[subsecond]"),
        Ok(vec![FormatDescription::Component(Component::Subsecond {
            digits: SubsecondDigits::OneOrMore
        })])
    );
    assert_eq!(
        parse_format_description("[weekday]"),
        Ok(vec![FormatDescription::Component(Component::Weekday {
            repr: WeekdayRepr::Long,
            one_indexed: true,
        })])
    );
    assert_eq!(
        parse_format_description("[week_number]"),
        Ok(vec![FormatDescription::Component(Component::WeekNumber {
            padding: Padding::Zero,
            repr: WeekNumberRepr::Iso
        })])
    );
    assert_eq!(
        parse_format_description("[year]"),
        Ok(vec![FormatDescription::Component(Component::Year {
            padding: Padding::Zero,
            repr: YearRepr::Full,
            iso_week_based: false,
            sign_is_mandatory: false
        })])
    );
}

#[test]
fn errors() {
    assert_eq!(
        parse_format_description("[ invalid ]"),
        Err(InvalidFormatDescription::InvalidComponentName {
            name: "invalid".to_owned(),
            index: 2
        })
    );
    assert_eq!(
        parse_format_description("["),
        Err(InvalidFormatDescription::UnclosedOpeningBracket { index: 0 })
    );
    assert_eq!(
        parse_format_description("[day sign:mandatory]"),
        Err(InvalidFormatDescription::InvalidModifier {
            value: "sign:mandatory".to_owned(),
            index: 5
        })
    );
}

#[test]
fn component_with_modifiers() {
    for (padding, padding_str) in iterator::padding() {
        assert_eq!(
            parse_format_description(&format!("[day {}]", padding_str)),
            Ok(vec![FormatDescription::Component(Component::Day {
                padding
            })])
        );
        assert_eq!(
            parse_format_description(&format!("[minute {}]", padding_str)),
            Ok(vec![FormatDescription::Component(Component::Minute {
                padding
            })])
        );
        assert_eq!(
            parse_format_description(&format!("[offset_minute {}]", padding_str)),
            Ok(vec![FormatDescription::Component(
                Component::OffsetMinute { padding }
            )])
        );
        assert_eq!(
            parse_format_description(&format!("[offset_second {}]", padding_str)),
            Ok(vec![FormatDescription::Component(
                Component::OffsetSecond { padding }
            )])
        );
        assert_eq!(
            parse_format_description(&format!("[ordinal {}]", padding_str)),
            Ok(vec![FormatDescription::Component(Component::Ordinal {
                padding
            })])
        );
        assert_eq!(
            parse_format_description(&format!("[second {}]", padding_str)),
            Ok(vec![FormatDescription::Component(Component::Second {
                padding
            })])
        );

        for (is_12_hour_clock, is_12_hour_clock_str) in iterator::hour_is_12_hour_clock() {
            assert_eq!(
                parse_format_description(&format!(
                    "[hour {} {}]",
                    padding_str, is_12_hour_clock_str
                )),
                Ok(vec![FormatDescription::Component(Component::Hour {
                    padding,
                    is_12_hour_clock
                })])
            );
        }
        for (repr, repr_str) in iterator::month_repr() {
            assert_eq!(
                parse_format_description(&format!("[month {} {}]", padding_str, repr_str)),
                Ok(vec![FormatDescription::Component(Component::Month {
                    padding,
                    repr
                })])
            );
        }
        for (is_uppercase, is_uppercase_str) in iterator::period_is_uppercase() {
            assert_eq!(
                parse_format_description(&format!("[period {}]", is_uppercase_str)),
                Ok(vec![FormatDescription::Component(Component::Period {
                    is_uppercase
                })])
            );
        }
        for (repr, repr_str) in iterator::week_number_repr() {
            assert_eq!(
                parse_format_description(&format!("[week_number {} {}]", padding_str, repr_str)),
                Ok(vec![FormatDescription::Component(Component::WeekNumber {
                    padding,
                    repr
                })])
            );
        }
        for (sign_is_mandatory, sign_is_mandatory_str) in iterator::sign_is_mandatory() {
            assert_eq!(
                parse_format_description(&format!(
                    "[offset_hour {} {}]",
                    padding_str, sign_is_mandatory_str
                )),
                Ok(vec![FormatDescription::Component(Component::OffsetHour {
                    sign_is_mandatory,
                    padding
                })])
            );

            for (repr, repr_str) in iterator::year_repr() {
                for (iso_week_based, iso_week_based_str) in iterator::year_is_iso_week_based() {
                    assert_eq!(
                        parse_format_description(&format!(
                            "[year {} {} {} {}]",
                            padding_str, repr_str, iso_week_based_str, sign_is_mandatory_str
                        )),
                        Ok(vec![FormatDescription::Component(Component::Year {
                            padding,
                            repr,
                            iso_week_based,
                            sign_is_mandatory
                        })])
                    );
                }
            }
        }
    }

    for (digits, digits_str) in iterator::subsecond_digits() {
        assert_eq!(
            parse_format_description(&format!("[subsecond {}]", digits_str)),
            Ok(vec![FormatDescription::Component(Component::Subsecond {
                digits
            })])
        );
    }

    for (repr, repr_str) in iterator::weekday_repr() {
        for (one_indexed, one_indexed_str) in iterator::weekday_is_one_indexed() {
            assert_eq!(
                parse_format_description(&format!("[weekday {} {} ]", repr_str, one_indexed_str)),
                Ok(vec![FormatDescription::Component(Component::Weekday {
                    repr,
                    one_indexed
                })])
            );
        }
    }
}

#[test]
fn error_display() {
    assert_eq!(
        InvalidFormatDescription::UnclosedOpeningBracket { index: 1 }.to_string(),
        "unclosed opening bracket at byte index 1"
    );
    assert_eq!(
        InvalidFormatDescription::InvalidComponentName {
            name: "foo".to_owned(),
            index: 2
        }
        .to_string(),
        "invalid component name `foo` at byte index 2"
    );
    assert_eq!(
        InvalidFormatDescription::InvalidModifier {
            value: "bar".to_owned(),
            index: 3
        }
        .to_string(),
        "invalid modifier `bar` at byte index 3"
    );
    assert_eq!(
        InvalidFormatDescription::MissingComponentName { index: 4 }.to_string(),
        "missing component name at byte index 4"
    );
}

#[test]
fn rfc_3339() {
    assert_eq!(
        parse_format_description(
            "[year]-[month repr:numerical]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour \
             sign:mandatory]:[offset_minute]"
        ),
        Ok(vec![
            FormatDescription::Component(Component::Year {
                padding: Padding::Zero,
                repr: YearRepr::Full,
                iso_week_based: false,
                sign_is_mandatory: false
            }),
            FormatDescription::Literal("-"),
            FormatDescription::Component(Component::Month {
                padding: Padding::Zero,
                repr: MonthRepr::Numerical
            }),
            FormatDescription::Literal("-"),
            FormatDescription::Component(Component::Day {
                padding: Padding::Zero
            }),
            FormatDescription::Literal("T"),
            FormatDescription::Component(Component::Hour {
                padding: Padding::Zero,
                is_12_hour_clock: false
            }),
            FormatDescription::Literal(":"),
            FormatDescription::Component(Component::Minute {
                padding: Padding::Zero
            }),
            FormatDescription::Literal(":"),
            FormatDescription::Component(Component::Second {
                padding: Padding::Zero
            }),
            FormatDescription::Literal("."),
            FormatDescription::Component(Component::Subsecond {
                digits: SubsecondDigits::OneOrMore
            }),
            FormatDescription::Component(Component::OffsetHour {
                padding: Padding::Zero,
                sign_is_mandatory: true
            }),
            FormatDescription::Literal(":"),
            FormatDescription::Component(Component::OffsetMinute {
                padding: Padding::Zero
            })
        ])
    );
}
