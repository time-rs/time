use quickcheck_dep::{quickcheck, Arbitrary, TestResult};
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

macro_rules! test_shrink {
    ($type:ty,
     $fn_name:ident,
     $($method:ident()).+
     $(, min=$min_value:literal)?
    ) => {
        quickcheck! {
            fn $fn_name(v: $type) -> TestResult {
                let method_value = v.$($method()).+;
                if method_value == test_shrink!(@min_or_zero $($min_value)?) {
                    TestResult::discard()
                } else {
                    TestResult::from_bool(v.shrink().any(|shrunk|
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
    (@min_or_zero) => { 0 };
    (@min_or_zero $min:literal) => { $min };
}

quickcheck! {
    fn date_roundtrip(d: Date) -> bool {
        Date::from_ordinal_date(d.year(), d.ordinal()) == Ok(d)
    }

    fn julian_day_roundtrip(d: Date) -> bool {
        Date::from_julian_day(d.to_julian_day()) == Ok(d)
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

    fn unix_timestamp_roundtrip(odt: OffsetDateTime) -> TestResult {
        match odt.date() {
            Date::MIN | Date::MAX => TestResult::discard(),
            _ => TestResult::from_bool({
                // nanoseconds are not stored in the basic Unix timestamp
                let odt = odt - Duration::nanoseconds(odt.nanosecond().into());
                OffsetDateTime::from_unix_timestamp(odt.unix_timestamp()) == Ok(odt)
            })
        }
    }

    fn unix_timestamp_nanos_roundtrip(odt: OffsetDateTime) -> TestResult {
        match odt.date() {
            Date::MIN | Date::MAX => TestResult::discard(),
            _ => TestResult::from_bool(
                OffsetDateTime::from_unix_timestamp_nanos(odt.unix_timestamp_nanos()) == Ok(odt)
            )
        }
    }

    fn weekday_supports_arbitrary(w: Weekday) -> bool {
        (1..=7).contains(&w.number_from_monday())
    }

    fn weekday_can_shrink(w: Weekday) -> bool {
        match w {
            Weekday::Monday => w.shrink().next() == None,
            _ => w.shrink().next() == Some(w.previous())
        }
    }

    fn month_supports_arbitrary(m: Month) -> bool {
        (1..=12).contains(&(m as u8))
    }

    fn month_can_shrink(m: Month) -> bool {
        match m {
            Month::January => m.shrink().next() == None,
            _ => m.shrink().next() == Some(m.previous())
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

test_shrink!(UtcOffset, utc_offset_can_shrink, whole_seconds());

test_shrink!(
    OffsetDateTime,
    offset_date_time_can_shrink_offset,
    offset().whole_seconds()
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
