use serde::{Deserialize, Serialize};
use serde_test::{assert_de_tokens_error, Token};
use time::macros::datetime;
use time::serde::timestamp_millis;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Test {
    #[serde(with = "timestamp_millis")]
    dt: OffsetDateTime,
}

#[test]
fn serialize_timestamp_millis() {
    // serde_test tokens do not support i128, so test with json
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00.123 UTC),
    };
    let value_json = r#"{"dt":946684800123}"#;

    let parsed: Test = serde_json::from_str(value_json).unwrap();
    let parsed_json = serde_json::to_string(&parsed).unwrap();
    
    assert_eq!(value, parsed);
    assert_eq!(value_json, parsed_json);
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
        "invalid type: string \"bad\", expected i128",
    );
}

