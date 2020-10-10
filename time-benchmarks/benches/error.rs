use bench_util::setup_benchmark;
use time::{error, Date, Error};

fn component_range() -> error::ComponentRange {
    Date::from_yo(0, 367).unwrap_err()
}

fn parse() -> error::Parse {
    Date::parse("", " ").unwrap_err()
}

fn format_insufficient() -> error::Format {
    error::Format::InsufficientTypeInformation
}

fn format_std() -> error::Format {
    std::fmt::Error.into()
}

setup_benchmark! {
    "Error",

    fn display(ben: &mut Bencher) {
        let a = error::ConversionRange;
        let b = Error::ConversionRange;
        let c = component_range();
        let d = Error::ComponentRange(component_range().into());
        let e = parse();
        let f = Error::Parse(parse());
        let g = format_insufficient();
        let h = Error::Format(format_insufficient());
        let i = format_std();
        let j = Error::Format(format_std());

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
        ));
    }

    fn source(ben: &mut Bencher) {
        use std::error::Error as StdError;

        let a = Error::from(error::ConversionRange);
        let b = Error::from(component_range());
        let c = Error::from(parse());
        let d = Error::from(format_insufficient());
        let e = format_insufficient();
        let f = format_std();

        ben.iter(|| (
            a.source(),
            b.source(),
            c.source(),
            d.source(),
            e.source(),
            f.source(),
        ));
    }
}
