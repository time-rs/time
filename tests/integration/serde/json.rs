use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_test::Configure;
use time::{Weekday, Time, Month, PrimitiveDateTime, Date, OffsetDateTime, Duration};

enum Format {
    Compact,
    Readable,
}

use Format::*;

fn serialize<T: Serialize>(val: T, fmt: Format) -> Result<String, Box<dyn Error>> {
    let mut buf: Vec<u8> = Vec::new();
    let mut ser = serde_json::Serializer::new(&mut buf);
    match fmt {
        Compact => {
            let ser = (&mut ser).compact();
            val.serialize(ser)?;
        }
        Readable => {
            let ser = (&mut ser).readable();
            val.serialize(ser)?;
        }
    }

    let str = String::from_utf8(buf)?;
    Ok(str)
}

fn deserialize<'a, T: Deserialize<'a>>(from: &'a str, fmt: Format) -> Result<T, Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_str(from);
    let val = match fmt {
        Compact => {
            let de = (&mut de).compact();
            T::deserialize(de)?
        }
        Readable => {
            let de = (&mut de).readable();
            T::deserialize(de)?
        }
    };
    Ok(val)
}

#[test]
fn weekday_json() -> Result<(), Box<dyn Error>> {
    use Weekday::*;

    assert_eq!(serialize(Monday, Compact)?, "1");
    assert_eq!(deserialize::<Weekday>("1", Compact)?, Monday);

    assert_eq!(serialize(Monday, Readable)?, "\"Monday\"");
    assert_eq!(deserialize::<Weekday>("\"Monday\"", Readable)?, Monday);

    Ok(())
}

#[test]
fn month_json() -> Result<(), Box<dyn Error>> {
    use Month::*;

    assert_eq!(serialize(March, Compact)?, "3");
    assert_eq!(deserialize::<Month>("3", Compact)?, March);

    assert_eq!(serialize(March, Readable)?, "\"March\"");
    assert_eq!(deserialize::<Month>("\"March\"", Readable)?, March);

    Ok(())
}

#[test]
fn time_json() -> Result<(), Box<dyn Error>> {
    assert_eq!(serialize(Time::from_hms(12, 40, 20)?, Compact)?, "[12,40,20,0]");
    assert_eq!(deserialize::<Time>("[12,40,20,0]", Compact)?, Time::from_hms(12, 40, 20)?);

    assert_eq!(serialize(Time::from_hms(12, 40, 20)?, Readable)?, "\"12:40:20.0\"");
    assert_eq!(deserialize::<Time>("\"12:40:20.0\"", Readable)?, Time::from_hms(12, 40, 20)?);

    Ok(())
}

#[test]
fn primitive_datetime_json() -> Result<(), Box<dyn Error>> {
    let dt = PrimitiveDateTime::new(Date::from_ordinal_date(2022, 140)?, Time::from_hms(12, 40, 20)?);

    assert_eq!(serialize(dt, Compact)?, "[2022,140,12,40,20,0]");
    assert_eq!(deserialize::<PrimitiveDateTime>("[2022,140,12,40,20,0]", Compact)?, dt);

    assert_eq!(serialize(dt, Readable)?, "\"2022-05-20 12:40:20.0\"");
    assert_eq!(deserialize::<PrimitiveDateTime>("\"2022-05-20 12:40:20.0\"", Readable)?, dt);

    Ok(())
}

#[test]
fn offset_datetime_json() -> Result<(), Box<dyn Error>> {
    let dt = PrimitiveDateTime::new(Date::from_ordinal_date(2022, 140)?, Time::from_hms(12, 40, 20)?);
    let dt = dt.assume_utc();

    assert_eq!(serialize(dt, Compact)?, "[2022,140,12,40,20,0,0,0,0]");
    assert_eq!(deserialize::<OffsetDateTime>("[2022,140,12,40,20,0,0,0,0]", Compact)?, dt);

    assert_eq!(serialize(dt, Readable)?, "\"2022-05-20 12:40:20.0 +00:00:00\"");
    assert_eq!(deserialize::<OffsetDateTime>("\"2022-05-20 12:40:20.0 +00:00:00\"", Readable)?, dt);

    Ok(())
}

#[test]
fn duration_json() -> Result<(), Box<dyn Error>> {
    let dur = Duration::new(50, 0);

    assert_eq!(serialize(dur, Compact)?, "[50,0]");
    assert_eq!(deserialize::<Duration>("[50,0]", Compact)?, dur);

    assert_eq!(serialize(dur, Readable)?, "\"50.000000000\"");
    assert_eq!(deserialize::<Duration>("\"50.000000000\"", Readable)?, dur);

    Ok(())
}

#[test]
fn date_json() -> Result<(), Box<dyn Error>> {
    let date = Date::from_calendar_date(2022, Month::April, 5)?;

    assert_eq!(serialize(date, Compact)?, "[2022,95]");
    assert_eq!(deserialize::<Date>("[2022,95]", Compact)?, date);

    assert_eq!(serialize(date, Readable)?, "\"2022-04-05\"");
    assert_eq!(deserialize::<Date>("\"2022-04-05\"", Readable)?, date);

    Ok(())
}