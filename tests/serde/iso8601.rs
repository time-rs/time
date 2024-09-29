use serde::{Deserialize, Serialize};
use serde_test::{
    assert_de_tokens_error, assert_ser_tokens_error, assert_tokens, Configure, Token,
};
use time::macros::datetime;
use time::serde::iso8601;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Test {
    #[serde(with = "iso8601")]
    dt: OffsetDateTime,
    #[serde(with = "iso8601::option")]
    option_dt: Option<OffsetDateTime>,
}

#[test]
fn serialize() {
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: Some(datetime!(2000-01-01 00:00:00 UTC)),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("+002000-01-01T00:00:00.000000000Z"),
            Token::Str("option_dt"),
            Token::Some,
            Token::BorrowedStr("+002000-01-01T00:00:00.000000000Z"),
            Token::StructEnd,
        ],
    );
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: None,
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("+002000-01-01T00:00:00.000000000Z"),
            Token::Str("option_dt"),
            Token::None,
            Token::StructEnd,
        ],
    );
}

#[test]
fn serialize_error() {
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 +00:00:01),
        option_dt: None,
    };
    assert_ser_tokens_error::<Test>(
        &value,
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
        ],
        "The offset_second component cannot be formatted into the requested format.",
    );
}

#[test]
fn deserialize_error() {
    assert_de_tokens_error::<Test>(
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("bad"),
            Token::StructEnd,
        ],
        "the 'year' component could not be parsed",
    );
}

#[test]
fn issue_674_leap_second_support() {
    serde_test::assert_de_tokens::<Test>(
        &Test {
            dt: datetime!(2016-12-31 23:59:59.999999999 UTC),
            option_dt: Some(datetime!(2016-12-31 23:59:59.999999999 UTC)),
        },
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("2016-12-31T23:59:60.000Z"),
            Token::Str("option_dt"),
            Token::Some,
            Token::BorrowedStr("2016-12-31T23:59:60.000Z"),
            Token::StructEnd,
        ],
    );
}
