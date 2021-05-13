use criterion::Bencher;
use time::format_description::{modifier, Component};
use time::parsing::Parsed;

setup_benchmark! {
    "Parsing",

    fn parse_component(ben: &mut Bencher<'_>) {
        let mut parsed = Parsed::new();

        ben.iter(|| {
            parsed.parse_component(b"2021", Component::Year(modifier::Year {
                padding: modifier::Padding::Zero,
                repr: modifier::YearRepr::Full,
                iso_week_based: false,
                sign_is_mandatory: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"21", Component::Year(modifier::Year {
                padding: modifier::Padding::Zero,
                repr: modifier::YearRepr::LastTwo,
                iso_week_based: false,
                sign_is_mandatory: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"2021", Component::Year(modifier::Year {
                padding: modifier::Padding::Zero,
                repr: modifier::YearRepr::Full,
                iso_week_based: true,
                sign_is_mandatory: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"21", Component::Year(modifier::Year {
                padding: modifier::Padding::Zero,
                repr: modifier::YearRepr::LastTwo,
                iso_week_based: true,
                sign_is_mandatory: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b" 1", Component::Month(modifier::Month {
                padding: modifier::Padding::Space,
                repr: modifier::MonthRepr::Numerical,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"Jan", Component::Month(modifier::Month {
                padding: modifier::Padding::None,
                repr: modifier::MonthRepr::Short,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"January", Component::Month(modifier::Month {
                padding: modifier::Padding::None,
                repr: modifier::MonthRepr::Long,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"012", Component::Ordinal(modifier::Ordinal {
                padding: modifier::Padding::Zero,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"Sun", Component::Weekday(modifier::Weekday {
                repr: modifier::WeekdayRepr::Short,
                one_indexed: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"Sunday", Component::Weekday(modifier::Weekday {
                repr: modifier::WeekdayRepr::Long,
                one_indexed: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"0", Component::Weekday(modifier::Weekday {
                repr: modifier::WeekdayRepr::Sunday,
                one_indexed: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"1", Component::Weekday(modifier::Weekday {
                repr: modifier::WeekdayRepr::Sunday,
                one_indexed: true,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"6", Component::Weekday(modifier::Weekday {
                repr: modifier::WeekdayRepr::Monday,
                one_indexed: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"7", Component::Weekday(modifier::Weekday {
                repr: modifier::WeekdayRepr::Monday,
                one_indexed: true,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"2", Component::WeekNumber(modifier::WeekNumber {
                padding: modifier::Padding::None,
                repr: modifier::WeekNumberRepr::Sunday,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"2", Component::WeekNumber(modifier::WeekNumber {
                padding: modifier::Padding::None,
                repr: modifier::WeekNumberRepr::Monday,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"2", Component::WeekNumber(modifier::WeekNumber {
                padding: modifier::Padding::None,
                repr: modifier::WeekNumberRepr::Iso,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"1", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::One,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"12", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::Two,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"123", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::Three,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"1234", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::Four,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"12345", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::Five,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"123456", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::Six,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"1234567", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::Seven,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"12345678", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::Eight,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"123456789", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::Nine,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"123456789", Component::Subsecond(modifier::Subsecond {
                digits: modifier::SubsecondDigits::OneOrMore,
            }))
        });
    }
}
