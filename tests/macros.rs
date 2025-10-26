use core::num::NonZero;

use rstest::rstest;
use time::format_description::modifier::*;
use time::format_description::{BorrowedFormatItem, Component};
use time::macros::{date, format_description, time};
use time::{Date, Time};

#[rstest]
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
        &[BorrowedFormatItem::Literal(b"foobar\n\r\t\\\"'\0 NN NN")]
    );
    #[rustfmt::skip]
    assert_eq!(
        format_description!(b"foo\
        bar\n\r\t\\\"\'\0\x20\x4E\x4e"),
        &[BorrowedFormatItem::Literal(b"foobar\n\r\t\\\"'\0 NN")]
    );
}

#[rstest]
fn format_description_version() {
    assert_eq!(
        format_description!(version = 1, "[["),
        &[BorrowedFormatItem::Literal(b"[")]
    );
    assert_eq!(
        format_description!(version = 1, r"\\"),
        &[BorrowedFormatItem::Literal(br"\\")]
    );
    assert_eq!(
        format_description!(version = 2, r"\\"),
        &[BorrowedFormatItem::Literal(br"\")]
    );
}

#[rstest]
fn nested_v1() {
    assert_eq!(
        format_description!(version = 1, "[optional [[[]]"),
        &[BorrowedFormatItem::Optional(&BorrowedFormatItem::Literal(
            b"["
        ))]
    );
    assert_eq!(
        format_description!(version = 1, "[optional [ [[ ]]"),
        &[BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(
            &[
                BorrowedFormatItem::Literal(b" "),
                BorrowedFormatItem::Literal(b"["),
                BorrowedFormatItem::Literal(b" "),
            ]
        ))]
    );
    assert_eq!(
        format_description!(version = 1, "[first [a][[[]]"),
        &[BorrowedFormatItem::First(&[
            BorrowedFormatItem::Literal(b"a"),
            BorrowedFormatItem::Literal(b"[")
        ])]
    );
}

#[rstest]
fn optional() {
    assert_eq!(
        format_description!(version = 2, "[optional [:[year]]]"),
        &[BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(
            &[
                BorrowedFormatItem::Literal(b":"),
                BorrowedFormatItem::Component(Component::Year(Default::default()))
            ]
        ))]
    );
    assert_eq!(
        format_description!(version = 2, "[optional [[year]]]"),
        &[BorrowedFormatItem::Optional(
            &BorrowedFormatItem::Component(Component::Year(Default::default()))
        )]
    );
    assert_eq!(
        format_description!(version = 2, r"[optional [\[]]"),
        &[BorrowedFormatItem::Optional(&BorrowedFormatItem::Literal(
            b"["
        ))]
    );
    assert_eq!(
        format_description!(version = 2, r"[optional [ \[ ]]"),
        &[BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(
            &[
                BorrowedFormatItem::Literal(b" "),
                BorrowedFormatItem::Literal(b"["),
                BorrowedFormatItem::Literal(b" "),
            ]
        ))]
    );
}

#[rstest]
fn first() {
    assert_eq!(
        format_description!(version = 2, "[first [a]]"),
        &[BorrowedFormatItem::First(&[BorrowedFormatItem::Literal(
            b"a"
        )])]
    );
    assert_eq!(
        format_description!(version = 2, "[first [a] [b]]"),
        &[BorrowedFormatItem::First(&[
            BorrowedFormatItem::Literal(b"a"),
            BorrowedFormatItem::Literal(b"b"),
        ])]
    );
    assert_eq!(
        format_description!(version = 2, "[first [a][b]]"),
        &[BorrowedFormatItem::First(&[
            BorrowedFormatItem::Literal(b"a"),
            BorrowedFormatItem::Literal(b"b"),
        ])]
    );
    assert_eq!(
        format_description!(version = 2, r"[first [a][\[]]"),
        &[BorrowedFormatItem::First(&[
            BorrowedFormatItem::Literal(b"a"),
            BorrowedFormatItem::Literal(b"["),
        ])]
    );
    assert_eq!(
        format_description!(version = 2, r"[first [a][\[\[]]"),
        &[BorrowedFormatItem::First(&[
            BorrowedFormatItem::Literal(b"a"),
            BorrowedFormatItem::Compound(&[
                BorrowedFormatItem::Literal(b"["),
                BorrowedFormatItem::Literal(b"["),
            ])
        ])]
    );
    assert_eq!(
        format_description!(
            version = 2,
            "[first [[period case:upper]] [[period case:lower]] ]"
        ),
        &[BorrowedFormatItem::First(&[
            BorrowedFormatItem::Component(Component::Period(modifier!(Period {
                is_uppercase: true,
                case_sensitive: true,
            }))),
            BorrowedFormatItem::Component(Component::Period(modifier!(Period {
                is_uppercase: false,
                case_sensitive: true,
            }))),
        ])]
    );
}

#[rstest]
fn backslash_escape() {
    assert_eq!(
        format_description!(version = 2, r"[optional [\]]]"),
        &[BorrowedFormatItem::Optional(&BorrowedFormatItem::Literal(
            b"]"
        ))]
    );
    assert_eq!(
        format_description!(version = 2, r"[optional [\[]]"),
        &[BorrowedFormatItem::Optional(&BorrowedFormatItem::Literal(
            b"["
        ))]
    );
    assert_eq!(
        format_description!(version = 2, r"[optional [\\]]"),
        &[BorrowedFormatItem::Optional(&BorrowedFormatItem::Literal(
            br"\"
        ))]
    );
    assert_eq!(
        format_description!(version = 2, r"\\"),
        &[BorrowedFormatItem::Literal(br"\")]
    );
    assert_eq!(
        format_description!(version = 2, r"\["),
        &[BorrowedFormatItem::Literal(br"[")]
    );
    assert_eq!(
        format_description!(version = 2, r"\]"),
        &[BorrowedFormatItem::Literal(br"]")]
    );
    assert_eq!(
        format_description!(version = 2, r"foo\\"),
        &[
            BorrowedFormatItem::Literal(b"foo"),
            BorrowedFormatItem::Literal(br"\"),
        ]
    );
    assert_eq!(
        format_description!(version = 2, r"\\"),
        &[BorrowedFormatItem::Literal(br"\")]
    );
    assert_eq!(
        format_description!(version = 2, r"\["),
        &[BorrowedFormatItem::Literal(br"[")]
    );
    assert_eq!(
        format_description!(version = 2, r"\]"),
        &[BorrowedFormatItem::Literal(br"]")]
    );
    assert_eq!(
        format_description!(version = 2, r"foo\\"),
        &[
            BorrowedFormatItem::Literal(b"foo"),
            BorrowedFormatItem::Literal(br"\"),
        ]
    );
}

#[rstest]
fn format_description_coverage() {
    assert_eq!(
        format_description!("[day padding:space][day padding:zero][day padding:none]"),
        &[
            BorrowedFormatItem::Component(Component::Day(modifier!(Day {
                padding: Padding::Space,
            }))),
            BorrowedFormatItem::Component(Component::Day(modifier!(Day {
                padding: Padding::Zero,
            }))),
            BorrowedFormatItem::Component(Component::Day(modifier!(Day {
                padding: Padding::None,
            })))
        ]
    );
    assert_eq!(
        format_description!(
            "[offset_minute padding:space][offset_minute padding:zero][offset_minute padding:none]"
        ),
        &[
            BorrowedFormatItem::Component(Component::OffsetMinute(modifier!(OffsetMinute {
                padding: Padding::Space,
            }))),
            BorrowedFormatItem::Component(Component::OffsetMinute(modifier!(OffsetMinute {
                padding: Padding::Zero,
            }))),
            BorrowedFormatItem::Component(Component::OffsetMinute(modifier!(OffsetMinute {
                padding: Padding::None,
            })))
        ]
    );
    assert_eq!(
        format_description!(
            "[offset_second padding:space][offset_second padding:zero][offset_second padding:none]"
        ),
        &[
            BorrowedFormatItem::Component(Component::OffsetSecond(modifier!(OffsetSecond {
                padding: Padding::Space,
            }))),
            BorrowedFormatItem::Component(Component::OffsetSecond(modifier!(OffsetSecond {
                padding: Padding::Zero,
            }))),
            BorrowedFormatItem::Component(Component::OffsetSecond(modifier!(OffsetSecond {
                padding: Padding::None,
            }))),
        ]
    );
    assert_eq!(
        format_description!("[ordinal padding:space][ordinal padding:zero][ordinal padding:none]"),
        &[
            BorrowedFormatItem::Component(Component::Ordinal(modifier!(Ordinal {
                padding: Padding::Space,
            }))),
            BorrowedFormatItem::Component(Component::Ordinal(modifier!(Ordinal {
                padding: Padding::Zero,
            }))),
            BorrowedFormatItem::Component(Component::Ordinal(modifier!(Ordinal {
                padding: Padding::None,
            }))),
        ]
    );
    assert_eq!(
        format_description!("[month repr:numerical]"),
        &[BorrowedFormatItem::Component(Component::Month(modifier!(
            Month {
                repr: MonthRepr::Numerical,
                padding: Padding::Zero,
            }
        )))]
    );
    assert_eq!(
        format_description!("[week_number repr:iso ]"),
        &[BorrowedFormatItem::Component(Component::WeekNumber(
            modifier!(WeekNumber {
                padding: Padding::Zero,
                repr: WeekNumberRepr::Iso,
            })
        ))]
    );
    assert_eq!(
        format_description!("[weekday repr:long one_indexed:true]"),
        &[BorrowedFormatItem::Component(Component::Weekday(
            modifier!(Weekday {
                repr: WeekdayRepr::Long,
                one_indexed: true,
            })
        ))]
    );
    assert_eq!(
        format_description!("[year repr:full base:calendar]"),
        &[BorrowedFormatItem::Component(Component::Year(modifier!(
            Year {
                repr: YearRepr::Full,
                iso_week_based: false,
                padding: Padding::Zero,
                sign_is_mandatory: false,
            }
        )))]
    );
    assert_eq!(
        format_description!("[[ "),
        &[
            BorrowedFormatItem::Literal(b"["),
            BorrowedFormatItem::Literal(b" ")
        ]
    );
    assert_eq!(
        format_description!("[ignore count:2]"),
        &[BorrowedFormatItem::Component(Component::Ignore(
            Ignore::count(NonZero::new(2).expect("2 is not zero"))
        ))]
    );
    assert_eq!(
        format_description!("[unix_timestamp precision:nanosecond sign:mandatory]"),
        &[BorrowedFormatItem::Component(Component::UnixTimestamp(
            modifier!(UnixTimestamp {
                precision: UnixTimestampPrecision::Nanosecond,
                sign_is_mandatory: true,
            })
        ))]
    );
    assert_eq!(
        format_description!("[end]"),
        &[BorrowedFormatItem::Component(Component::End(modifier!(
            End
        )))]
    );
}

#[rstest]
fn date_coverage() {
    assert_eq!(Ok(date!(2000-001)), Date::from_ordinal_date(2000, 1));
    assert_eq!(Ok(date!(2019-W 01-1)), Date::from_ordinal_date(2018, 365));
    assert_eq!(Ok(date!(2021-W 52-6)), Date::from_ordinal_date(2022, 1));
    assert_eq!(Ok(date!(2021-W 34-5)), Date::from_ordinal_date(2021, 239));
}

#[rstest]
fn time_coverage() {
    assert_eq!(time!(12 AM), Time::MIDNIGHT);
    assert_eq!(Ok(time!(12 PM)), Time::from_hms(12, 0, 0));
}

mod demo {
    #[expect(dead_code)]
    type Result<T> = core::result::Result<T, ()>;
    #[expect(dead_code)]
    type Option = core::option::Option<()>;

    time::serde::format_description!(
        seconds,
        OffsetDateTime,
        "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
    );
}
