#![cfg(feature = "alloc")]

mod iterator {
    use time::format_description::modifier::{
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

use time::format_description::{
    modifier::{self, MonthRepr, Padding, SubsecondDigits, WeekNumberRepr, WeekdayRepr, YearRepr},
    Component, FormatDescription, InvalidFormatDescription,
};

macro_rules! owned {
    ($($x:tt)*) => {
        FormatDescription::OwnedCompound(vec![$($x)*])
    };
}

#[test]
fn empty() {
    assert_eq!(FormatDescription::parse(""), Ok(owned![]));
}

#[test]
fn only_literal() {
    assert_eq!(
        FormatDescription::parse("foo bar"),
        Ok(owned![FormatDescription::Literal("foo bar")])
    );
    assert_eq!(
        FormatDescription::parse("  leading spaces"),
        Ok(owned![FormatDescription::Literal("  leading spaces")])
    );
    assert_eq!(
        FormatDescription::parse("trailing spaces  "),
        Ok(owned![FormatDescription::Literal("trailing spaces  ")])
    );
    assert_eq!(
        FormatDescription::parse("[["),
        Ok(owned![FormatDescription::Literal("[")])
    );
    assert_eq!(
        FormatDescription::parse("foo[[bar"),
        Ok(owned![
            FormatDescription::Literal("foo"),
            FormatDescription::Literal("["),
            FormatDescription::Literal("bar")
        ])
    );
}

#[test]
fn simple_component() {
    assert_eq!(
        FormatDescription::parse("[day]"),
        Ok(owned![FormatDescription::Component(Component::Day(
            modifier::Day {
                padding: Padding::Zero
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[hour]"),
        Ok(owned![FormatDescription::Component(Component::Hour(
            modifier::Hour {
                padding: Padding::Zero,
                is_12_hour_clock: false
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[minute]"),
        Ok(owned![FormatDescription::Component(Component::Minute(
            modifier::Minute {
                padding: Padding::Zero
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[month]"),
        Ok(owned![FormatDescription::Component(Component::Month(
            modifier::Month {
                padding: Padding::Zero,
                repr: MonthRepr::Long
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[offset_hour]"),
        Ok(owned![FormatDescription::Component(Component::OffsetHour(
            modifier::OffsetHour {
                sign_is_mandatory: false,
                padding: Padding::Zero
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[offset_minute]"),
        Ok(owned![FormatDescription::Component(
            Component::OffsetMinute(modifier::OffsetMinute {
                padding: Padding::Zero
            })
        )])
    );
    assert_eq!(
        FormatDescription::parse("[offset_second]"),
        Ok(owned![FormatDescription::Component(
            Component::OffsetSecond(modifier::OffsetSecond {
                padding: Padding::Zero
            })
        )])
    );
    assert_eq!(
        FormatDescription::parse("[ordinal]"),
        Ok(owned![FormatDescription::Component(Component::Ordinal(
            modifier::Ordinal {
                padding: Padding::Zero
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[period]"),
        Ok(owned![FormatDescription::Component(Component::Period(
            modifier::Period { is_uppercase: true }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[second]"),
        Ok(owned![FormatDescription::Component(Component::Second(
            modifier::Second {
                padding: Padding::Zero
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[subsecond]"),
        Ok(owned![FormatDescription::Component(Component::Subsecond(
            modifier::Subsecond {
                digits: SubsecondDigits::OneOrMore
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[weekday]"),
        Ok(owned![FormatDescription::Component(Component::Weekday(
            modifier::Weekday {
                repr: WeekdayRepr::Long,
                one_indexed: true,
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[week_number]"),
        Ok(owned![FormatDescription::Component(Component::WeekNumber(
            modifier::WeekNumber {
                padding: Padding::Zero,
                repr: WeekNumberRepr::Iso
            }
        ))])
    );
    assert_eq!(
        FormatDescription::parse("[year]"),
        Ok(owned![FormatDescription::Component(Component::Year(
            modifier::Year {
                padding: Padding::Zero,
                repr: YearRepr::Full,
                iso_week_based: false,
                sign_is_mandatory: false
            }
        ))])
    );
}

#[test]
fn errors() {
    assert_eq!(
        FormatDescription::parse("[ invalid ]"),
        Err(InvalidFormatDescription::InvalidComponentName {
            name: "invalid".to_owned(),
            index: 2
        })
    );
    assert_eq!(
        FormatDescription::parse("["),
        Err(InvalidFormatDescription::UnclosedOpeningBracket { index: 0 })
    );
    assert_eq!(
        FormatDescription::parse("[day sign:mandatory]"),
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
            FormatDescription::parse(&format!("[day {}]", padding_str)),
            Ok(owned![FormatDescription::Component(Component::Day(
                modifier::Day { padding }
            ))])
        );
        assert_eq!(
            FormatDescription::parse(&format!("[minute {}]", padding_str)),
            Ok(owned![FormatDescription::Component(Component::Minute(
                modifier::Minute { padding }
            ))])
        );
        assert_eq!(
            FormatDescription::parse(&format!("[offset_minute {}]", padding_str)),
            Ok(owned![FormatDescription::Component(
                Component::OffsetMinute(modifier::OffsetMinute { padding })
            )])
        );
        assert_eq!(
            FormatDescription::parse(&format!("[offset_second {}]", padding_str)),
            Ok(owned![FormatDescription::Component(
                Component::OffsetSecond(modifier::OffsetSecond { padding })
            )])
        );
        assert_eq!(
            FormatDescription::parse(&format!("[ordinal {}]", padding_str)),
            Ok(owned![FormatDescription::Component(Component::Ordinal(
                modifier::Ordinal { padding }
            ))])
        );
        assert_eq!(
            FormatDescription::parse(&format!("[second {}]", padding_str)),
            Ok(owned![FormatDescription::Component(Component::Second(
                modifier::Second { padding }
            ))])
        );

        for (is_12_hour_clock, is_12_hour_clock_str) in iterator::hour_is_12_hour_clock() {
            assert_eq!(
                FormatDescription::parse(&format!(
                    "[hour {} {}]",
                    padding_str, is_12_hour_clock_str
                )),
                Ok(owned![FormatDescription::Component(Component::Hour(
                    modifier::Hour {
                        padding,
                        is_12_hour_clock
                    }
                ))])
            );
        }
        for (repr, repr_str) in iterator::month_repr() {
            assert_eq!(
                FormatDescription::parse(&format!("[month {} {}]", padding_str, repr_str)),
                Ok(owned![FormatDescription::Component(Component::Month(
                    modifier::Month { padding, repr }
                ))])
            );
        }
        for (is_uppercase, is_uppercase_str) in iterator::period_is_uppercase() {
            assert_eq!(
                FormatDescription::parse(&format!("[period {}]", is_uppercase_str)),
                Ok(owned![FormatDescription::Component(Component::Period(
                    modifier::Period { is_uppercase }
                ))])
            );
        }
        for (repr, repr_str) in iterator::week_number_repr() {
            assert_eq!(
                FormatDescription::parse(&format!("[week_number {} {}]", padding_str, repr_str)),
                Ok(owned![FormatDescription::Component(Component::WeekNumber(
                    modifier::WeekNumber { padding, repr }
                ))])
            );
        }
        for (sign_is_mandatory, sign_is_mandatory_str) in iterator::sign_is_mandatory() {
            assert_eq!(
                FormatDescription::parse(&format!(
                    "[offset_hour {} {}]",
                    padding_str, sign_is_mandatory_str
                )),
                Ok(owned![FormatDescription::Component(Component::OffsetHour(
                    modifier::OffsetHour {
                        sign_is_mandatory,
                        padding
                    }
                ))])
            );

            for (repr, repr_str) in iterator::year_repr() {
                for (iso_week_based, iso_week_based_str) in iterator::year_is_iso_week_based() {
                    assert_eq!(
                        FormatDescription::parse(&format!(
                            "[year {} {} {} {}]",
                            padding_str, repr_str, iso_week_based_str, sign_is_mandatory_str
                        )),
                        Ok(owned![FormatDescription::Component(Component::Year(
                            modifier::Year {
                                padding,
                                repr,
                                iso_week_based,
                                sign_is_mandatory
                            }
                        ))])
                    );
                }
            }
        }
    }

    for (digits, digits_str) in iterator::subsecond_digits() {
        assert_eq!(
            FormatDescription::parse(&format!("[subsecond {}]", digits_str)),
            Ok(owned![FormatDescription::Component(Component::Subsecond(
                modifier::Subsecond { digits }
            ))])
        );
    }

    for (repr, repr_str) in iterator::weekday_repr() {
        for (one_indexed, one_indexed_str) in iterator::weekday_is_one_indexed() {
            assert_eq!(
                FormatDescription::parse(&format!("[weekday {} {} ]", repr_str, one_indexed_str)),
                Ok(owned![FormatDescription::Component(Component::Weekday(
                    modifier::Weekday { repr, one_indexed }
                ))])
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
        FormatDescription::parse(
            "[year]-[month repr:numerical]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour \
             sign:mandatory]:[offset_minute]"
        ),
        Ok(owned![
            FormatDescription::Component(Component::Year(modifier::Year {
                padding: Padding::Zero,
                repr: YearRepr::Full,
                iso_week_based: false,
                sign_is_mandatory: false
            })),
            FormatDescription::Literal("-"),
            FormatDescription::Component(Component::Month(modifier::Month {
                padding: Padding::Zero,
                repr: MonthRepr::Numerical
            })),
            FormatDescription::Literal("-"),
            FormatDescription::Component(Component::Day(modifier::Day {
                padding: Padding::Zero
            })),
            FormatDescription::Literal("T"),
            FormatDescription::Component(Component::Hour(modifier::Hour {
                padding: Padding::Zero,
                is_12_hour_clock: false
            })),
            FormatDescription::Literal(":"),
            FormatDescription::Component(Component::Minute(modifier::Minute {
                padding: Padding::Zero
            })),
            FormatDescription::Literal(":"),
            FormatDescription::Component(Component::Second(modifier::Second {
                padding: Padding::Zero
            })),
            FormatDescription::Literal("."),
            FormatDescription::Component(Component::Subsecond(modifier::Subsecond {
                digits: SubsecondDigits::OneOrMore
            })),
            FormatDescription::Component(Component::OffsetHour(modifier::OffsetHour {
                padding: Padding::Zero,
                sign_is_mandatory: true
            })),
            FormatDescription::Literal(":"),
            FormatDescription::Component(Component::OffsetMinute(modifier::OffsetMinute {
                padding: Padding::Zero
            }))
        ])
    );
}
