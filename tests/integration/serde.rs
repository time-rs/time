use serde_test::{assert_de_tokens_error, assert_tokens, Compact, Configure, Readable, Token};
use time::macros::{date, datetime, offset, time};
use time::{Duration, Time, Weekday};

#[test]
fn time() {
    assert_tokens(
        &Time::MIDNIGHT.compact(),
        &[
            Token::Tuple { len: 4 },
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &time!("23:58:59.123_456_789").compact(),
        &[
            Token::Tuple { len: 4 },
            Token::U8(23),
            Token::U8(58),
            Token::U8(59),
            Token::U32(123_456_789),
            Token::TupleEnd,
        ],
    );
    assert_de_tokens_error::<Compact<Time>>(
        &[
            Token::Tuple { len: 4 },
            Token::U8(24),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::TupleEnd,
        ],
        "invalid value: integer `24`, expected a value in the range 0..=23",
    );

    assert_tokens(
        &Time::MIDNIGHT.readable(),
        &[Token::BorrowedStr("00:00:00.0")],
    );
    assert_tokens(
        &time!("23:58:59.123_456_789").readable(),
        &[Token::BorrowedStr("23:58:59.123456789")],
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::BorrowedStr("24:00:00.0")],
        "invalid value: integer `24`, expected a value in the range 0..=23",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::BorrowedStr("24-00:00.0")],
        "invalid value: literal, expected valid format",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::BorrowedStr("0:00:00.0")],
        "invalid value: hour, expected valid hour",
    );
    assert_de_tokens_error::<Readable<Time>>(
        &[Token::BorrowedStr("00:00:00.0x")],
        "invalid value: literal, expected no extraneous characters",
    );
}

#[test]
fn date() {
    assert_tokens(
        &date!("-9999-001").compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I32(-9999),
            Token::U16(1),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &date!("+9999-365").compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I32(9999),
            Token::U16(365),
            Token::TupleEnd,
        ],
    );

    assert_tokens(
        &date!("-9999-001").readable(),
        &[Token::BorrowedStr("-9999-01-01")],
    );
    assert_tokens(
        &date!("+9999-365").readable(),
        &[Token::BorrowedStr("9999-12-31")],
    );
}

#[test]
fn primitive_date_time() {
    assert_tokens(
        &datetime!("-9999-001 0:00").compact(),
        &[
            Token::Tuple { len: 6 },
            Token::I32(-9999),
            Token::U16(1),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(0),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &datetime!("+9999-365 23:58:59.123_456_789").compact(),
        &[
            Token::Tuple { len: 6 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(23),
            Token::U8(58),
            Token::U8(59),
            Token::U32(123_456_789),
            Token::TupleEnd,
        ],
    );

    assert_tokens(
        &datetime!("-9999-001 0:00").readable(),
        &[Token::BorrowedStr("-9999-01-01 00:00:00.0")],
    );
    assert_tokens(
        &datetime!("+9999-365 23:58:59.123_456_789").readable(),
        &[Token::BorrowedStr("9999-12-31 23:58:59.123456789")],
    );
}

#[test]
fn offset_date_time() {
    assert_tokens(
        &datetime!("-9999-001 0:00 UTC")
            .to_offset(offset!("+23:58:59"))
            .compact(),
        &[
            Token::Tuple { len: 9 },
            Token::I32(-9999),
            Token::U16(1),
            Token::U8(23),
            Token::U8(58),
            Token::U8(59),
            Token::U32(0),
            Token::I8(23),
            Token::I8(58),
            Token::I8(59),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &datetime!("+9999-365 23:58:59.123_456_789 UTC")
            .to_offset(offset!("-23:58:59"))
            .compact(),
        &[
            Token::Tuple { len: 9 },
            Token::I32(9999),
            Token::U16(365),
            Token::U8(0),
            Token::U8(0),
            Token::U8(0),
            Token::U32(123_456_789),
            Token::I8(-23),
            Token::I8(-58),
            Token::I8(-59),
            Token::TupleEnd,
        ],
    );

    assert_tokens(
        &datetime!("-9999-001 0:00 UTC")
            .to_offset(offset!("+23:58:59"))
            .readable(),
        &[Token::BorrowedStr("-9999-01-01 23:58:59.0 +23:58:59")],
    );
    assert_tokens(
        &datetime!("+9999-365 23:58:59.123_456_789 UTC")
            .to_offset(offset!("-23:58:59"))
            .readable(),
        &[Token::BorrowedStr(
            "9999-12-31 00:00:00.123456789 -23:58:59",
        )],
    );
}

#[test]
fn utc_offset() {
    assert_tokens(
        &offset!("-23:58:59").compact(),
        &[
            Token::Tuple { len: 3 },
            Token::I8(-23),
            Token::I8(-58),
            Token::I8(-59),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &offset!("+23:58:59").compact(),
        &[
            Token::Tuple { len: 3 },
            Token::I8(23),
            Token::I8(58),
            Token::I8(59),
            Token::TupleEnd,
        ],
    );

    assert_tokens(
        &offset!("-23:58:59").readable(),
        &[Token::BorrowedStr("-23:58:59")],
    );
    assert_tokens(
        &offset!("+23:58:59").readable(),
        &[Token::BorrowedStr("+23:58:59")],
    );
}

#[test]
fn duration() {
    assert_tokens(
        &Duration::MIN.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I64(i64::MIN),
            Token::I32(-999_999_999),
            Token::TupleEnd,
        ],
    );
    assert_tokens(
        &Duration::MAX.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I64(i64::MAX),
            Token::I32(999_999_999),
            Token::TupleEnd,
        ],
    );

    assert_tokens(
        &Duration::MIN.readable(),
        &[Token::BorrowedStr("-9223372036854775808.999999999")],
    );
    assert_tokens(
        &Duration::MAX.readable(),
        &[Token::BorrowedStr("9223372036854775807.999999999")],
    );
    assert_tokens(
        &Duration::ZERO.readable(),
        &[Token::BorrowedStr("0.000000000")],
    );
    assert_de_tokens_error::<Readable<Duration>>(
        &[Token::BorrowedStr("x")],
        r#"invalid value: string "x", expected a decimal point"#,
    );
    assert_de_tokens_error::<Readable<Duration>>(
        &[Token::BorrowedStr("x.0")],
        r#"invalid value: string "x", expected a number"#,
    );
    assert_de_tokens_error::<Readable<Duration>>(
        &[Token::BorrowedStr("0.x")],
        r#"invalid value: string "x", expected a number"#,
    );
}

#[test]
fn weekday() {
    assert_tokens(&Weekday::Monday.compact(), &[Token::U8(1)]);
    assert_tokens(&Weekday::Tuesday.compact(), &[Token::U8(2)]);
    assert_tokens(&Weekday::Wednesday.compact(), &[Token::U8(3)]);
    assert_tokens(&Weekday::Thursday.compact(), &[Token::U8(4)]);
    assert_tokens(&Weekday::Friday.compact(), &[Token::U8(5)]);
    assert_tokens(&Weekday::Saturday.compact(), &[Token::U8(6)]);
    assert_tokens(&Weekday::Sunday.compact(), &[Token::U8(7)]);
    assert_de_tokens_error::<Compact<Weekday>>(
        &[Token::U8(0)],
        "invalid value: integer `0`, expected a value in the range 1..=7",
    );

    assert_tokens(&Weekday::Monday.readable(), &[Token::BorrowedStr("Monday")]);
    assert_tokens(
        &Weekday::Tuesday.readable(),
        &[Token::BorrowedStr("Tuesday")],
    );
    assert_tokens(
        &Weekday::Wednesday.readable(),
        &[Token::BorrowedStr("Wednesday")],
    );
    assert_tokens(
        &Weekday::Thursday.readable(),
        &[Token::BorrowedStr("Thursday")],
    );
    assert_tokens(&Weekday::Friday.readable(), &[Token::BorrowedStr("Friday")]);
    assert_tokens(
        &Weekday::Saturday.readable(),
        &[Token::BorrowedStr("Saturday")],
    );
    assert_tokens(&Weekday::Sunday.readable(), &[Token::BorrowedStr("Sunday")]);
    assert_de_tokens_error::<Readable<Weekday>>(
        &[Token::BorrowedStr("NotADay")],
        r#"invalid value: string "NotADay", expected a day of the week"#,
    );
}
