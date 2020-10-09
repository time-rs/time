#![cfg(feature = "alloc")]

mod iterator;

use time_formatting::{
    format_description::{
        error::InvalidFormatDescription,
        modifier::{MonthRepr, Padding, SubsecondDigits, WeekNumberRepr, WeekdayRepr, YearRepr},
        Component, Description,
    },
    parse_format_description,
};

#[test]
fn empty() {
    assert_eq!(parse_format_description(""), Ok(vec![]));
}

#[test]
fn only_literal() {
    assert_eq!(
        parse_format_description("foo bar"),
        Ok(vec![Description::Literal("foo bar")])
    );
    assert_eq!(
        parse_format_description("  leading spaces"),
        Ok(vec![Description::Literal("  leading spaces")])
    );
    assert_eq!(
        parse_format_description("trailing spaces  "),
        Ok(vec![Description::Literal("trailing spaces  ")])
    );
    assert_eq!(
        parse_format_description("[["),
        Ok(vec![Description::Literal("[")])
    );
    assert_eq!(
        parse_format_description("foo[[bar"),
        Ok(vec![
            Description::Literal("foo"),
            Description::Literal("["),
            Description::Literal("bar")
        ])
    );
}

#[test]
fn simple_component() {
    assert_eq!(
        parse_format_description("[day]"),
        Ok(vec![Description::Component(Component::Day {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[hour]"),
        Ok(vec![Description::Component(Component::Hour {
            padding: Padding::Zero,
            is_12_hour_clock: false
        })])
    );
    assert_eq!(
        parse_format_description("[minute]"),
        Ok(vec![Description::Component(Component::Minute {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[month]"),
        Ok(vec![Description::Component(Component::Month {
            padding: Padding::Zero,
            repr: MonthRepr::Long
        })])
    );
    assert_eq!(
        parse_format_description("[offset_hour]"),
        Ok(vec![Description::Component(Component::OffsetHour {
            sign_is_mandatory: false,
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[offset_minute]"),
        Ok(vec![Description::Component(Component::OffsetMinute {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[offset_second]"),
        Ok(vec![Description::Component(Component::OffsetSecond {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[ordinal]"),
        Ok(vec![Description::Component(Component::Ordinal {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[period]"),
        Ok(vec![Description::Component(Component::Period {
            is_uppercase: true
        })])
    );
    assert_eq!(
        parse_format_description("[second]"),
        Ok(vec![Description::Component(Component::Second {
            padding: Padding::Zero
        })])
    );
    assert_eq!(
        parse_format_description("[subsecond]"),
        Ok(vec![Description::Component(Component::Subsecond {
            digits: SubsecondDigits::OneOrMore
        })])
    );
    assert_eq!(
        parse_format_description("[weekday]"),
        Ok(vec![Description::Component(Component::Weekday {
            repr: WeekdayRepr::Long,
            one_indexed: true,
        })])
    );
    assert_eq!(
        parse_format_description("[week_number]"),
        Ok(vec![Description::Component(Component::WeekNumber {
            padding: Padding::Zero,
            repr: WeekNumberRepr::Iso
        })])
    );
    assert_eq!(
        parse_format_description("[year]"),
        Ok(vec![Description::Component(Component::Year {
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
            name: "invalid",
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
            value: "sign:mandatory",
            index: 5
        })
    );
}

#[test]
fn component_with_modifiers() {
    for (padding, padding_str) in iterator::padding() {
        assert_eq!(
            parse_format_description(&format!("[day {}]", padding_str)),
            Ok(vec![Description::Component(Component::Day { padding })])
        );
        assert_eq!(
            parse_format_description(&format!("[minute {}]", padding_str)),
            Ok(vec![Description::Component(Component::Minute { padding })])
        );
        assert_eq!(
            parse_format_description(&format!("[offset_minute {}]", padding_str)),
            Ok(vec![Description::Component(Component::OffsetMinute {
                padding
            })])
        );
        assert_eq!(
            parse_format_description(&format!("[offset_second {}]", padding_str)),
            Ok(vec![Description::Component(Component::OffsetSecond {
                padding
            })])
        );
        assert_eq!(
            parse_format_description(&format!("[ordinal {}]", padding_str)),
            Ok(vec![Description::Component(Component::Ordinal { padding })])
        );
        assert_eq!(
            parse_format_description(&format!("[second {}]", padding_str)),
            Ok(vec![Description::Component(Component::Second { padding })])
        );

        for (is_12_hour_clock, is_12_hour_clock_str) in iterator::hour_is_12_hour_clock() {
            assert_eq!(
                parse_format_description(&format!(
                    "[hour {} {}]",
                    padding_str, is_12_hour_clock_str
                )),
                Ok(vec![Description::Component(Component::Hour {
                    padding,
                    is_12_hour_clock
                })])
            );
        }
        for (repr, repr_str) in iterator::month_repr() {
            assert_eq!(
                parse_format_description(&format!("[month {} {}]", padding_str, repr_str)),
                Ok(vec![Description::Component(Component::Month {
                    padding,
                    repr
                })])
            );
        }
        for (is_uppercase, is_uppercase_str) in iterator::period_is_uppercase() {
            assert_eq!(
                parse_format_description(&format!("[period {}]", is_uppercase_str)),
                Ok(vec![Description::Component(Component::Period {
                    is_uppercase
                })])
            );
        }
        for (repr, repr_str) in iterator::week_number_repr() {
            assert_eq!(
                parse_format_description(&format!("[week_number {} {}]", padding_str, repr_str)),
                Ok(vec![Description::Component(Component::WeekNumber {
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
                Ok(vec![Description::Component(Component::OffsetHour {
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
                        Ok(vec![Description::Component(Component::Year {
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
            Ok(vec![Description::Component(Component::Subsecond {
                digits
            })])
        );
    }

    for (repr, repr_str) in iterator::weekday_repr() {
        for (one_indexed, one_indexed_str) in iterator::weekday_is_one_indexed() {
            assert_eq!(
                parse_format_description(&format!("[weekday {} {} ]", repr_str, one_indexed_str)),
                Ok(vec![Description::Component(Component::Weekday {
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
            name: "foo",
            index: 2
        }
        .to_string(),
        "invalid component name `foo` at byte index 2"
    );
    assert_eq!(
        InvalidFormatDescription::InvalidModifier {
            value: "bar",
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
            Description::Component(Component::Year {
                padding: Padding::Zero,
                repr: YearRepr::Full,
                iso_week_based: false,
                sign_is_mandatory: false
            }),
            Description::Literal("-"),
            Description::Component(Component::Month {
                padding: Padding::Zero,
                repr: MonthRepr::Numerical
            }),
            Description::Literal("-"),
            Description::Component(Component::Day {
                padding: Padding::Zero
            }),
            Description::Literal("T"),
            Description::Component(Component::Hour {
                padding: Padding::Zero,
                is_12_hour_clock: false
            }),
            Description::Literal(":"),
            Description::Component(Component::Minute {
                padding: Padding::Zero
            }),
            Description::Literal(":"),
            Description::Component(Component::Second {
                padding: Padding::Zero
            }),
            Description::Literal("."),
            Description::Component(Component::Subsecond {
                digits: SubsecondDigits::OneOrMore
            }),
            Description::Component(Component::OffsetHour {
                padding: Padding::Zero,
                sign_is_mandatory: true
            }),
            Description::Literal(":"),
            Description::Component(Component::OffsetMinute {
                padding: Padding::Zero
            })
        ])
    );
}
