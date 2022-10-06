use rand::Rng;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

#[test]
fn support() {
    // Work around rust-random/rand#1020.
    let mut rng = rand::rngs::mock::StepRng::new(0, 656_175_560);

    for _ in 0..7 {
        let _ = rng.gen::<Weekday>();
    }
    for _ in 0..12 {
        let _ = rng.gen::<Month>();
    }
    let _ = rng.gen::<Time>();
    let _ = rng.gen::<Date>();
    let _ = rng.gen::<UtcOffset>();
    let _ = rng.gen::<PrimitiveDateTime>();
    let _ = rng.gen::<OffsetDateTime>();
    let _ = rng.gen::<Duration>();
}
