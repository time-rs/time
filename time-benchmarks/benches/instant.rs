use bench_util::setup_benchmark;
use criterion::BatchSize;
use std::time::{Duration as StdDuration, Instant as StdInstant};
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    Duration, Instant,
};

setup_benchmark! {
    "Instant",

    fn checked_add(ben: &mut Bencher) {
        let instant = Instant::now();
        let dt = 5.seconds();
        ben.iter(|| instant.checked_add(dt));
    }

    fn checked_sub(ben: &mut Bencher) {
        let instant = Instant::now();
        let dt = 5.seconds();
        ben.iter(|| instant.checked_sub(dt));
    }

    fn from_std(ben: &mut Bencher) {
        let std_instant = StdInstant::now();
        ben.iter(|| Instant::from(std_instant));
    }

    fn to_std(ben: &mut Bencher) {
        let instant = Instant::now();
        ben.iter(|| StdInstant::from(instant));
    }

    fn sub(ben: &mut Bencher) {
        let start: Instant = Instant::now();
        let end: Instant = start + 1.milliseconds();
        ben.iter(|| end - start);
    }

    fn sub_std(ben: &mut Bencher) {
        let start = StdInstant::now();
        let end: Instant = Instant::from(start + 1.milliseconds());
        ben.iter(|| end - start);
    }

    fn std_sub(ben: &mut Bencher) {
        let start = Instant::now();
        let end: StdInstant = StdInstant::from(start + 1.std_milliseconds());
        ben.iter(|| end - start);
    }

    fn add_duration(ben: &mut Bencher) {
        let start = Instant::now();
        let dt: Duration = 1.seconds();
        ben.iter(|| start + dt);
    }

    fn std_add_duration(ben: &mut Bencher) {
        let start = StdInstant::now();
        let dt: Duration = 1.milliseconds();
        ben.iter(|| start + dt);
    }

    fn add_std_duration(ben: &mut Bencher) {
        let start = Instant::now();
        let dt: StdDuration = 1.std_milliseconds();
        ben.iter(|| start + dt);
    }

    fn add_assign_duration(ben: &mut Bencher) {
        let dt: Duration = 1.milliseconds();
        ben.iter_batched_ref(
            Instant::now,
            |start| {
                *start += dt;
            },
            BatchSize::SmallInput
        );
    }

    fn std_add_assign_duration(ben: &mut Bencher) {
        let dt: Duration = 1.milliseconds();
        ben.iter_batched_ref(
            StdInstant::now,
            |start| {
                *start += dt;
            },
            BatchSize::SmallInput
        );
    }

    fn add_assign_std_duration(ben: &mut Bencher) {
        let dt: StdDuration = 1.std_milliseconds();
        ben.iter_batched_ref(
            Instant::now,
            |start| {
                *start += dt;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_duration(ben: &mut Bencher) {
        let instant = Instant::now();
        let dt: Duration = 100.milliseconds();
        ben.iter(|| instant - dt);
    }

    fn std_sub_duration(ben: &mut Bencher) {
        let instant = StdInstant::now();
        let dt: Duration = 100.milliseconds();
        ben.iter(|| instant - dt);
    }

    fn sub_std_duration(ben: &mut Bencher) {
        let instant = Instant::now();
        let dt: StdDuration = 100.std_milliseconds();
        ben.iter(|| instant - dt);
    }

    fn sub_assign_duration(ben: &mut Bencher) {
        let dt: Duration = 100.milliseconds();
        ben.iter_batched_ref(
            Instant::now,
            |instant| {
                *instant -= dt;
            },
            BatchSize::SmallInput
        );
    }

    fn std_sub_assign_duration(ben: &mut Bencher) {
        let dt: Duration = 100.milliseconds();
        ben.iter_batched_ref(
            StdInstant::now,
            |instant| {
                *instant -= dt;
            },
            BatchSize::SmallInput
        );
    }

    fn sub_assign_std_duration(ben: &mut Bencher) {
        let dt: StdDuration = 100.std_milliseconds();
        ben.iter_batched_ref(
            Instant::now,
            |instant| {
                *instant -= dt;
            },
            BatchSize::SmallInput
        );
    }

    fn eq_std(ben: &mut Bencher) {
        let instant = Instant::now();
        let std_instant = StdInstant::from(instant);
        ben.iter(|| instant == std_instant);
    }

    fn std_eq(ben: &mut Bencher) {
        let instant = Instant::now();
        let std_instant = StdInstant::from(instant);
        ben.iter(|| std_instant == instant);
    }

    fn ord_std(ben: &mut Bencher) {
        let instant = Instant::now();
        let std_instant = StdInstant::from(instant) + 1.seconds();
        ben.iter(|| (
            instant < std_instant,
            instant > std_instant,
        ));
    }

    fn std_ord(ben: &mut Bencher) {
        let instant = Instant::now();
        let std_instant = StdInstant::from(instant) + 1.seconds();
        ben.iter(|| (
            std_instant > instant,
            std_instant < instant,
        ));
    }
}
