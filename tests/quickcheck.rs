#![cfg(feature = "quickcheck")]

use quickcheck_dep::{quickcheck, Arbitrary, TestResult};
use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

macro_rules! test_shrink {
    ($type:ty,
     $fn_name:ident,
     $($method:ident()).+
     $(, min=$min_value:literal)?
    ) => {
        quickcheck! {
            fn $fn_name(v: $type) -> TestResult {
                let method_value = v.$($method()).+;
                if method_value == 0 $(|| method_value == $min_value)? {
                    TestResult::discard()
                } else {
                    TestResult::from_bool(v.shrink().take(1_000).any(|shrunk|
                        if method_value > 0 {
                            shrunk.$($method()).+ < v.$($method()).+
                        } else {
                            shrunk.$($method()).+ > v.$($method()).+
                        }
                    ))
                }
            }
        }
    };
}

quickcheck! {
    fn date_roundtrip(d: Date) -> bool {
        Date::from_ordinal_date(d.year(), d.ordinal()) == Ok(d)
    }

    fn duration_roundtrip(d: Duration) -> bool {
        Duration::new(d.whole_seconds(), d.subsec_nanoseconds()) == d
    }

    fn time_roundtrip(t: Time) -> bool {
        Time::from_hms_nano(t.hour(), t.minute(), t.second(), t.nanosecond()) == Ok(t)
    }

    fn primitive_date_time_roundtrip(a: PrimitiveDateTime) -> bool {
        PrimitiveDateTime::new(a.date(), a.time()) == a
    }

    fn utc_offset_roundtrip(o: UtcOffset) -> bool {
        let (hours, minutes, seconds) = o.as_hms();
        UtcOffset::from_hms(hours, minutes, seconds) == Ok(o)
    }

    fn offset_date_time_roundtrip(a: OffsetDateTime) -> bool {
        PrimitiveDateTime::new(a.date(), a.time()).assume_offset(a.offset()) == a
    }

    fn weekday_supports_arbitrary(w: Weekday) -> bool {
        (1..=7).contains(&w.iso_weekday_number())
    }

    fn weekday_can_shrink(w: Weekday) -> bool {
        match w {
            Weekday::Monday => w.shrink().next() == None,
            _ => w.shrink().next() == Some(w.previous())
        }
    }
}

test_shrink!(Date, date_can_shrink_year, year());
test_shrink!(Date, date_can_shrink_ordinal, ordinal(), min = 1);

test_shrink!(Duration, duration_can_shrink_seconds, whole_seconds());
test_shrink!(Duration, duration_can_shrink_ns, subsec_nanoseconds());

test_shrink!(Time, time_can_shrink_hour, hour());
test_shrink!(Time, time_can_shrink_minute, minute());
test_shrink!(Time, time_can_shrink_second, second());
test_shrink!(Time, time_can_shrink_nanosecond, nanosecond());

test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_year,
    year()
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_ordinal,
    ordinal(),
    min = 1
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_hour,
    hour()
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_minute,
    minute()
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_second,
    second()
);
test_shrink!(
    PrimitiveDateTime,
    primitive_date_time_can_shrink_nanosecond,
    nanosecond()
);

test_shrink!(UtcOffset, utc_offset_can_shrink, to_seconds());

test_shrink!(
    OffsetDateTime,
    offset_date_time_can_shrink_offset,
    offset().to_seconds()
);
test_shrink!(OffsetDateTime, offset_date_time_can_shrink_year, year());
test_shrink!(
    OffsetDateTime,
    offset_date_time_can_shrink_ordinal,
    ordinal(),
    min = 1
);
test_shrink!(OffsetDateTime, offset_date_time_can_shrink_hour, hour());
test_shrink!(OffsetDateTime, offset_date_time_can_shrink_minute, minute());
test_shrink!(OffsetDateTime, offset_date_time_can_shrink_second, second());
test_shrink!(
    OffsetDateTime,
    offset_date_time_can_shrink_nanosecond,
    nanosecond()
);
