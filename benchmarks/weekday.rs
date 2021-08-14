use criterion::Bencher;
use criterion_cycles_per_byte::CyclesPerByte;
use time::Weekday::*;

setup_benchmark! {
    "Weekday",

    fn previous(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Sunday.previous());
        ben.iter(|| Monday.previous());
        ben.iter(|| Tuesday.previous());
        ben.iter(|| Wednesday.previous());
        ben.iter(|| Thursday.previous());
        ben.iter(|| Friday.previous());
        ben.iter(|| Saturday.previous());
    }

    fn next(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Sunday.next());
        ben.iter(|| Monday.next());
        ben.iter(|| Tuesday.next());
        ben.iter(|| Wednesday.next());
        ben.iter(|| Thursday.next());
        ben.iter(|| Friday.next());
        ben.iter(|| Saturday.next());
    }

    fn number_from_monday(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Monday.number_from_monday());
        ben.iter(|| Tuesday.number_from_monday());
        ben.iter(|| Wednesday.number_from_monday());
        ben.iter(|| Thursday.number_from_monday());
        ben.iter(|| Friday.number_from_monday());
        ben.iter(|| Saturday.number_from_monday());
        ben.iter(|| Sunday.number_from_monday());
    }

    fn number_from_sunday(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Sunday.number_from_sunday());
        ben.iter(|| Monday.number_from_sunday());
        ben.iter(|| Tuesday.number_from_sunday());
        ben.iter(|| Wednesday.number_from_sunday());
        ben.iter(|| Thursday.number_from_sunday());
        ben.iter(|| Friday.number_from_sunday());
        ben.iter(|| Saturday.number_from_sunday());
    }

    fn number_days_from_monday(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Monday.number_days_from_monday());
        ben.iter(|| Tuesday.number_days_from_monday());
        ben.iter(|| Wednesday.number_days_from_monday());
        ben.iter(|| Thursday.number_days_from_monday());
        ben.iter(|| Friday.number_days_from_monday());
        ben.iter(|| Saturday.number_days_from_monday());
        ben.iter(|| Sunday.number_days_from_monday());
    }

    fn number_days_from_sunday(ben: &mut Bencher<'_, CyclesPerByte>) {
        ben.iter(|| Sunday.number_days_from_sunday());
        ben.iter(|| Monday.number_days_from_sunday());
        ben.iter(|| Tuesday.number_days_from_sunday());
        ben.iter(|| Wednesday.number_days_from_sunday());
        ben.iter(|| Thursday.number_days_from_sunday());
        ben.iter(|| Friday.number_days_from_sunday());
        ben.iter(|| Saturday.number_days_from_sunday());
    }
}
