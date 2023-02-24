use core::num::NonZeroU16;

use time::format_description::modifier::*;
use time::format_description::{Component, FormatItem};
use time::macros::{date, format_description, time};
use time::{Date, Time};

#[test]
fn nontrivial_string() {
    assert!(format_description!(r"").is_empty());
    assert!(format_description!(r###""###).is_empty());
    assert!(format_description!(b"").is_empty());
    assert!(format_description!(br"").is_empty());
    assert!(format_description!(br###""###).is_empty());
    #[rustfmt::skip]
    assert_eq!(
        format_description!("foo\
        bar\n\r\t\\\"\'\0\x20\x4E\x4e\u{20}\u{4E}\u{4_e}"),
        &[FormatItem::Literal(b"foobar\n\r\t\\\"'\0 NN NN")]
    );
    #[rustfmt::skip]
    assert_eq!(
        format_description!(b"foo\
        bar\n\r\t\\\"\'\0\x20\x4E\x4e"),
        &[FormatItem::Literal(b"foobar\n\r\t\\\"'\0 NN")]
    );
}

#[test]
fn format_description_version() {
    assert_eq!(
        format_description!(version = 1, "[["),
        &[FormatItem::Literal(b"[")]
    );
    assert_eq!(
        format_description!(version = 1, r"\\"),
        &[FormatItem::Literal(br"\\")]
    );
    assert_eq!(
        format_description!(version = 2, r"\\"),
        &[FormatItem::Literal(br"\")]
    );
}

#[test]
fn nested_v1() {
    assert_eq!(
        format_description!(version = 1, "[optional [[[]]"),
        &[FormatItem::Optional(&FormatItem::Literal(b"["))]
    );
    assert_eq!(
        format_description!(version = 1, "[optional [ [[ ]]"),
        &[FormatItem::Optional(&FormatItem::Compound(&[
            FormatItem::Literal(b" "),
            FormatItem::Literal(b"["),
            FormatItem::Literal(b" "),
        ]))]
    );
    assert_eq!(
        format_description!(version = 1, "[first [a][[[]]"),
        &[FormatItem::First(&[
            FormatItem::Literal(b"a"),
            FormatItem::Literal(b"[")
        ])]
    );
}

#[test]
fn optional() {
    assert_eq!(
        format_description!(version = 2, "[optional [:[year]]]"),
        &[FormatItem::Optional(&FormatItem::Compound(&[
            FormatItem::Literal(b":"),
            FormatItem::Component(Component::Year(Default::default()))
        ]))]
    );
    assert_eq!(
        format_description!(version = 2, "[optional [[year]]]"),
        &[FormatItem::Optional(&FormatItem::Component(
            Component::Year(Default::default())
        ))]
    );
    assert_eq!(
        format_description!(version = 2, r"[optional [\[]]"),
        &[FormatItem::Optional(&FormatItem::Literal(b"["))]
    );
    assert_eq!(
        format_description!(version = 2, r"[optional [ \[ ]]"),
        &[FormatItem::Optional(&FormatItem::Compound(&[
            FormatItem::Literal(b" "),
            FormatItem::Literal(b"["),
            FormatItem::Literal(b" "),
        ]))]
    );
}

#[test]
fn first() {
    assert_eq!(
        format_description!(version = 2, "[first [a]]"),
        &[FormatItem::First(&[FormatItem::Literal(b"a")])]
    );
    assert_eq!(
        format_description!(version = 2, "[first [a] [b]]"),
        &[FormatItem::First(&[
            FormatItem::Literal(b"a"),
            FormatItem::Literal(b"b"),
        ])]
    );
    assert_eq!(
        format_description!(version = 2, "[first [a][b]]"),
        &[FormatItem::First(&[
            FormatItem::Literal(b"a"),
            FormatItem::Literal(b"b"),
        ])]
    );
    assert_eq!(
        format_description!(version = 2, r"[first [a][\[]]"),
        &[FormatItem::First(&[
            FormatItem::Literal(b"a"),
            FormatItem::Literal(b"["),
        ])]
    );
    assert_eq!(
        format_description!(version = 2, r"[first [a][\[\[]]"),
        &[FormatItem::First(&[
            FormatItem::Literal(b"a"),
            FormatItem::Compound(&[FormatItem::Literal(b"["), FormatItem::Literal(b"["),])
        ])]
    );
    assert_eq!(
        format_description!(
            version = 2,
            "[first [[period case:upper]] [[period case:lower]] ]"
        ),
        &[FormatItem::First(&[
            FormatItem::Component(Component::Period(modifier!(Period {
                is_uppercase: true,
                case_sensitive: true,
            }))),
            FormatItem::Component(Component::Period(modifier!(Period {
                is_uppercase: false,
                case_sensitive: true,
            }))),
        ])]
    );
}

#[test]
fn backslash_escape() {
    assert_eq!(
        format_description!(version = 2, r"[optional [\]]]"),
        &[FormatItem::Optional(&FormatItem::Literal(b"]"))]
    );
    assert_eq!(
        format_description!(version = 2, r"[optional [\[]]"),
        &[FormatItem::Optional(&FormatItem::Literal(b"["))]
    );
    assert_eq!(
        format_description!(version = 2, r"[optional [\\]]"),
        &[FormatItem::Optional(&FormatItem::Literal(br"\"))]
    );
    assert_eq!(
        format_description!(version = 2, r"\\"),
        &[FormatItem::Literal(br"\")]
    );
    assert_eq!(
        format_description!(version = 2, r"\["),
        &[FormatItem::Literal(br"[")]
    );
    assert_eq!(
        format_description!(version = 2, r"\]"),
        &[FormatItem::Literal(br"]")]
    );
    assert_eq!(
        format_description!(version = 2, r"foo\\"),
        &[FormatItem::Literal(b"foo"), FormatItem::Literal(br"\"),]
    );
    assert_eq!(
        format_description!(version = 2, r"\\"),
        &[FormatItem::Literal(br"\")]
    );
    assert_eq!(
        format_description!(version = 2, r"\["),
        &[FormatItem::Literal(br"[")]
    );
    assert_eq!(
        format_description!(version = 2, r"\]"),
        &[FormatItem::Literal(br"]")]
    );
    assert_eq!(
        format_description!(version = 2, r"foo\\"),
        &[FormatItem::Literal(b"foo"), FormatItem::Literal(br"\"),]
    );
}

#[test]
fn format_description_coverage() {
    assert_eq!(
        format_description!("[day padding:space][day padding:zero][day padding:none]"),
        &[
            FormatItem::Component(Component::Day(modifier!(Day {
                padding: Padding::Space,
            }))),
            FormatItem::Component(Component::Day(modifier!(Day {
                padding: Padding::Zero,
            }))),
            FormatItem::Component(Component::Day(modifier!(Day {
                padding: Padding::None,
            })))
        ]
    );
    assert_eq!(
        format_description!(
            "[offset_minute padding:space][offset_minute padding:zero][offset_minute padding:none]"
        ),
        &[
            FormatItem::Component(Component::OffsetMinute(modifier!(OffsetMinute {
                padding: Padding::Space,
            }))),
            FormatItem::Component(Component::OffsetMinute(modifier!(OffsetMinute {
                padding: Padding::Zero,
            }))),
            FormatItem::Component(Component::OffsetMinute(modifier!(OffsetMinute {
                padding: Padding::None,
            })))
        ]
    );
    assert_eq!(
        format_description!(
            "[offset_second padding:space][offset_second padding:zero][offset_second padding:none]"
        ),
        &[
            FormatItem::Component(Component::OffsetSecond(modifier!(OffsetSecond {
                padding: Padding::Space,
            }))),
            FormatItem::Component(Component::OffsetSecond(modifier!(OffsetSecond {
                padding: Padding::Zero,
            }))),
            FormatItem::Component(Component::OffsetSecond(modifier!(OffsetSecond {
                padding: Padding::None,
            }))),
        ]
    );
    assert_eq!(
        format_description!("[ordinal padding:space][ordinal padding:zero][ordinal padding:none]"),
        &[
            FormatItem::Component(Component::Ordinal(modifier!(Ordinal {
                padding: Padding::Space,
            }))),
            FormatItem::Component(Component::Ordinal(modifier!(Ordinal {
                padding: Padding::Zero,
            }))),
            FormatItem::Component(Component::Ordinal(modifier!(Ordinal {
                padding: Padding::None,
            }))),
        ]
    );
    assert_eq!(
        format_description!("[month repr:numerical]"),
        &[FormatItem::Component(Component::Month(modifier!(Month {
            repr: MonthRepr::Numerical,
            padding: Padding::Zero,
        })))]
    );
    assert_eq!(
        format_description!("[week_number repr:iso ]"),
        &[FormatItem::Component(Component::WeekNumber(modifier!(
            WeekNumber {
                padding: Padding::Zero,
                repr: WeekNumberRepr::Iso,
            }
        )))]
    );
    assert_eq!(
        format_description!("[weekday repr:long one_indexed:true]"),
        &[FormatItem::Component(Component::Weekday(modifier!(
            Weekday {
                repr: WeekdayRepr::Long,
                one_indexed: true,
            }
        )))]
    );
    assert_eq!(
        format_description!("[year repr:full base:calendar]"),
        &[FormatItem::Component(Component::Year(modifier!(Year {
            repr: YearRepr::Full,
            iso_week_based: false,
            padding: Padding::Zero,
            sign_is_mandatory: false,
        })))]
    );
    assert_eq!(
        format_description!("[[ "),
        &[FormatItem::Literal(b"["), FormatItem::Literal(b" ")]
    );
    assert_eq!(
        format_description!("[ignore count:2]"),
        &[FormatItem::Component(Component::Ignore(Ignore::count(
            NonZeroU16::new(2).unwrap()
        )))]
    );
    assert_eq!(
        format_description!("[unix_timestamp precision:nanosecond sign:mandatory]"),
        &[FormatItem::Component(Component::UnixTimestamp(modifier!(
            UnixTimestamp {
                precision: UnixTimestampPrecision::Nanosecond,
                sign_is_mandatory: true,
            }
        )))]
    );
}

#[test]
fn date_coverage() {
    assert_eq!(Ok(date!(2000 - 001)), Date::from_ordinal_date(2000, 1));
    assert_eq!(Ok(date!(2019-W 01-1)), Date::from_ordinal_date(2018, 365));
    assert_eq!(Ok(date!(2021-W 52-6)), Date::from_ordinal_date(2022, 1));
    assert_eq!(Ok(date!(2021-W 34-5)), Date::from_ordinal_date(2021, 239));
}

#[test]
fn time_coverage() {
    assert_eq!(time!(12 AM), Time::MIDNIGHT);
    assert_eq!(Ok(time!(12 PM)), Time::from_hms(12, 0, 0));
}
