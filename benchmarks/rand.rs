use criterion::{BatchSize, Bencher};
use rand::Rng;
use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

setup_benchmark! {
    "Random",

    fn rng(ben: &mut Bencher) {
        ben.iter_batched_ref(
            || rand::rngs::mock::StepRng::new(0, 1),
            |rng| {
                let _ = rng.gen::<Time>();
                let _ = rng.gen::<Date>();
                let _ = rng.gen::<UtcOffset>();
                let _ = rng.gen::<PrimitiveDateTime>();
                let _ = rng.gen::<OffsetDateTime>();
                let _ = rng.gen::<Duration>();
                let _ = rng.gen::<Weekday>();
            },
            BatchSize::SmallInput
        );
    }
}
