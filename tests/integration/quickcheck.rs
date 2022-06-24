use quickcheck::{Arbitrary, TestResult};
use quickcheck_macros::quickcheck;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

macro_rules! test_shrink {
    ($type:ty,
     $fn_name:ident,
     $($method:ident()).+
     $(, min=$min_value:literal)?
    ) => {
        #[quickcheck]
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
    };
    (@min_or_zero) => { 0 };
    (@min_or_zero $min:literal) => { $min };
}

#[quickcheck]
fn date_yo_roundtrip(d: Date) -> bool {
    Date::from_ordinal_date(d.year(), d.ordinal()) == Ok(d)
}

#[quickcheck]
fn date_ymd_roundtrip(d: Date) -> bool {
    Date::from_calendar_date(d.year(), d.month(), d.day()) == Ok(d)
}

#[quickcheck]
fn date_ywd_roundtrip(d: Date) -> bool {
    let (year, week, weekday) = d.to_iso_week_date();
    Date::from_iso_week_date(year, week, weekday) == Ok(d)
}

#[quickcheck]
fn julian_day_roundtrip(d: Date) -> bool {
    Date::from_julian_day(d.to_julian_day()) == Ok(d)
}

#[quickcheck]
fn duration_roundtrip(d: Duration) -> bool {
    Duration::new(d.whole_seconds(), d.subsec_nanoseconds()) == d
}

#[quickcheck]
fn time_roundtrip(t: Time) -> bool {
    Time::from_hms_nano(t.hour(), t.minute(), t.second(), t.nanosecond()) == Ok(t)
}

#[quickcheck]
fn primitive_date_time_roundtrip(a: PrimitiveDateTime) -> bool {
    PrimitiveDateTime::new(a.date(), a.time()) == a
}

#[quickcheck]
fn utc_offset_roundtrip(o: UtcOffset) -> bool {
    let (hours, minutes, seconds) = o.as_hms();
    UtcOffset::from_hms(hours, minutes, seconds) == Ok(o)
}

#[quickcheck]
fn offset_date_time_roundtrip(a: OffsetDateTime) -> bool {
    PrimitiveDateTime::new(a.date(), a.time()).assume_offset(a.offset()) == a
}

#[quickcheck]
fn unix_timestamp_roundtrip(odt: OffsetDateTime) -> TestResult {
    match odt.date() {
        Date::MIN | Date::MAX => TestResult::discard(),
        _ => TestResult::from_bool({
            // nanoseconds are not stored in the basic Unix timestamp
            let odt = odt - Duration::nanoseconds(odt.nanosecond().into());
            OffsetDateTime::from_unix_timestamp(odt.unix_timestamp()) == Ok(odt)
        }),
    }
}

#[quickcheck]
fn unix_timestamp_nanos_roundtrip(odt: OffsetDateTime) -> TestResult {
    match odt.date() {
        Date::MIN | Date::MAX => TestResult::discard(),
        _ => TestResult::from_bool(
            OffsetDateTime::from_unix_timestamp_nanos(odt.unix_timestamp_nanos()) == Ok(odt),
        ),
    }
}

#[quickcheck]
fn weekday_supports_arbitrary(w: Weekday) -> bool {
    (1..=7).contains(&w.number_from_monday())
}

#[quickcheck]
fn weekday_can_shrink(w: Weekday) -> bool {
    match w {
        Weekday::Monday => w.shrink().next() == None,
        _ => w.shrink().next() == Some(w.previous()),
    }
}

#[quickcheck]
fn month_supports_arbitrary(m: Month) -> bool {
    (1..=12).contains(&(m as u8))
}

#[quickcheck]
fn month_can_shrink(m: Month) -> bool {
    match m {
        Month::January => m.shrink().next() == None,
        _ => m.shrink().next() == Some(m.previous()),
    }
}

#[quickcheck]
fn date_replace_year(date: Date, year: i32) -> bool {
    date.replace_year(year) == Date::from_calendar_date(year, date.month(), date.day())
}

#[quickcheck]
fn date_replace_month(date: Date, month: Month) -> bool {
    date.replace_month(month) == Date::from_calendar_date(date.year(), month, date.day())
}

#[quickcheck]
fn date_replace_day(date: Date, day: u8) -> bool {
    date.replace_day(day) == Date::from_calendar_date(date.year(), date.month(), day)
}

#[quickcheck]
fn time_replace_hour(time: Time, hour: u8) -> bool {
    time.replace_hour(hour)
        == Time::from_hms_nano(hour, time.minute(), time.second(), time.nanosecond())
}

#[quickcheck]
fn pdt_replace_year(pdt: PrimitiveDateTime, year: i32) -> bool {
    pdt.replace_year(year)
        == Date::from_calendar_date(year, pdt.month(), pdt.day())
            .map(|date| date.with_time(pdt.time()))
}

#[quickcheck]
fn pdt_replace_month(pdt: PrimitiveDateTime, month: Month) -> bool {
    pdt.replace_month(month)
        == Date::from_calendar_date(pdt.year(), month, pdt.day())
            .map(|date| date.with_time(pdt.time()))
}

#[quickcheck]
fn pdt_replace_day(pdt: PrimitiveDateTime, day: u8) -> bool {
    pdt.replace_day(day)
        == Date::from_calendar_date(pdt.year(), pdt.month(), day)
            .map(|date| date.with_time(pdt.time()))
}

// Regression test for #481
#[quickcheck]
fn time_sub_time_no_panic(time_a: Time, time_b: Time) -> bool {
    std::panic::catch_unwind(|| {
        let _ = time_a - time_b;
        let _ = time_b - time_a;
    })
    .is_ok()
}

#[quickcheck]
fn from_julian_day_no_panic(julian_day: i32) -> TestResult {
    if !(Date::MIN.to_julian_day()..=Date::MAX.to_julian_day()).contains(&julian_day) {
        return TestResult::discard();
    }

    TestResult::from_bool(
        std::panic::catch_unwind(|| {
            let _ = Date::from_julian_day(julian_day);
        })
        .is_ok(),
    )
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
