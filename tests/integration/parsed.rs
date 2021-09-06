use std::num::{NonZeroU16, NonZeroU8};

use time::format_description::FormatItem;
use time::parsing::Parsed;
use time::{Month, Time, Weekday};

#[test]
fn getters_setters() {
    macro_rules! getters_setters {
        ($($setter:ident $getter:ident $value:expr;)*) => {$(
            let mut parsed = Parsed::new();
            parsed.$setter($value);
            assert_eq!(parsed.$getter(), Some($value));
        )*};
    }

    getters_setters! {
        set_year year 5;
        set_year_last_two year_last_two 5;
        set_iso_year iso_year 5;
        set_iso_year_last_two iso_year_last_two 5;
        set_month month Month::May;
        set_sunday_week_number sunday_week_number 5;
        set_monday_week_number monday_week_number 5;
        set_iso_week_number iso_week_number NonZeroU8::new(5).expect("valid value");
        set_weekday weekday Weekday::Monday;
        set_ordinal ordinal NonZeroU16::new(5).expect("valid value");
        set_day day NonZeroU8::new(5).expect("valid value");
        set_hour_24 hour_24 5;
        set_hour_12 hour_12 NonZeroU8::new(5).expect("valid value");
        set_hour_12_is_pm hour_12_is_pm true;
        set_minute minute 5;
        set_second second 5;
        set_subsecond subsecond 5;
        set_offset_hour offset_hour 5;
        set_offset_minute offset_minute 5;
        set_offset_second offset_second 5;
    }
}

#[test]
fn single_item_parse() {
    assert!(Time::parse("a", &FormatItem::Literal(b"a")).is_err());
    assert!(Time::parse("b", &FormatItem::Literal(b"a")).is_err());
}
