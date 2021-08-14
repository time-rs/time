use criterion::Bencher;
use criterion_cycles_per_byte::CyclesPerByte;
use time::Month::*;

setup_benchmark! {
    "Month",

    fn previous(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| January.previous());
        ben.iter(|| February.previous());
        ben.iter(|| March.previous());
        ben.iter(|| April.previous());
        ben.iter(|| May.previous());
        ben.iter(|| June.previous());
        ben.iter(|| July.previous());
        ben.iter(|| August.previous());
        ben.iter(|| September.previous());
        ben.iter(|| October.previous());
        ben.iter(|| November.previous());
        ben.iter(|| December.previous());
    }

    fn next(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| January.next());
        ben.iter(|| February.next());
        ben.iter(|| March.next());
        ben.iter(|| April.next());
        ben.iter(|| May.next());
        ben.iter(|| June.next());
        ben.iter(|| July.next());
        ben.iter(|| August.next());
        ben.iter(|| September.next());
        ben.iter(|| October.next());
        ben.iter(|| November.next());
        ben.iter(|| December.next());
    }
}
