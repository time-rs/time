use bench_util::setup_benchmark;
use time::{offset, OffsetDateTime, UtcOffset};

setup_benchmark! {
    "UtcOffset",

    fn hours(ben: &mut Bencher) {
        ben.iter(|| (
            UtcOffset::hours(1),
            UtcOffset::hours(-1),
            UtcOffset::hours(23),
            UtcOffset::hours(-23),
        ));
    }

    fn directional_hours(ben: &mut Bencher) {
        ben.iter(|| (
            UtcOffset::east_hours(1),
            UtcOffset::west_hours(1),
        ));
    }

    fn minutes(ben: &mut Bencher) {
        ben.iter(|| (
            UtcOffset::minutes(1),
            UtcOffset::minutes(-1),
            UtcOffset::minutes(1_439),
            UtcOffset::minutes(-1_439),
        ));
    }

    fn directional_minutes(ben: &mut Bencher) {
        ben.iter(|| (
            UtcOffset::east_minutes(1),
            UtcOffset::west_minutes(1),
        ));
    }

    fn seconds(ben: &mut Bencher) {
        ben.iter(|| (
            UtcOffset::seconds(1),
            UtcOffset::seconds(-1),
            UtcOffset::seconds(86_399),
            UtcOffset::seconds(-86_399),
        ));
    }

    fn directional_seconds(ben: &mut Bencher) {
        ben.iter(|| (
            UtcOffset::east_seconds(1),
            UtcOffset::west_seconds(1),
        ));
    }

    fn as_hours(ben: &mut Bencher) {
        let a = offset!("+1");
        let b = offset!("+0:59");
        let c = offset!("-1");
        let d = offset!("-0:59");
        ben.iter(|| (
            a.as_hours(),
            b.as_hours(),
            c.as_hours(),
            d.as_hours(),
        ));
    }

    fn as_minutes(ben: &mut Bencher) {
        let a = offset!("+1");
        let b = offset!("+0:01");
        let c = offset!("+0:00:59");
        let d = offset!("-1");
        let e = offset!("-0:01");
        let f = offset!("-0:00:59");
        ben.iter(|| (
            a.as_minutes(),
            b.as_minutes(),
            c.as_minutes(),
            d.as_minutes(),
            e.as_minutes(),
            f.as_minutes(),
        ));
    }

    fn as_seconds(ben: &mut Bencher) {
        let a = offset!("+1");
        let b = offset!("+0:01");
        let c = offset!("+0:00:01");
        let d = offset!("-1");
        let e = offset!("-0:01");
        let f = offset!("-0:00:01");
        ben.iter(|| (
            a.as_seconds(),
            b.as_seconds(),
            c.as_seconds(),
            d.as_seconds(),
            e.as_seconds(),
            f.as_seconds(),
        ));
    }

    fn format(ben: &mut Bencher) {
        let a = offset!("+1");
        let b = offset!("-1");
        let c = offset!("+0");
        let d = offset!("-0");
        let e = offset!("+0:01");
        let f = offset!("-0:01");
        let g = offset!("+0:00:01");
        let h = offset!("-0:00:01");

        ben.iter(|| (
            a.format("%z"),
            b.format("%z"),
            c.format("%z"),
            d.format("%z"),
            e.format("%z"),
            f.format("%z"),
            g.format("%z"),
            h.format("%z"),
        ));
    }

    fn parse(ben: &mut Bencher) {
        ben.iter(|| (
            UtcOffset::parse("+0100", "%z"),
            UtcOffset::parse("-0100", "%z"),
            UtcOffset::parse("+0000", "%z"),
            UtcOffset::parse("-0000", "%z"),
            UtcOffset::parse("+0001", "%z"),
            UtcOffset::parse("-0001", "%z"),
        ));
    }

    fn display(ben: &mut Bencher) {
        let a = offset!("UTC");
        let b = offset!("+0:00:01");
        let c = offset!("-0:00:01");
        let d = offset!("+1");
        let e = offset!("-1");
        let f = offset!("+23:59");
        let g = offset!("-23:59");
        let h = offset!("+23:59:59");
        let i = offset!("-23:59:59");

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
        ));
    }

    fn local_offset_at(ben: &mut Bencher) {
        let epoch = OffsetDateTime::unix_epoch();
        ben.iter(|| UtcOffset::local_offset_at(epoch));
    }

    fn current_local_offset(ben: &mut Bencher) {
        ben.iter(UtcOffset::current_local_offset);
    }
}
