use std::fmt::Debug;
use std::marker::PhantomData;

use rstest::rstest;
use serde::{Deserialize, Serialize};
use serde_test::{Token, assert_de_tokens_error, assert_tokens};
use time::OffsetDateTime;
use time::macros::datetime;
use time::serde::timestamp;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestSeconds {
    #[serde(with = "timestamp")]
    dt: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestMilliseconds {
    #[serde(with = "timestamp::milliseconds")]
    dt: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestMicroseconds {
    #[serde(with = "timestamp::microseconds")]
    dt: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestNanoseconds {
    #[serde(with = "timestamp::nanoseconds")]
    dt: OffsetDateTime,
}

#[rstest]
#[case(
    TestSeconds { dt: datetime!(2000-01-01 00:00:00 UTC) },
    &[
        Token::Struct {
            name: "TestSeconds",
            len: 1,
        },
        Token::Str("dt"),
        Token::I64(946684800),
        Token::StructEnd,
    ],
)]
// TODO add cases for milliseconds, microseconds, and nanoseconds when serde_test supports i128
fn round_trip<T>(#[case] value: T, #[case] tokens: &[Token])
where
    T: Debug + PartialEq + Serialize + for<'de> Deserialize<'de>,
{
    assert_tokens(&value, tokens);
}

#[rstest]
#[case(
    PhantomData::<TestSeconds>,
    &[
        Token::Struct {
            name: "TestSeconds",
            len: 1,
        },
        Token::Str("dt"),
        Token::Str("bad"),
        Token::StructEnd,
    ],
    r#"invalid type: string "bad", expected i64"#,
)]
#[case(
    PhantomData::<TestMilliseconds>,
    &[
        Token::Struct {
            name: "TestMilliseconds",
            len: 1,
        },
        Token::Str("dt"),
        Token::Str("bad"),
        Token::StructEnd,
    ],
    r#"invalid type: string "bad", expected i128"#,
)]
#[case(
    PhantomData::<TestMicroseconds>,
    &[
        Token::Struct {
            name: "TestMicroseconds",
            len: 1,
        },
        Token::Str("dt"),
        Token::Str("bad"),
        Token::StructEnd,
    ],
    r#"invalid type: string "bad", expected i128"#,
)]
#[case(
    PhantomData::<TestNanoseconds>,
    &[
        Token::Struct {
            name: "TestNanoseconds",
            len: 1,
        },
        Token::Str("dt"),
        Token::Str("bad"),
        Token::StructEnd,
    ],
    r#"invalid type: string "bad", expected i128"#,
)]
fn deserialize_error<T>(
    #[case] _type: PhantomData<T>,
    #[case] tokens: &[Token],
    #[case] expected: &str,
) where
    T: for<'de> Deserialize<'de>,
{
    assert_de_tokens_error::<T>(tokens, expected);
}
