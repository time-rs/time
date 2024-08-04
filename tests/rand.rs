use rand::Rng;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

#[test]
fn support() {
    // Work around rust-random/rand#1020.
    let mut rng = rand::rngs::mock::StepRng::new(0, 656_175_560);

    for _ in 0..7 {
        let _ = rng.r#gen::<Weekday>();
    }
    for _ in 0..12 {
        let _ = rng.r#gen::<Month>();
    }
    let _ = rng.r#gen::<Time>();
    let _ = rng.r#gen::<Date>();
    let _ = rng.r#gen::<UtcOffset>();
    let _ = rng.r#gen::<PrimitiveDateTime>();
    let _ = rng.r#gen::<OffsetDateTime>();
    let _ = rng.r#gen::<Duration>();
}
