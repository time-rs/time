use serde::{Deserialize, Serialize};
use serde_test::{
    assert_de_tokens_error, assert_ser_tokens_error, assert_tokens, Configure, Token,
};
use time::macros::datetime;
use time::{declare_format_string, OffsetDateTime};

declare_format_string!(
    test_custom_format,
    "custom format: [year]-[month]-[day] [hour]:[minute]:[second] [offset_hour]:[offset_minute]"
);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestCustomFormat {
    #[serde(with = "test_custom_format")]
    dt: OffsetDateTime,
}

#[test]
fn custom_serialize() {
    let value = TestCustomFormat {
        dt: datetime!(2000-01-01 00:00:00 -4:00),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 1,
            },
            Token::Str("dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00 -04:00"),
            Token::StructEnd,
        ],
    );
}

#[test]
fn custom_serialize_error() {
    // Deserialization error due to parse problem.
    assert_de_tokens_error::<TestCustomFormat>(
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 1,
            },
            Token::Str("dt"),
            Token::BorrowedStr("not a date"),
            Token::StructEnd,
        ],
        "invalid value: literal, expected valid format",
    );
}

mod test_well_known_format {
    use time::format_description::well_known::Rfc3339;
    time::declare_format!(Rfc3339);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestWellKnownFormat {
    #[serde(with = "test_well_known_format")]
    dt: OffsetDateTime,
}

#[test]
fn well_known_serialize() {
    let value = TestWellKnownFormat {
        dt: datetime!(2000-01-01 00:00:00 -4:00),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestWellKnownFormat",
                len: 1,
            },
            Token::Str("dt"),
            Token::BorrowedStr("2000-01-01T00:00:00-04:00"),
            Token::StructEnd,
        ],
    );
}

#[test]
fn well_known_serialize_error() {
    // Deserialization error due to parse problem.
    assert_de_tokens_error::<TestWellKnownFormat>(
        &[
            Token::Struct {
                name: "TestWellKnownFormat",
                len: 1,
            },
            Token::Str("dt"),
            Token::BorrowedStr("not a date"),
            Token::StructEnd,
        ],
        "invalid value: year, expected valid year",
    );

    // Serialization error due to invalid date.
    let value = TestWellKnownFormat {
        dt: datetime!(-2000-01-01 00:00:00 UTC),
    };
    assert_ser_tokens_error::<TestWellKnownFormat>(
        &value,
        &[
            Token::Struct {
                name: "TestWellKnownFormat",
                len: 1,
            },
            Token::Str("dt"),
        ],
        "The component year has a value that cannot be formatted into the requested format.",
    );
}
