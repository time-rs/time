use bench_util::setup_benchmark;
use time::{macros::offset, OffsetDateTime, UtcOffset};

setup_benchmark! {
    "UtcOffset",

    fn from_hms(ben: &mut Bencher) {
        ben.iter(|| (
            UtcOffset::from_hms(0, 0, 0),
            UtcOffset::from_hms(0, 0, 1),
            UtcOffset::from_hms(0, 0, -1),
            UtcOffset::from_hms(1, 0, 0),
            UtcOffset::from_hms(-1, 0, 0),
            UtcOffset::from_hms(23, 59, 0),
            UtcOffset::from_hms(-23, -59, 0),
            UtcOffset::from_hms(23, 59, 59),
            UtcOffset::from_hms(-23, -59, -59),
        ));
    }

    fn as_hms(ben: &mut Bencher) {
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
            a.as_hms(),
            b.as_hms(),
            c.as_hms(),
            d.as_hms(),
            e.as_hms(),
            f.as_hms(),
            g.as_hms(),
            h.as_hms(),
            i.as_hms(),
        ));
    }

    fn to_seconds(ben: &mut Bencher) {
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
            a.to_seconds(),
            b.to_seconds(),
            c.to_seconds(),
            d.to_seconds(),
            e.to_seconds(),
            f.to_seconds(),
            g.to_seconds(),
            h.to_seconds(),
            i.to_seconds(),
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
