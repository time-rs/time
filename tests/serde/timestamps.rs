use serde::{Deserialize, Serialize};
use serde_test::{assert_de_tokens_error, assert_de_tokens, assert_tokens, Configure, Token};
use time::macros::datetime;
use time::serde::timestamp;
use time::{OffsetDateTime,PrimitiveDateTime};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestOffset {
    #[serde(with = "timestamp")]
    dt: OffsetDateTime,
}

#[test]
fn serialize_timestamp_offset() {
    let value = TestOffset {
        dt: datetime!(2000-01-01 00:00:00 UTC),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestOffset",
                len: 1,
            },
            Token::Str("dt"),
            Token::I64(946684800),
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestOffset>(
        &[
            Token::Struct {
                name: "TestOffset",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestPrimitive{
    #[serde(with = "timestamp")]
    dt: PrimitiveDateTime,
}


#[test]
fn serialize_timestamp_primitive() {
    let value = TestPrimitive {
        dt: datetime!(2015-01-01 00:00:00),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestPrimitive",
                len: 1,
            },
            Token::Str("dt"),
            Token::I64(1420070400),
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestPrimitive>(
        &[
            Token::Struct {
                name: "TestPrimitive",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestOffsetOption {
    #[serde(with = "timestamp")]
    dt: Option<OffsetDateTime>,
}


#[test]
fn serialize_timestamp_offset_option() {
    let value = TestOffsetOption {
        dt: Some(datetime!(1990-01-01 00:00:00 UTC)),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestOffsetOption",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::I64(631152000),
            Token::StructEnd,
        ],
    );
    let value = TestOffsetOption {
        dt: None,
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestOffsetOption",
                len: 1,
            },
            Token::Str("dt"),
            Token::None,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestOffsetOption>(
        &[
            Token::Struct {
                name: "TestOffsetOption",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
    assert_de_tokens::<TestOffsetOption>(
        &TestOffsetOption {
            dt: Some(datetime!(1969-09-30 09:46:40 +0000))
        },
        &[
            Token::Struct {
                name: "TestOffsetOption",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::I64(-8_000_000),
            Token::StructEnd,
        ],
    );
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestPrimitiveOption {
    #[serde(with = "timestamp")]
    dt: Option<PrimitiveDateTime>,
}


#[test]
fn serialize_timestamp_primitive_option() {
    let value = TestPrimitiveOption {
        dt: Some(datetime!(1980-01-01 00:00:00)),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestPrimitiveOption",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::I64(315532800),
            Token::StructEnd,
        ],
    );
    let value = TestPrimitiveOption {
        dt: None,
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestPrimitiveOption",
                len: 1,
            },
            Token::Str("dt"),
            Token::None,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestPrimitiveOption>(
        &[
            Token::Struct {
                name: "TestPrimitiveOption",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
    assert_de_tokens::<TestPrimitiveOption>(
        &TestPrimitiveOption {
            dt: Some(datetime!(1930-04-11 02:35:12))
        },
        &[
            Token::Struct {
                name: "TestPrimitiveOption",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::I64(-1253654688),
            Token::StructEnd,
        ],
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestOffsetMillis {
    #[serde(with = "timestamp::millis")]
    dt: OffsetDateTime,
}

#[test]
fn serialize_timestamp_offset_millis() {
    let value = TestOffsetMillis {
        dt: datetime!(2000-01-01 00:00:00.123 UTC),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestOffsetMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::I64(946684800123),
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestOffsetMillis>(
        &[
            Token::Struct {
                name: "TestOffsetMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );

    assert_de_tokens::<TestOffsetMillis>(
        &TestOffsetMillis {
            dt: datetime!(1930-04-11 02:35:12.123 +00)
        },
        &[
            Token::Struct {
                name: "TestOffsetMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::I64(-1253654687877),
            Token::StructEnd,
        ],
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestPrimitiveMillis{
    #[serde(with = "timestamp::millis")]
    dt: PrimitiveDateTime,
}


#[test]
fn serialize_timestamp_primitive_millis() {
    let value = TestPrimitiveMillis {
        dt: datetime!(2015-01-01 00:00:00.123),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestPrimitiveMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::I64(1420070400123),
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestPrimitiveMillis>(
        &[
            Token::Struct {
                name: "TestPrimitiveMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
    assert_de_tokens::<TestPrimitiveMillis>(
        &TestPrimitiveMillis {
            dt: datetime!(1969-09-30 09:46:40.123)
        },
        &[
            Token::Struct {
                name: "TestPrimitiveMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::I64(-7_999_999_877),
            Token::StructEnd,
        ],
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestOffsetOptionMillis {
    #[serde(with = "timestamp::millis")]
    dt: Option<OffsetDateTime>,
}


#[test]
fn serialize_timestamp_offset_option_millis() {
    let value = TestOffsetOptionMillis {
        dt: Some(datetime!(1990-01-01 00:00:00.123 UTC)),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestOffsetOptionMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::I64(631152000123),
            Token::StructEnd,
        ],
    );
    let value = TestOffsetOptionMillis {
        dt: None,
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestOffsetOptionMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::None,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestOffsetOptionMillis>(
        &[
            Token::Struct {
                name: "TestOffsetOptionMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
    assert_de_tokens::<TestOffsetOptionMillis>(
        &TestOffsetOptionMillis {
            dt: Some(datetime!(1969-09-30 09:46:40.123 +0000))
        },
        &[
            Token::Struct {
                name: "TestOffsetOptionMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::I64(-7_999_999_877),
            Token::StructEnd,
        ],
    );
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestPrimitiveOptionMillis {
    #[serde(with = "timestamp::millis")]
    dt: Option<PrimitiveDateTime>,
}


#[test]
fn serialize_timestamp_primitive_option_millis() {
    let value = TestPrimitiveOptionMillis {
        dt: Some(datetime!(1980-01-01 00:00:00.123)),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestPrimitiveOptionMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::I64(315532800123),
            Token::StructEnd,
        ],
    );
    let value = TestPrimitiveOptionMillis {
        dt: None,
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestPrimitiveOptionMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::None,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestPrimitiveOptionMillis>(
        &[
            Token::Struct {
                name: "TestPrimitiveOptionMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
    assert_de_tokens::<TestPrimitiveOptionMillis>(
        &TestPrimitiveOptionMillis {
            dt: Some(datetime!(1930-04-11 02:35:12.123))
        },
        &[
            Token::Struct {
                name: "TestPrimitiveOptionMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Some,
            Token::I64(-1253654687877),
            Token::StructEnd,
        ],
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestOffsetVec {
    #[serde(with = "timestamp")]
    dt: Vec<OffsetDateTime>,
}

#[test]
fn serialize_timestamp_offset_vec() {
    let value = TestOffsetVec {
        dt: vec![datetime!(2000-01-01 00:00:00 UTC),datetime!(2010-01-01 00:00:00 UTC)]
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestOffsetVec",
                len: 1,
            },
            Token::Str("dt"),
            Token::Seq{ len: Some(2) },
            Token::I64(946684800),
            Token::I64(1262304000),
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestOffsetVec>(
        &[
            Token::Struct {
                name: "TestOffsetVec",
                len: 1,
            },
            Token::Str("dt"),
            Token::Seq{ len : Some(1) },
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestPrimitiveVec{
    #[serde(with = "timestamp")]
    dt: Vec<PrimitiveDateTime>,
}


#[test]
fn serialize_timestamp_primitive_vec() {
    let value = TestPrimitiveVec {
        dt: vec![datetime!(1860-01-01 00:00:00),datetime!(1972-01-01 00:00:00)],
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestPrimitiveVec",
                len: 1,
            },
            Token::Str("dt"),
            Token::Seq{ len: Some(2) },
            Token::I64(-3471292800),
            Token::I64(63072000),
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestPrimitiveVec>(
        &[
            Token::Struct {
                name: "TestPrimitiveVec",
                len: 1,
            },
            Token::Str("dt"),
            Token::Seq{ len : Some(1) },
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestOffsetVecMillis {
    #[serde(with = "timestamp::millis")]
    dt: Vec<OffsetDateTime>,
}

#[test]
fn serialize_timestamp_offset_vec_millis() {
    let value = TestOffsetVecMillis {
        dt: vec![datetime!(2000-01-01 00:00:00.123 UTC),datetime!(2010-01-01 00:00:00.123 UTC)]
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestOffsetVecMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Seq{ len: Some(2) },
            Token::I64(946684800123),
            Token::I64(1262304000123),
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestOffsetVecMillis>(
        &[
            Token::Struct {
                name: "TestOffsetVecMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Seq{ len : Some(1) },
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestPrimitiveVecMillis{
    #[serde(with = "timestamp::millis")]
    dt: Vec<PrimitiveDateTime>,
}


#[test]
fn serialize_timestamp_primitive_vec_millis() {
    let value = TestPrimitiveVecMillis {
        dt: vec![datetime!(1860-01-01 00:00:00.123),datetime!(1972-01-01 00:00:00.123)],
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "TestPrimitiveVecMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Seq{ len: Some(2) },
            Token::I64(-3471292799877),
            Token::I64(63072000123),
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<TestPrimitiveVecMillis>(
        &[
            Token::Struct {
                name: "TestPrimitiveVecMillis",
                len: 1,
            },
            Token::Str("dt"),
            Token::Seq{ len : Some(1) },
            Token::Str("bad"),
            Token::StructEnd,
        ],
        "invalid type: string \"bad\", expected i64",
    );
}
