use std::io;

use criterion::Bencher;
use time::format_description;
use time::format_description::well_known::Rfc3339;
use time::macros::{date, datetime, format_description as fd, offset, time};

setup_benchmark! {
    "Formatting",

    fn rfc_3339(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($value:expr) => {
                $value.format_into(&mut io::sink(), &Rfc3339)
            }
        }

        ben.iter(|| (
            item!(datetime!("2021-01-02 03:04:05 UTC")),
            item!(datetime!("2021-01-02 03:04:05.1 UTC")),
            item!(datetime!("2021-01-02 03:04:05.12 UTC")),
            item!(datetime!("2021-01-02 03:04:05.123 UTC")),
            item!(datetime!("2021-01-02 03:04:05.123_4 UTC")),
            item!(datetime!("2021-01-02 03:04:05.123_45 UTC")),
            item!(datetime!("2021-01-02 03:04:05.123_456 UTC")),
            item!(datetime!("2021-01-02 03:04:05.123_456_7 UTC")),
            item!(datetime!("2021-01-02 03:04:05.123_456_78 UTC")),
            item!(datetime!("2021-01-02 03:04:05.123_456_789 UTC")),
            item!(datetime!("2021-01-02 03:04:05.123_456_789 -01:02")),
            item!(datetime!("2021-01-02 03:04:05.123_456_789 +01:02")),
        ));
    }

    fn format_time(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($format:expr) => {
                time!("13:02:03.456_789_012").format_into(
                    &mut io::sink(),
                    &$format,
                )
            }
        }

        ben.iter(|| (
            item!(fd!("[hour]")),
            item!(fd!("[hour repr:12]")),
            item!(fd!("[hour repr:12 padding:none]")),
            item!(fd!("[hour repr:12 padding:space]")),
            item!(fd!("[hour repr:24]")),
            item!(fd!("[hour repr:24]")),
            item!(fd!("[hour repr:24 padding:none]")),
            item!(fd!("[hour repr:24 padding:space]")),
            item!(fd!("[minute]")),
            item!(fd!("[minute padding:none]")),
            item!(fd!("[minute padding:space]")),
            item!(fd!("[minute padding:zero]")),
            item!(fd!("[period]")),
            item!(fd!("[period case:upper]")),
            item!(fd!("[period case:lower]")),
            item!(fd!("[second]")),
            item!(fd!("[second padding:none]")),
            item!(fd!("[second padding:space]")),
            item!(fd!("[second padding:zero]")),
            item!(fd!("[subsecond]")),
            item!(fd!("[subsecond digits:1]")),
            item!(fd!("[subsecond digits:2]")),
            item!(fd!("[subsecond digits:3]")),
            item!(fd!("[subsecond digits:4]")),
            item!(fd!("[subsecond digits:5]")),
            item!(fd!("[subsecond digits:6]")),
            item!(fd!("[subsecond digits:7]")),
            item!(fd!("[subsecond digits:8]")),
            item!(fd!("[subsecond digits:9]")),
            item!(fd!("[subsecond digits:1+]")),
        ));
    }

    fn display_time(ben: &mut Bencher<'_>) {
        ben.iter(|| (
            time!("0:00").to_string(),
            time!("23:59").to_string(),
            time!("23:59:59").to_string(),
            time!("0:00:01").to_string(),
            time!("0:00:00.001").to_string(),
            time!("0:00:00.000_001").to_string(),
            time!("0:00:00.000_000_001").to_string(),
        ));
    }

    fn format_date(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($format:expr) => {
                date!("2019-12-31").format_into(&mut io::sink(), &$format)
            }
        }

        ben.iter(|| (
            item!(fd!("[day]")),
            item!(fd!("[month]")),
            item!(fd!("[month repr:short]")),
            item!(fd!("[month repr:long]")),
            item!(fd!("[ordinal]")),
            item!(fd!("[weekday]")),
            item!(fd!("[weekday repr:short]")),
            item!(fd!("[weekday repr:sunday]")),
            item!(fd!("[weekday repr:sunday one_indexed:false]")),
            item!(fd!("[weekday repr:monday]")),
            item!(fd!("[weekday repr:monday one_indexed:false]")),
            item!(fd!("[week_number]")),
            item!(fd!("[week_number padding:none]")),
            item!(fd!("[week_number padding:space]")),
            item!(fd!("[week_number repr:sunday]")),
            item!(fd!("[week_number repr:monday]")),
            item!(fd!("[year]")),
            item!(fd!("[year base:iso_week]")),
            item!(fd!("[year sign:mandatory]")),
            item!(fd!("[year base:iso_week sign:mandatory]")),
            item!(fd!("[year repr:last_two]")),
            item!(fd!("[year base:iso_week repr:last_two]")),
        ));
    }

    fn display_date(ben: &mut Bencher<'_>) {
        ben.iter(|| (
            date!("2019-01-01").to_string(),
            date!("2019-12-31").to_string(),
            date!("-4713-11-24").to_string(),
            date!("-0001-01-01").to_string(),
        ));
    }

    fn format_offset(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($value:expr, $format:expr) => {
                $value.format_into(&mut io::sink(), &$format)
            }
        }

        ben.iter(|| (
            item!(offset!("+01:02:03"), fd!("[offset_hour sign:automatic]")),
            item!(offset!("+01:02:03"), fd!("[offset_hour sign:mandatory]")),
            item!(offset!("-01:02:03"), fd!("[offset_hour sign:automatic]")),
            item!(offset!("-01:02:03"), fd!("[offset_hour sign:mandatory]")),
            item!(offset!("+01:02:03"), fd!("[offset_minute]")),
            item!(offset!("+01:02:03"), fd!("[offset_second]")),
        ));
    }

    fn display_offset(ben: &mut Bencher<'_>) {
        ben.iter(|| (
            offset!("UTC").to_string(),
            offset!("+0:00:01").to_string(),
            offset!("-0:00:01").to_string(),
            offset!("+1").to_string(),
            offset!("-1").to_string(),
            offset!("+23:59").to_string(),
            offset!("-23:59").to_string(),
            offset!("+23:59:59").to_string(),
            offset!("-23:59:59").to_string(),
        ));
    }

    fn format_pdt(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            datetime!("1970-01-01 0:00").format_into(
                &mut io::sink(),
                &fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
            )
        });
    }

    fn display_pdt(ben: &mut Bencher<'_>) {
        ben.iter(|| (
            datetime!("1970-01-01 0:00").to_string(),
            datetime!("1970-01-01 0:00:01").to_string(),
        ));
    }

    fn format_odt(ben: &mut Bencher<'_>) {
        // We can't currently handle escaped line breaks in the format description macro.
        let format_description = format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond] [offset_hour \
            sign:mandatory]:[offset_minute]:[offset_second]",
        ).expect("invalid format description");

        ben.iter(|| {
            datetime!("1970-01-01 0:00 UTC").format_into(&mut io::sink(), &format_description)
        });
    }

    fn display_odt(ben: &mut Bencher<'_>) {
        ben.iter(|| datetime!("1970-01-01 0:00 UTC").to_string());
    }
}
