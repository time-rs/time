use time_formatting::format_description::modifier::{
    MonthRepr, Padding, SubsecondDigits, WeekNumberRepr, WeekdayRepr, YearRepr,
};

pub fn padding() -> Vec<(Padding, &'static str)> {
    vec![
        (Padding::Space, "padding:space"),
        (Padding::Zero, "padding:zero"),
        (Padding::None, "padding:none"),
    ]
}

pub fn hour_is_12_hour_clock() -> Vec<(bool, &'static str)> {
    vec![(false, "repr:24"), (true, "repr:12")]
}

pub fn period_is_uppercase() -> Vec<(bool, &'static str)> {
    vec![(true, "case:upper"), (false, "case:lower")]
}

pub fn month_repr() -> Vec<(MonthRepr, &'static str)> {
    vec![
        (MonthRepr::Numerical, "repr:numerical"),
        (MonthRepr::Long, "repr:long"),
        (MonthRepr::Short, "repr:short"),
    ]
}

pub fn subsecond_digits() -> Vec<(SubsecondDigits, &'static str)> {
    vec![
        (SubsecondDigits::One, "digits:1"),
        (SubsecondDigits::Two, "digits:2"),
        (SubsecondDigits::Three, "digits:3"),
        (SubsecondDigits::Four, "digits:4"),
        (SubsecondDigits::Five, "digits:5"),
        (SubsecondDigits::Six, "digits:6"),
        (SubsecondDigits::Seven, "digits:7"),
        (SubsecondDigits::Eight, "digits:8"),
        (SubsecondDigits::Nine, "digits:9"),
        (SubsecondDigits::OneOrMore, "digits:1+"),
    ]
}

pub fn weekday_repr() -> Vec<(WeekdayRepr, &'static str)> {
    vec![
        (WeekdayRepr::Short, "repr:short"),
        (WeekdayRepr::Long, "repr:long"),
        (WeekdayRepr::Sunday, "repr:sunday"),
        (WeekdayRepr::Monday, "repr:monday"),
    ]
}

pub fn week_number_repr() -> Vec<(WeekNumberRepr, &'static str)> {
    vec![
        (WeekNumberRepr::Iso, "repr:iso"),
        (WeekNumberRepr::Sunday, "repr:sunday"),
        (WeekNumberRepr::Monday, "repr:monday"),
    ]
}

pub fn year_repr() -> Vec<(YearRepr, &'static str)> {
    vec![
        (YearRepr::Full, "repr:full"),
        (YearRepr::Century, "repr:century"),
        (YearRepr::LastTwo, "repr:last_two"),
    ]
}

pub fn year_is_iso_week_based() -> Vec<(bool, &'static str)> {
    vec![(false, "base:calendar"), (true, "base:iso_week")]
}

pub fn sign_is_mandatory() -> Vec<(bool, &'static str)> {
    vec![(false, "sign:automatic"), (true, "sign:mandatory")]
}

pub fn weekday_is_one_indexed() -> Vec<(bool, &'static str)> {
    vec![(true, "one_indexed:true"), (false, "one_indexed:false")]
}
