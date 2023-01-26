use criterion::Bencher;
use time::Weekday::*;

setup_benchmark! {
    "Weekday",

    fn previous(ben: &mut Bencher<'_>) {
        ben.iter(|| Sunday.previous());
        ben.iter(|| Monday.previous());
        ben.iter(|| Tuesday.previous());
        ben.iter(|| Wednesday.previous());
        ben.iter(|| Thursday.previous());
        ben.iter(|| Friday.previous());
        ben.iter(|| Saturday.previous());
    }

    fn next(ben: &mut Bencher<'_>) {
        ben.iter(|| Sunday.next());
        ben.iter(|| Monday.next());
        ben.iter(|| Tuesday.next());
        ben.iter(|| Wednesday.next());
        ben.iter(|| Thursday.next());
        ben.iter(|| Friday.next());
        ben.iter(|| Saturday.next());
    }

    fn nth(ben: &mut Bencher<'_>) {
        ben.iter(|| Sunday.nth(0));
        ben.iter(|| Sunday.nth(1));
        ben.iter(|| Sunday.nth(2));
        ben.iter(|| Sunday.nth(3));
        ben.iter(|| Sunday.nth(4));
        ben.iter(|| Sunday.nth(5));
        ben.iter(|| Sunday.nth(6));

        ben.iter(|| Monday.nth(0));
        ben.iter(|| Monday.nth(1));
        ben.iter(|| Monday.nth(2));
        ben.iter(|| Monday.nth(3));
        ben.iter(|| Monday.nth(4));
        ben.iter(|| Monday.nth(5));
        ben.iter(|| Monday.nth(6));

        ben.iter(|| Sunday.nth(7));
        ben.iter(|| Sunday.nth(u8::MAX));
        ben.iter(|| Monday.nth(7));
        ben.iter(|| Monday.nth(u8::MAX));
    }

    fn number_from_monday(ben: &mut Bencher<'_>) {
        ben.iter(|| Monday.number_from_monday());
        ben.iter(|| Tuesday.number_from_monday());
        ben.iter(|| Wednesday.number_from_monday());
        ben.iter(|| Thursday.number_from_monday());
        ben.iter(|| Friday.number_from_monday());
        ben.iter(|| Saturday.number_from_monday());
        ben.iter(|| Sunday.number_from_monday());
    }

    fn number_from_sunday(ben: &mut Bencher<'_>) {
        ben.iter(|| Sunday.number_from_sunday());
        ben.iter(|| Monday.number_from_sunday());
        ben.iter(|| Tuesday.number_from_sunday());
        ben.iter(|| Wednesday.number_from_sunday());
        ben.iter(|| Thursday.number_from_sunday());
        ben.iter(|| Friday.number_from_sunday());
        ben.iter(|| Saturday.number_from_sunday());
    }

    fn number_days_from_monday(ben: &mut Bencher<'_>) {
        ben.iter(|| Monday.number_days_from_monday());
        ben.iter(|| Tuesday.number_days_from_monday());
        ben.iter(|| Wednesday.number_days_from_monday());
        ben.iter(|| Thursday.number_days_from_monday());
        ben.iter(|| Friday.number_days_from_monday());
        ben.iter(|| Saturday.number_days_from_monday());
        ben.iter(|| Sunday.number_days_from_monday());
    }

    fn number_days_from_sunday(ben: &mut Bencher<'_>) {
        ben.iter(|| Sunday.number_days_from_sunday());
        ben.iter(|| Monday.number_days_from_sunday());
        ben.iter(|| Tuesday.number_days_from_sunday());
        ben.iter(|| Wednesday.number_days_from_sunday());
        ben.iter(|| Thursday.number_days_from_sunday());
        ben.iter(|| Friday.number_days_from_sunday());
        ben.iter(|| Saturday.number_days_from_sunday());
    }
}
