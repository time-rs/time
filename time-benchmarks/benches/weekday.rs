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

    fn iso_weekday_number(ben: &mut Bencher) {
        ben.iter(|| (
            Monday.iso_weekday_number(),
            Tuesday.iso_weekday_number(),
            Wednesday.iso_weekday_number(),
            Thursday.iso_weekday_number(),
            Friday.iso_weekday_number(),
            Saturday.iso_weekday_number(),
            Sunday.iso_weekday_number(),
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

    fn display(ben: &mut Bencher) {
        ben.iter(|| (
            Monday.to_string(),
            Tuesday.to_string(),
            Wednesday.to_string(),
            Thursday.to_string(),
            Friday.to_string(),
            Saturday.to_string(),
            Sunday.to_string(),
        ));
    }
}
