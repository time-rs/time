use criterion::Bencher;
use criterion_cycles_per_byte::CyclesPerByte;
use time::{OffsetDateTime, UtcOffset};

setup_benchmark! {
    "UtcOffset",

    // region: constructors
    fn from_hms(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::from_hms(0, 0, 0));
    }

    fn from_whole_seconds(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::from_whole_seconds(0));
    }
    // endregion constructors

    // region: getters
    fn as_hms(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.as_hms());
    }

    fn whole_hours(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.whole_hours());
    }

    fn whole_minutes(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.whole_minutes());
    }

    fn minutes_past_hour(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.minutes_past_hour());
    }

    fn whole_seconds(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.whole_seconds());
    }

    fn seconds_past_minute(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.seconds_past_minute());
    }
    // endregion getters

    // region: is_{sign}
    fn is_utc(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.is_utc());
    }

    fn is_positive(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.is_positive());
    }

    fn is_negative(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::UTC.is_negative());
    }
    // endregion is_{sign}

    fn local_offset_at(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH));
    }

    fn current_local_offset(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(UtcOffset::current_local_offset);
    }
}
