#![cfg(feature = "serde")]

use serde_test::{assert_tokens, Configure, Token};
use time::{
    macros::{date, datetime, offset, time},
    Duration, Time, Weekday,
};

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
}
