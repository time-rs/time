use serde::{Deserialize, Serialize};
use serde_test::{assert_de_tokens_error, assert_tokens, Configure, Token};
use time::macros::datetime;
use time::serde::timestamp;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Test {
    #[serde(with = "timestamp")]
    dt: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestMilliseconds {
    #[serde(with = "timestamp::milliseconds")]
    dt: OffsetDateTime,
}

#[test]
fn serialize_timestamp() {
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 1,
            },
            Token::Str("dt"),
            Token::I64(946684800),
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<Test>(
        &[
            Token::Struct {
                name: "Test",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
    let value_milliseconds = TestMilliseconds {
        dt: datetime!(2000-01-01 00:00:00.999 UTC),
    };
    assert_de_tokens_error::<TestMilliseconds>(
        &[
            Token::Struct {
                name: "TestMilliseconds",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i128",
    );
    // serde_test does not support I128, see: https://github.com/serde-rs/test/issues/18
    let serde_milliseconds: TestMilliseconds = serde_json::from_str(r#"{"dt":946684800999}"#).unwrap();
    assert_eq!(value_milliseconds.dt, serde_milliseconds.dt);
}
