use criterion::Bencher;
use criterion_cycles_per_byte::CyclesPerByte;
use time::format_description::{modifier, Component};
use time::parsing::Parsed;

macro_rules! component {
    ($name:ident {$($field:ident : $value:expr),+ $(,)? }) => {{
        const COMPONENT: Component = Component::$name({
            let mut modifier = modifier::$name::default();
            $(modifier.$field = $value;)+
            modifier
        });
        COMPONENT
    }};
}

setup_benchmark! {
    "Parsing",

    fn parse_component_year(ben: &mut Bencher<'_, CyclesPerByte>) {
        let mut parsed = Parsed::new();
        ben.iter(|| {
            parsed.parse_component(b"2021", component!(Year {
                padding: modifier::Padding::Zero,
                repr: modifier::YearRepr::Full,
                iso_week_based: false,
                sign_is_mandatory: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"21", component!(Year {
                padding: modifier::Padding::Zero,
                repr: modifier::YearRepr::LastTwo,
                iso_week_based: false,
                sign_is_mandatory: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"2021", component!(Year {
                padding: modifier::Padding::Zero,
                repr: modifier::YearRepr::Full,
                iso_week_based: true,
                sign_is_mandatory: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"21", component!(Year {
                padding: modifier::Padding::Zero,
                repr: modifier::YearRepr::LastTwo,
                iso_week_based: true,
                sign_is_mandatory: false,
            }))
        });
    }

    fn parse_component_month(ben: &mut Bencher<'_, CyclesPerByte>) {
        let mut parsed = Parsed::new();
        ben.iter(|| {
            parsed.parse_component(b" 1", component!(Month {
                padding: modifier::Padding::Space,
                repr: modifier::MonthRepr::Numerical,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"Jan", component!(Month {
                padding: modifier::Padding::None,
                repr: modifier::MonthRepr::Short,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"January", component!(Month {
                padding: modifier::Padding::None,
                repr: modifier::MonthRepr::Long,
            }))
        });
    }

    fn parse_component_ordinal(ben: &mut Bencher<'_, CyclesPerByte>) {
        let mut parsed = Parsed::new();
        ben.iter(|| {
            parsed.parse_component(b"012", component!(Ordinal {
                padding: modifier::Padding::Zero,
            }))
        });
    }

    fn parse_component_weekday(ben: &mut Bencher<'_, CyclesPerByte>) {
        let mut parsed = Parsed::new();
        ben.iter(|| {
            parsed.parse_component(b"Sun", component!(Weekday {
                repr: modifier::WeekdayRepr::Short,
                one_indexed: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"Sunday", component!(Weekday {
                repr: modifier::WeekdayRepr::Long,
                one_indexed: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"0", component!(Weekday {
                repr: modifier::WeekdayRepr::Sunday,
                one_indexed: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"1", component!(Weekday {
                repr: modifier::WeekdayRepr::Sunday,
                one_indexed: true,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"6", component!(Weekday {
                repr: modifier::WeekdayRepr::Monday,
                one_indexed: false,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"7", component!(Weekday {
                repr: modifier::WeekdayRepr::Monday,
                one_indexed: true,
            }))
        });
    }

    fn parse_component_week_number(ben: &mut Bencher<'_, CyclesPerByte>) {
        let mut parsed = Parsed::new();
        ben.iter(|| {
            parsed.parse_component(b"2", component!(WeekNumber {
                padding: modifier::Padding::None,
                repr: modifier::WeekNumberRepr::Sunday,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"2", component!(WeekNumber {
                padding: modifier::Padding::None,
                repr: modifier::WeekNumberRepr::Monday,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"2", component!(WeekNumber {
                padding: modifier::Padding::None,
                repr: modifier::WeekNumberRepr::Iso,
            }))
        });
    }

    fn parse_component_subsecond(ben: &mut Bencher<'_, CyclesPerByte>) {
        let mut parsed = Parsed::new();
        ben.iter(|| {
            parsed.parse_component(b"1", component!(Subsecond {
                digits: modifier::SubsecondDigits::One,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"12", component!(Subsecond {
                digits: modifier::SubsecondDigits::Two,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"123", component!(Subsecond {
                digits: modifier::SubsecondDigits::Three,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"1234", component!(Subsecond {
                digits: modifier::SubsecondDigits::Four,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"12345", component!(Subsecond {
                digits: modifier::SubsecondDigits::Five,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"123456", component!(Subsecond {
                digits: modifier::SubsecondDigits::Six,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"1234567", component!(Subsecond {
                digits: modifier::SubsecondDigits::Seven,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"12345678", component!(Subsecond {
                digits: modifier::SubsecondDigits::Eight,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"123456789", component!(Subsecond {
                digits: modifier::SubsecondDigits::Nine,
            }))
        });
        ben.iter(|| {
            parsed.parse_component(b"123456789", component!(Subsecond {
                digits: modifier::SubsecondDigits::OneOrMore,
            }))
        });
    }
}
