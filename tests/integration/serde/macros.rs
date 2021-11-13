use std::io;

use serde::{Deserialize, Serialize};
use serde_test::{
    assert_de_tokens_error, assert_ser_tokens_error, assert_tokens, Configure, Token,
};
use time::macros::{
    datetime, declare_format_string_offset_date_time, declare_format_string_primitive_date_time,
};
use time::{OffsetDateTime, PrimitiveDateTime};

declare_format_string_offset_date_time!(
    test_custom_format_offset_date_time,
    "custom format: [year]-[month]-[day] [hour]:[minute]:[second] [offset_hour]:[offset_minute]"
);

declare_format_string_primitive_date_time!(
    test_custom_format_primitive_date_time,
    "custom format: [year]-[month]-[day] [hour]:[minute]:[second]"
);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestCustomFormat {
    #[serde(with = "test_custom_format_offset_date_time")]
    offset_dt: OffsetDateTime,
    #[serde(with = "test_custom_format_offset_date_time::option")]
    offset_option: Option<OffsetDateTime>,
    #[serde(with = "test_custom_format_primitive_date_time")]
    primitive_dt: PrimitiveDateTime,
    #[serde(with = "test_custom_format_primitive_date_time::option")]
    primitive_option: Option<PrimitiveDateTime>,
}

#[test]
fn custom_serialize() {
    let value = TestCustomFormat {
        offset_dt: datetime!(2000-01-01 00:00 -4:00),
        offset_option: Some(datetime!(2000-01-01 00:00 -4:00)),
        primitive_dt: datetime!(2000-01-01 00:00),
        primitive_option: Some(datetime!(2000-01-01 00:00)),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 4,
            },
            Token::Str("offset_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00 -04:00"),
            Token::Str("offset_option"),
            Token::Some,
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00 -04:00"),
            Token::Str("primitive_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00"),
            Token::Str("primitive_option"),
            Token::Some,
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00"),
            Token::StructEnd,
        ],
    );
    let value = TestCustomFormat {
        offset_dt: datetime!(2000-01-01 00:00 -4:00),
        offset_option: None,
        primitive_dt: datetime!(2000-01-01 00:00),
        primitive_option: None,
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 4,
            },
            Token::Str("offset_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00 -04:00"),
            Token::Str("offset_option"),
            Token::None,
            Token::Str("primitive_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00"),
            Token::Str("primitive_option"),
            Token::None,
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
                len: 4,
            },
            Token::Str("offset_dt"),
            Token::BorrowedStr("not a date"),
            Token::StructEnd,
        ],
        "invalid value: literal, expected valid format",
    );
    // Parse problem in optional field.
    assert_de_tokens_error::<TestCustomFormat>(
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 2,
            },
            Token::Str("offset_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00 -04:00"),
            Token::Str("offset_option"),
            Token::Some,
            Token::BorrowedStr("not a date"),
            Token::StructEnd,
        ],
        "invalid value: literal, expected valid format",
    );
}

struct BadWriter;

impl io::Write for BadWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "oh no"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "oh no"))
    }
}

#[test]
fn custom_serialize_io_error() {
    let value = TestCustomFormat {
        offset_dt: datetime!(2000-01-01 00:00 -4:00),
        offset_option: None,
        primitive_dt: datetime!(2000-01-01 00:00),
        primitive_option: None,
    };

    let mut bad_writer = BadWriter;
    let mut ser = serde_json::Serializer::new(&mut bad_writer);
    let res = value.compact().serialize(&mut ser);

    match res {
        Err(err) => {
            assert!(err.is_io());
            assert_eq!(format!("{}", err), "oh no".to_string());
        }
        _ => {
            panic!("Expected error.");
        }
    };
}

// This format string has offset_hour and offset_minute, but is for formatting
// PrimitiveDateTime.
declare_format_string_primitive_date_time!(
    test_custom_format_primitive_date_time_bad,
    "[offset_hour]:[offset_minute]"
);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestCustomFormatPrimitiveDateTimeBad {
    #[serde(with = "test_custom_format_primitive_date_time_bad")]
    dt: PrimitiveDateTime,
}

#[test]
fn custom_serialize_bad_type_error() {
    let value = TestCustomFormatPrimitiveDateTimeBad {
        dt: datetime!(2000-01-01 00:00),
    };

    assert_ser_tokens_error::<TestCustomFormatPrimitiveDateTimeBad>(
        &value,
        &[
            Token::Struct {
                name: "TestCustomFormatPrimitiveDateTimeBad",
                len: 1,
            },
            Token::Str("dt"),
        ],
        "insufficient type information to format a component",
    );
}
