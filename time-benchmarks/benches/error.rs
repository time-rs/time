use bench_util::setup_benchmark;
use time::{error, Date, Error};

fn component_range() -> error::ComponentRange {
    Date::from_ordinal_date(0, 367).unwrap_err()
}

setup_benchmark! {
    "Error",

    fn display(ben: &mut Bencher) {
        let a = error::ConversionRange;
        let b = Error::ConversionRange;
        let c = component_range();

        ben.iter(|| (
            a.to_string(),
            b.to_string(),
            c.to_string(),
        ));
    }

    fn source(ben: &mut Bencher) {
        use std::error::Error as StdError;

        let a = Error::from(error::ConversionRange);
        let b = Error::from(component_range());

        ben.iter(|| (
            a.source(),
            b.source(),
        ));
    }
}
