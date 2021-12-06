use std::io::{self, ErrorKind};

use time::format_description::well_known::{Rfc2822, Rfc3339};
use time::format_description::{self, FormatItem};
use time::macros::{date, datetime, format_description as fd, offset, time};
use time::{OffsetDateTime, Time};

#[test]
fn rfc_2822() -> time::Result<()> {
    assert_eq!(
        datetime!(2021-01-02 03:04:05 UTC).format(&Rfc2822)?,
        "Sat, 02 Jan 2021 03:04:05 +0000"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05 +06:07).format(&Rfc2822)?,
        "Sat, 02 Jan 2021 03:04:05 +0607"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05 -06:07).format(&Rfc2822)?,
        "Sat, 02 Jan 2021 03:04:05 -0607"
    );

    assert!(matches!(
        datetime!(1885-01-01 01:01:01 UTC).format(&Rfc2822),
        Err(time::error::Format::InvalidComponent("year"))
    ));
    assert!(matches!(
        datetime!(2000-01-01 00:00:00 +00:00:01).format(&Rfc2822),
        Err(time::error::Format::InvalidComponent("offset_second"))
    ));

    Ok(())
}

#[test]
fn rfc_3339() -> time::Result<()> {
    assert_eq!(
        datetime!(2021-01-02 03:04:05 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.1 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.1Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.12 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.12Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_4 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.1234Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_45 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.12345Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123456Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_7 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.1234567Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_78 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.12345678Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_789 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123456789Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_789 -01:02).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123456789-01:02"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_789 +01:02).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123456789+01:02"
    );

    assert!(matches!(
        datetime!(-0001-01-01 0:00 UTC).format(&Rfc3339),
        Err(time::error::Format::InvalidComponent("year"))
    ));
    assert!(matches!(
        datetime!(0000-01-01 0:00 +00:00:01).format(&Rfc3339),
        Err(time::error::Format::InvalidComponent("offset_second"))
    ));

    Ok(())
}

#[test]
fn format_time() -> time::Result<()> {
    let format_output = [
        (fd!("[hour]"), "13"),
        (fd!("[hour repr:12]"), "01"),
        (fd!("[hour repr:12 padding:none]"), "1"),
        (fd!("[hour repr:12 padding:space]"), " 1"),
        (fd!("[hour repr:24]"), "13"),
        (fd!("[hour repr:24]"), "13"),
        (fd!("[hour repr:24 padding:none]"), "13"),
        (fd!("[hour repr:24 padding:space]"), "13"),
        (fd!("[minute]"), "02"),
        (fd!("[minute padding:none]"), "2"),
        (fd!("[minute padding:space]"), " 2"),
        (fd!("[minute padding:zero]"), "02"),
        (fd!("[period]"), "PM"),
        (fd!("[period case:upper]"), "PM"),
        (fd!("[period case:lower]"), "pm"),
        (fd!("[second]"), "03"),
        (fd!("[second padding:none]"), "3"),
        (fd!("[second padding:space]"), " 3"),
        (fd!("[second padding:zero]"), "03"),
        (fd!("[subsecond]"), "456789012"),
        (fd!("[subsecond digits:1]"), "4"),
        (fd!("[subsecond digits:2]"), "45"),
        (fd!("[subsecond digits:3]"), "456"),
        (fd!("[subsecond digits:4]"), "4567"),
        (fd!("[subsecond digits:5]"), "45678"),
        (fd!("[subsecond digits:6]"), "456789"),
        (fd!("[subsecond digits:7]"), "4567890"),
        (fd!("[subsecond digits:8]"), "45678901"),
        (fd!("[subsecond digits:9]"), "456789012"),
        (fd!("[subsecond digits:1+]"), "456789012"),
    ];

    for &(format_description, output) in &format_output {
        assert_eq!(
            time!(13:02:03.456_789_012).format(format_description)?,
            output
        );
        assert!(
            time!(13:02:03.456_789_012)
                .format_into(&mut io::sink(), format_description)
                .is_ok()
        );
        let mut buf = String::new();
        assert!(
            time!(13:02:03.456_789_012)
                .format_into_fmt_writer(&mut buf, format_description)
                .is_ok()
        );
    }

    assert_eq!(
        time!(1:02:03).format(fd!("[hour repr:12][period]"))?,
        "01AM"
    );
    assert_eq!(
        Time::MIDNIGHT.format(fd!("[hour repr:12][period case:lower]"))?,
        "12am"
    );
    assert_eq!(Time::MIDNIGHT.format(fd!("[subsecond digits:1+]"))?, "0");
    assert_eq!(
        time!(0:00:00.01).format(fd!("[subsecond digits:1+]"))?,
        "01"
    );
    assert_eq!(
        time!(0:00:00.001).format(fd!("[subsecond digits:1+]"))?,
        "001"
    );
    assert_eq!(
        time!(0:00:00.0001).format(fd!("[subsecond digits:1+]"))?,
        "0001"
    );
    assert_eq!(
        time!(0:00:00.00001).format(fd!("[subsecond digits:1+]"))?,
        "00001"
    );
    assert_eq!(
        time!(0:00:00.000001).format(fd!("[subsecond digits:1+]"))?,
        "000001"
    );
    assert_eq!(
        time!(0:00:00.0000001).format(fd!("[subsecond digits:1+]"))?,
        "0000001"
    );
    assert_eq!(
        time!(0:00:00.00000001).format(fd!("[subsecond digits:1+]"))?,
        "00000001"
    );
    assert_eq!(
        time!(0:00:00.000000001).format(fd!("[subsecond digits:1+]"))?,
        "000000001"
    );

    Ok(())
}

#[test]
fn display_time() {
    assert_eq!(time!(0:00).to_string(), "0:00:00.0");
    assert_eq!(time!(23:59).to_string(), "23:59:00.0");
    assert_eq!(time!(23:59:59).to_string(), "23:59:59.0");
    assert_eq!(time!(0:00:01).to_string(), "0:00:01.0");
    assert_eq!(time!(0:00:00.1).to_string(), "0:00:00.1");
    assert_eq!(time!(0:00:00.01).to_string(), "0:00:00.01");
    assert_eq!(time!(0:00:00.001).to_string(), "0:00:00.001");
    assert_eq!(time!(0:00:00.000_1).to_string(), "0:00:00.0001");
    assert_eq!(time!(0:00:00.000_01).to_string(), "0:00:00.00001");
    assert_eq!(time!(0:00:00.000_001).to_string(), "0:00:00.000001");
    assert_eq!(time!(0:00:00.000_000_1).to_string(), "0:00:00.0000001");
    assert_eq!(time!(0:00:00.000_000_01).to_string(), "0:00:00.00000001");
    assert_eq!(time!(0:00:00.000_000_001).to_string(), "0:00:00.000000001");
}

#[test]
fn format_date() -> time::Result<()> {
    let format_output = [
        (fd!("[day]"), "31"),
        (fd!("[month]"), "12"),
        (fd!("[month repr:short]"), "Dec"),
        (fd!("[month repr:long]"), "December"),
        (fd!("[ordinal]"), "365"),
        (fd!("[weekday]"), "Tuesday"),
        (fd!("[weekday repr:short]"), "Tue"),
        (fd!("[weekday repr:sunday]"), "3"),
        (fd!("[weekday repr:sunday one_indexed:false]"), "2"),
        (fd!("[weekday repr:monday]"), "2"),
        (fd!("[weekday repr:monday one_indexed:false]"), "1"),
        (fd!("[week_number]"), "01"),
        (fd!("[week_number padding:none]"), "1"),
        (fd!("[week_number padding:space]"), " 1"),
        (fd!("[week_number repr:sunday]"), "52"),
        (fd!("[week_number repr:monday]"), "52"),
        (fd!("[year]"), "2019"),
        (fd!("[year base:iso_week]"), "2020"),
        (fd!("[year sign:mandatory]"), "+2019"),
        (fd!("[year base:iso_week sign:mandatory]"), "+2020"),
        (fd!("[year repr:last_two]"), "19"),
        (fd!("[year base:iso_week repr:last_two]"), "20"),
    ];

    for &(format_description, output) in &format_output {
        assert_eq!(date!(2019 - 12 - 31).format(format_description)?, output);
        assert!(
            date!(2019 - 12 - 31)
                .format_into(&mut io::sink(), format_description)
                .is_ok()
        );
        let mut buf = String::new();
        assert!(
            date!(2019 - 12 - 31)
                .format_into_fmt_writer(&mut buf, format_description)
                .is_ok()
        );
    }

    Ok(())
}

#[test]
fn display_date() {
    assert_eq!(date!(2019 - 01 - 01).to_string(), "2019-01-01");
    assert_eq!(date!(2019 - 12 - 31).to_string(), "2019-12-31");
    assert_eq!(date!(-4713 - 11 - 24).to_string(), "-4713-11-24");
    assert_eq!(date!(-0001 - 01 - 01).to_string(), "-0001-01-01");

    assert_eq!(date!(+10_000-01-01).to_string(), "+10000-01-01");
    assert_eq!(date!(+100_000-01-01).to_string(), "+100000-01-01");
    assert_eq!(date!(-10_000 - 01 - 01).to_string(), "-10000-01-01");
    assert_eq!(date!(-100_000 - 01 - 01).to_string(), "-100000-01-01");
}

#[test]
fn format_offset() -> time::Result<()> {
    let value_format_output = [
        (
            offset!(+01:02:03),
            fd!("[offset_hour sign:automatic]"),
            "01",
        ),
        (
            offset!(+01:02:03),
            fd!("[offset_hour sign:mandatory]"),
            "+01",
        ),
        (
            offset!(-01:02:03),
            fd!("[offset_hour sign:automatic]"),
            "-01",
        ),
        (
            offset!(-01:02:03),
            fd!("[offset_hour sign:mandatory]"),
            "-01",
        ),
        (offset!(+01:02:03), fd!("[offset_minute]"), "02"),
        (offset!(+01:02:03), fd!("[offset_second]"), "03"),
    ];

    for &(value, format_description, output) in &value_format_output {
        assert_eq!(value.format(format_description)?, output);
        assert!(
            value
                .format_into(&mut io::sink(), format_description)
                .is_ok()
        );
        let mut buf = String::new();
        assert!(
            value
                .format_into_fmt_writer(&mut buf, format_description)
                .is_ok()
        );
    }

    Ok(())
}

#[test]
fn display_offset() {
    assert_eq!(offset!(UTC).to_string(), "+00:00:00");
    assert_eq!(offset!(+0:00:01).to_string(), "+00:00:01");
    assert_eq!(offset!(-0:00:01).to_string(), "-00:00:01");
    assert_eq!(offset!(+1).to_string(), "+01:00:00");
    assert_eq!(offset!(-1).to_string(), "-01:00:00");
    assert_eq!(offset!(+23:59).to_string(), "+23:59:00");
    assert_eq!(offset!(-23:59).to_string(), "-23:59:00");
    assert_eq!(offset!(+23:59:59).to_string(), "+23:59:59");
    assert_eq!(offset!(-23:59:59).to_string(), "-23:59:59");
}

#[test]
fn format_pdt() -> time::Result<()> {
    let format_description = fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");

    assert_eq!(
        datetime!(1970-01-01 0:00).format(format_description)?,
        "1970-01-01 00:00:00.0"
    );
    assert!(
        datetime!(1970-01-01 0:00)
            .format_into(&mut io::sink(), format_description)
            .is_ok()
    );
    let mut buf = String::new();
    assert!(
        datetime!(1970-01-01 0:00)
            .format_into_fmt_writer(&mut buf, format_description)
            .is_ok()
    );

    Ok(())
}

#[test]
fn display_pdt() {
    assert_eq!(
        datetime!(1970-01-01 0:00).to_string(),
        String::from("1970-01-01 0:00:00.0")
    );
    assert_eq!(
        datetime!(1970-01-01 0:00:01).to_string(),
        String::from("1970-01-01 0:00:01.0")
    );
}

#[test]
fn format_odt() -> time::Result<()> {
    let format_description = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:1+] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]",
    )?;

    assert_eq!(
        datetime!(1970-01-01 0:00 UTC).format(&format_description)?,
        "1970-01-01 00:00:00.0 +00:00:00"
    );
    assert!(
        datetime!(1970-01-01 0:00 UTC)
            .format_into(&mut io::sink(), &format_description)
            .is_ok()
    );
    let mut buf = String::new();
    assert!(
        datetime!(1970-01-01 0:00 UTC)
            .format_into_fmt_writer(&mut buf, &format_description)
            .is_ok()
    );

    Ok(())
}

#[test]
fn display_odt() {
    assert_eq!(
        datetime!(1970-01-01 0:00 UTC).to_string(),
        "1970-01-01 0:00:00.0 +00:00:00"
    );
}

#[test]
fn insufficient_type_information() {
    let assert_insufficient_type_information = |res| {
        assert!(matches!(
            res,
            Err(time::error::Format::InsufficientTypeInformation { .. })
        ));
    };
    assert_insufficient_type_information(Time::MIDNIGHT.format(fd!("[year]")));
    assert_insufficient_type_information(Time::MIDNIGHT.format(&Rfc3339));
    assert_insufficient_type_information(date!(2021 - 001).format(&Rfc3339));
    assert_insufficient_type_information(datetime!(2021 - 001 0:00).format(&Rfc3339));
    assert_insufficient_type_information(Time::MIDNIGHT.format(&Rfc2822));
    assert_insufficient_type_information(date!(2021 - 001).format(&Rfc2822));
    assert_insufficient_type_information(datetime!(2021 - 001 0:00).format(&Rfc2822));
    assert_insufficient_type_information(
        Time::MIDNIGHT.format(&FormatItem::First(&[FormatItem::Compound(fd!("[year]"))])),
    );
}

#[test]
fn failed_write() -> time::Result<()> {
    /// Allow at most `$n` bytes to be written.
    macro_rules! bytes {
        ($n:literal) => {
            &mut &mut [0; $n][..]
        };
    }

    let assert_err = |res| {
        assert!(
            matches!(res, Err(time::error::Format::StdIo(e)) if e.kind() == ErrorKind::WriteZero)
        );
    };
    assert_err(Time::MIDNIGHT.format_into(bytes!(0), fd!("foo")));
    assert_err(Time::MIDNIGHT.format_into(bytes!(0), &FormatItem::Compound(fd!("foo"))));
    assert_err(Time::MIDNIGHT.format_into(
        bytes!(0),
        &FormatItem::Optional(&FormatItem::Compound(fd!("foo"))),
    ));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(0), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(4), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(5), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(7), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(8), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(10), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(11), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(13), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(14), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(16), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(17), &Rfc3339));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(19), &Rfc3339));
    assert_err(datetime!(2021-001 0:00:00.1 UTC).format_into(bytes!(19), &Rfc3339));
    assert_err(datetime!(2021-001 0:00:00.1 UTC).format_into(bytes!(20), &Rfc3339));
    assert_err(datetime!(2021-001 0:00 +0:01).format_into(bytes!(19), &Rfc3339));
    assert_err(datetime!(2021-001 0:00 +0:01).format_into(bytes!(20), &Rfc3339));
    assert_err(datetime!(2021-001 0:00 +0:01).format_into(bytes!(22), &Rfc3339));
    assert_err(datetime!(2021-001 0:00 +0:01).format_into(bytes!(23), &Rfc3339));

    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(0), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(4), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(6), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(7), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(8), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(11), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(12), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(16), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(17), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(19), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(20), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(22), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(23), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(25), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(26), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(27), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(29), &Rfc2822));
    assert_err(OffsetDateTime::UNIX_EPOCH.format_into(bytes!(30), &Rfc2822));

    assert_err(Time::MIDNIGHT.format_into(bytes!(0), fd!("[hour padding:space]")));
    assert_err(Time::MIDNIGHT.format_into(bytes!(1), fd!("[hour padding:space]")));
    assert_err(offset!(+1).format_into(bytes!(0), fd!("[offset_hour sign:mandatory]")));
    assert_err(offset!(-1).format_into(bytes!(0), fd!("[offset_hour]")));
    assert_err(offset!(-1).format_into(bytes!(1), fd!("[offset_hour]")));
    assert_err(date!(-1 - 001).format_into(bytes!(0), fd!("[year]")));
    assert_err(date!(2021 - 001).format_into(bytes!(0), fd!("[year sign:mandatory]")));
    assert_err(date!(+999_999 - 001).format_into(bytes!(4), fd!("[year]")));
    assert_err(date!(+99_999 - 001).format_into(bytes!(4), fd!("[year]")));

    let component_names = [
        "day",
        "month",
        "ordinal",
        "weekday",
        "week_number",
        "year",
        "hour",
        "minute",
        "period",
        "second",
        "subsecond",
        "offset_hour",
        "offset_minute",
        "offset_second",
    ];
    for component in &component_names {
        assert_err(OffsetDateTime::UNIX_EPOCH.format_into(
            bytes!(0),
            &format_description::parse(&format!("[{}]", component))?,
        ));
    }

    Ok(())
}

#[test]
fn first() -> time::Result<()> {
    assert_eq!(Time::MIDNIGHT.format(&FormatItem::First(&[]))?, "");
    assert_eq!(
        Time::MIDNIGHT.format(&FormatItem::First(&[FormatItem::Compound(fd!("[hour]"))]))?,
        "00"
    );

    Ok(())
}
