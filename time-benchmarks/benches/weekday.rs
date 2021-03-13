use bench_util::setup_benchmark;
use time::Weekday::*;

setup_benchmark! {
    "Weekday",

    fn previous(ben: &mut Bencher) {
        ben.iter(|| (
            Sunday.previous(),
            Monday.previous(),
            Tuesday.previous(),
            Wednesday.previous(),
            Thursday.previous(),
            Friday.previous(),
            Saturday.previous(),
        ));
    }

    fn next(ben: &mut Bencher) {
        ben.iter(|| (
            Sunday.next(),
            Monday.next(),
            Tuesday.next(),
            Wednesday.next(),
            Thursday.next(),
            Friday.next(),
            Saturday.next(),
        ));
    }

    fn number_from_monday(ben: &mut Bencher) {
        ben.iter(|| (
            Monday.number_from_monday(),
            Tuesday.number_from_monday(),
            Wednesday.number_from_monday(),
            Thursday.number_from_monday(),
            Friday.number_from_monday(),
            Saturday.number_from_monday(),
            Sunday.number_from_monday(),
        ));
    }

    fn number_from_sunday(ben: &mut Bencher) {
        ben.iter(|| (
            Sunday.number_from_sunday(),
            Monday.number_from_sunday(),
            Tuesday.number_from_sunday(),
            Wednesday.number_from_sunday(),
            Thursday.number_from_sunday(),
            Friday.number_from_sunday(),
            Saturday.number_from_sunday(),
        ));
    }

    fn number_days_from_monday(ben: &mut Bencher) {
        ben.iter(|| (
            Monday.number_days_from_monday(),
            Tuesday.number_days_from_monday(),
            Wednesday.number_days_from_monday(),
            Thursday.number_days_from_monday(),
            Friday.number_days_from_monday(),
            Saturday.number_days_from_monday(),
            Sunday.number_days_from_monday(),
        ));
    }

    fn number_days_from_sunday(ben: &mut Bencher) {
        ben.iter(|| (
            Sunday.number_days_from_sunday(),
            Monday.number_days_from_sunday(),
            Tuesday.number_days_from_sunday(),
            Wednesday.number_days_from_sunday(),
            Thursday.number_days_from_sunday(),
            Friday.number_days_from_sunday(),
            Saturday.number_days_from_sunday(),
        ));
    }
}
