#![cfg(feature = "rand")]

use rand::Rng;
use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

#[test]
fn support() {
    let mut rng = rand::rngs::mock::StepRng::new(0, 1);

    let _ = rng.gen::<Time>();
    let _ = rng.gen::<Date>();
    let _ = rng.gen::<UtcOffset>();
    let _ = rng.gen::<PrimitiveDateTime>();
    let _ = rng.gen::<OffsetDateTime>();
    let _ = rng.gen::<Duration>();
    let _ = rng.gen::<Weekday>();
}
