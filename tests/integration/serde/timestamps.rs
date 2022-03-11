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
}
