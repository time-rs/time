use std::time::Instant as StdInstant;

use criterion::Bencher;
use criterion_cycles_per_byte::CyclesPerByte;
use time::ext::NumericalDuration;
use time::{Duration, Instant};

setup_benchmark! {
    "Instant",

    // region: checked arithmetic
    fn checked_add(ben: &mut Bencher<'_, CyclesPerByte>) {
        let instant = Instant::now();
        let dt = 5.seconds();
        ben.iter(|| instant.checked_add(dt));
    }

    fn checked_sub(ben: &mut Bencher<'_, CyclesPerByte>) {
        let instant = Instant::now();
        let dt = 5.seconds();
        ben.iter(|| instant.checked_sub(dt));
    }
    // endregion checked arithmetic

    // region: trait impls
    fn sub(ben: &mut Bencher<'_, CyclesPerByte>) {
        let start: Instant = Instant::now();
        let end: Instant = start + 1.milliseconds();
        ben.iter(|| end - start);
    }

    fn add_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let start = Instant::now();
        let dt: Duration = 1.seconds();
        ben.iter(|| start + dt);
    }

    fn std_add_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let start = StdInstant::now();
        let dt: Duration = 1.milliseconds();
        ben.iter(|| start + dt);
    }

    fn add_assign_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt: Duration = 1.milliseconds();
        iter_batched_ref!(
            ben,
            Instant::now,
            [|start| *start += dt]
        );
    }

    fn std_add_assign_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt: Duration = 1.milliseconds();
        iter_batched_ref!(
            ben,
            StdInstant::now,
            [|start| *start += dt]
        );
    }

    fn sub_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let instant = Instant::now();
        let dt: Duration = 100.milliseconds();
        ben.iter(|| instant - dt);
    }

    fn std_sub_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let instant = StdInstant::now();
        let dt: Duration = 100.milliseconds();
        ben.iter(|| instant - dt);
    }

    fn sub_assign_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt: Duration = 100.milliseconds();
        iter_batched_ref!(
            ben,
            Instant::now,
            [|instant| *instant -= dt]
        );
    }

    fn std_sub_assign_duration(ben: &mut Bencher<'_, CyclesPerByte>) {
        let dt: Duration = 100.milliseconds();
        iter_batched_ref!(
            ben,
            StdInstant::now,
            [|instant| *instant -= dt]
        );
    }
    // endregion trait impls
}
