use bench_util::setup_benchmark;
use time::error;

fn component_range() -> error::ComponentRange {
    time::Date::from_yo(0, 367).unwrap_err()
}

setup_benchmark! {
    "Parse error",

    fn display(ben: &mut Bencher) {
        let a = error::Parse::InvalidNanosecond;
        let b = error::Parse::InvalidSecond;
        let c = error::Parse::InvalidMinute;
        let d = error::Parse::InvalidHour;
        let e = error::Parse::InvalidAmPm;
        let f = error::Parse::InvalidMonth;
        let g = error::Parse::InvalidYear;
        let h = error::Parse::InvalidWeek;
        let i = error::Parse::InvalidDayOfWeek;
        let j = error::Parse::InvalidDayOfMonth;
        let k = error::Parse::InvalidDayOfYear;
        let l = error::Parse::InvalidOffset;
        let m = error::Parse::MissingFormatSpecifier;
        let n = error::Parse::InvalidFormatSpecifier('!');
        let o = error::Parse::UnexpectedCharacter {
            expected: 'a',
            actual: 'b',
        };
        let p = error::Parse::UnexpectedEndOfString;
        let q = error::Parse::InsufficientInformation;
        let r = error::Parse::ComponentOutOfRange(Box::new(component_range()));

        ben.iter(|| (
            a.to_string(),
            b.to_string(),
            c.to_string(),
            d.to_string(),
            e.to_string(),
            f.to_string(),
            g.to_string(),
            h.to_string(),
            i.to_string(),
            j.to_string(),
            k.to_string(),
            l.to_string(),
            m.to_string(),
            n.to_string(),
            o.to_string(),
            p.to_string(),
            q.to_string(),
            r.to_string(),
        ));
    }

    fn source(ben: &mut Bencher) {
        use std::error::Error as StdError;
        let a = error::Parse::from(component_range());
        let b = error::Parse::InvalidNanosecond;

        ben.iter(|| (
            a.source(),
            b.source(),
        ));
    }
}
