use rstest::rstest;
use serde::{Deserialize, Serialize};
use serde_test2::{Compact, Configure, Token, assert_ser_tokens_error, assert_tokens};
use time::OffsetDateTime;
use time::serde::rfc6265;
use time_macros::datetime;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Test {
    #[serde(with = "rfc6265")]
    dt: OffsetDateTime,
    #[serde(with = "rfc6265::option")]
    option_dt: Option<OffsetDateTime>,
}

#[rstest]
#[case(
    Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: Some(datetime!(2000-01-01 00:00:00 UTC)),
    }.compact(),
    &[
        Token::Struct {
            name: "Test",
            len: 2,
        },
        Token::Str("dt"),
        Token::BorrowedStr("Sat, 01 Jan 2000 00:00:00 GMT"),
        Token::Str("option_dt"),
        Token::Some,
        Token::BorrowedStr("Sat, 01 Jan 2000 00:00:00 GMT"),
        Token::StructEnd,
    ]
)]
fn serialize_deserialize(#[case] value: Compact<Test>, #[case] tokens: &[Token]) {
    assert_tokens(&value, tokens);
}

#[rstest]
#[case(
    Test {
        dt: datetime!(1600-01-01 00:00:00 UTC),
        option_dt: None,
    },
    &[
        Token::Struct {
            name: "Test",
            len: 2,
        },
        Token::Str("dt"),
    ],
    "The year component cannot be formatted into the requested format."
)]
#[case(
    Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: Some(datetime!(1600-01-01 00:00:00 UTC)),
    },
    &[
        Token::Struct {
            name: "Test",
            len: 2,
        },
        Token::Str("dt"),
        Token::BorrowedStr("Sat, 01 Jan 2000 00:00:00 GMT"),
        Token::Str("option_dt"),
    ],
    "The year component cannot be formatted into the requested format."
)]
fn serialize_error(#[case] value: Test, #[case] tokens: &[Token], #[case] expected: &str) {
    assert_ser_tokens_error::<Test>(&value, tokens, expected);
}

#[rstest]
#[case(
    r#"{"dt": "Sat, 01 Jan 2000 00:00:00 GMT", "option_dt": null}"#,
    Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: None,
    }
)]
fn parse_json(#[case] json: &str, #[case] expected: Test) {
    assert_eq!(serde_json::from_str::<Test>(json).ok(), Some(expected));
}
