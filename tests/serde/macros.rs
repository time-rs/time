use ::serde::{Deserialize, Serialize};
use serde_test::{
    assert_de_tokens_error, assert_ser_tokens_error, assert_tokens, Configure, Token,
};
use time::macros::{date, datetime, offset};
use time::{serde, Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

serde::format_description!(
    offset_dt_format,
    OffsetDateTime,
    "custom format: [year]-[month]-[day] [hour]:[minute]:[second] [offset_hour]:[offset_minute]"
);
serde::format_description!(
    primitive_dt_format,
    PrimitiveDateTime,
    "custom format: [year]-[month]-[day] [hour]:[minute]:[second]"
);
serde::format_description!(time_format, Time, "custom format: [minute]:[second]");
serde::format_description!(date_format, Date, "custom format: [year]-[month]-[day]");
serde::format_description!(
    offset_format,
    UtcOffset,
    "custom format: [offset_hour]:[offset_minute]"
);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestCustomFormat {
    #[serde(with = "offset_dt_format")]
    offset_dt: OffsetDateTime,
    #[serde(with = "primitive_dt_format::option")]
    primitive_dt: Option<PrimitiveDateTime>,
    #[serde(with = "date_format")]
    date: Date,
    #[serde(with = "time_format::option")]
    time: Option<Time>,
    #[serde(with = "offset_format")]
    offset: UtcOffset,
}

#[test]
fn custom_serialize() {
    let value = TestCustomFormat {
        offset_dt: datetime!(2000-01-01 00:00 -4:00),
        primitive_dt: Some(datetime!(2000-01-01 00:00)),
        date: date!(2000 - 01 - 01),
        time: None,
        offset: offset!(-4),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 5,
            },
            Token::Str("offset_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00 -04:00"),
            Token::Str("primitive_dt"),
            Token::Some,
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00"),
            Token::Str("date"),
            Token::BorrowedStr("custom format: 2000-01-01"),
            Token::Str("time"),
            Token::None,
            Token::Str("offset"),
            Token::BorrowedStr("custom format: -04:00"),
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
                len: 5,
            },
            Token::Str("offset_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 0:00:00 -04:00"),
        ],
        "the 'hour' component could not be parsed",
    );
    // Parse problem in optional field.
    assert_de_tokens_error::<TestCustomFormat>(
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 5,
            },
            Token::Str("offset_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00 -04:00"),
            Token::Str("primitive_dt"),
            Token::Some,
            Token::BorrowedStr("custom format: 2000-01-01 0:00:00 -04:00"),
        ],
        "the 'hour' component could not be parsed",
    );
    // Type error
    assert_de_tokens_error::<TestCustomFormat>(
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 5,
            },
            Token::Str("offset_dt"),
            Token::Bool(false),
        ],
        "invalid type: boolean `false`, expected a(n) `OffsetDateTime` in the format \"custom \
         format: [year]-[month]-[day] [hour]:[minute]:[second] [offset_hour]:[offset_minute]\"",
    );
    assert_de_tokens_error::<TestCustomFormat>(
        &[
            Token::Struct {
                name: "TestCustomFormat",
                len: 5,
            },
            Token::Str("offset_dt"),
            Token::BorrowedStr("custom format: 2000-01-01 00:00:00 -04:00"),
            Token::Str("primitive_dt"),
            Token::Bool(false),
        ],
        "invalid type: boolean `false`, expected an `Option<PrimitiveDateTime>` in the format \
         \"custom format: [year]-[month]-[day] [hour]:[minute]:[second]\"",
    );
}

// This format string has offset_hour and offset_minute, but is for formatting PrimitiveDateTime.
serde::format_description!(
    primitive_date_time_format_bad,
    PrimitiveDateTime,
    "[offset_hour]:[offset_minute]"
);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TestCustomFormatPrimitiveDateTimeBad {
    #[serde(with = "primitive_date_time_format_bad")]
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
        "The type being formatted does not contain sufficient information to format a component.",
    );
}
