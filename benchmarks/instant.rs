use std::time::Instant as StdInstant;

use criterion::{BatchSize, Bencher};
use time::ext::NumericalDuration;
use time::{Duration, Instant};

setup_benchmark! {
    "Instant",

    // region: checked arithmetic
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
    // endregion checked arithmetic

    // region: trait impls
    fn sub(ben: &mut Bencher) {
        let start: Instant = Instant::now();
        let end: Instant = start + 1.milliseconds();
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
    // endregion trait impls
}
