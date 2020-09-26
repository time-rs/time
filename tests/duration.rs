use standback::convert::TryFrom;
#[allow(unused_imports)]
use standback::prelude::*;
use std::{cmp::Ordering, time::Duration as StdDuration};
use time::{prelude::*, Duration};

macro_rules! assert_panics {
    ($e:expr $(, $message:literal)?) => {
        #[allow(box_pointers)]
        {
            if std::panic::catch_unwind(move || $e).is_ok() {
                panic!(concat!(
                    "assertion failed: expected `",
                    stringify!($e),
                    "` to panic",
                    $(concat!(" (", $message, ")"))?
                ));
            }
        }
    };
}

#[test]
fn unit_values() {
    assert_eq!(Duration::zero(), 0.seconds());
    assert_eq!(Duration::nanosecond(), 1.nanoseconds());
    assert_eq!(Duration::microsecond(), 1.microseconds());
    assert_eq!(Duration::millisecond(), 1.milliseconds());
    assert_eq!(Duration::second(), 1.seconds());
    assert_eq!(Duration::minute(), 60.seconds());
    assert_eq!(Duration::hour(), 3_600.seconds());
    assert_eq!(Duration::day(), 86_400.seconds());
    assert_eq!(Duration::week(), 604_800.seconds());
}

#[test]
fn is_zero() {
    assert!(!(-1).nanoseconds().is_zero());
    assert!(0.seconds().is_zero());
    assert!(!1.nanoseconds().is_zero());
}

#[test]
fn is_negative() {
    assert!((-1).seconds().is_negative());
    assert!(!0.seconds().is_negative());
    assert!(!1.seconds().is_negative());
}

#[test]
fn is_positive() {
    assert!(!(-1).seconds().is_positive());
    assert!(!0.seconds().is_positive());
    assert!(1.seconds().is_positive());
}

#[allow(deprecated)]
#[test]
fn sign() {
    use time::Sign::*;
    assert_eq!(1.seconds().sign(), Positive);
    assert_eq!((-1).seconds().sign(), Negative);
    assert_eq!(1.nanoseconds().sign(), Positive);
    assert_eq!((-1).nanoseconds().sign(), Negative);
    assert_eq!(0.seconds().sign(), Zero);
}

#[test]
fn abs() {
    assert_eq!(1.seconds().abs(), 1.seconds());
    assert_eq!(0.seconds().abs(), 0.seconds());
    assert_eq!((-1).seconds().abs(), 1.seconds());
}

#[test]
fn new() {
    assert_eq!(Duration::new(1, 0), 1.seconds());
    assert_eq!(Duration::new(-1, 0), (-1).seconds());
    assert_eq!(Duration::new(1, 2_000_000_000), 3.seconds());

    assert_eq!(Duration::new(0, 0), 0.seconds());
    assert_eq!(Duration::new(0, 1_000_000_000), 1.seconds());
    assert_eq!(Duration::new(-1, 1_000_000_000), 0.seconds());
    assert_eq!(Duration::new(-2, 1_000_000_000), (-1).seconds());

    assert_eq!(Duration::new(1, -1), 999_999_999.nanoseconds());
    assert_eq!(Duration::new(-1, 1), (-999_999_999).nanoseconds());
    assert_eq!(Duration::new(1, 1), 1_000_000_001.nanoseconds());
    assert_eq!(Duration::new(-1, -1), (-1_000_000_001).nanoseconds());
    assert_eq!(Duration::new(0, 1), 1.nanoseconds());
    assert_eq!(Duration::new(0, -1), (-1).nanoseconds());

    assert_eq!(Duration::new(-1, 1_400_000_000), 400.milliseconds());
    assert_eq!(Duration::new(-2, 1_400_000_000), (-600).milliseconds());
    assert_eq!(Duration::new(-3, 1_400_000_000), (-1_600).milliseconds());
    assert_eq!(Duration::new(1, -1_400_000_000), (-400).milliseconds());
    assert_eq!(Duration::new(2, -1_400_000_000), 600.milliseconds());
    assert_eq!(Duration::new(3, -1_400_000_000), 1_600.milliseconds());
}

#[test]
fn weeks() {
    assert_eq!(Duration::weeks(1), 604_800.seconds());
    assert_eq!(Duration::weeks(2), (2 * 604_800).seconds());
    assert_eq!(Duration::weeks(-1), (-604_800).seconds());
    assert_eq!(Duration::weeks(-2), (2 * -604_800).seconds());
}

#[test]
fn whole_weeks() {
    assert_eq!(Duration::weeks(1).whole_weeks(), 1);
    assert_eq!(Duration::weeks(-1).whole_weeks(), -1);
    assert_eq!(Duration::days(6).whole_weeks(), 0);
    assert_eq!(Duration::days(-6).whole_weeks(), 0);
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn num_weeks() {
    assert_eq!(Duration::weeks(1).num_weeks(), 1);
    assert_eq!(Duration::weeks(-1).num_weeks(), -1);
    assert_eq!(Duration::days(6).num_weeks(), 0);
    assert_eq!(Duration::days(-6).num_weeks(), 0);
}

#[test]
fn days() {
    assert_eq!(Duration::days(1), 86_400.seconds());
    assert_eq!(Duration::days(2), (2 * 86_400).seconds());
    assert_eq!(Duration::days(-1), (-86_400).seconds());
    assert_eq!(Duration::days(-2), (2 * -86_400).seconds());
}

#[test]
fn whole_days() {
    assert_eq!(Duration::days(1).whole_days(), 1);
    assert_eq!(Duration::days(-1).whole_days(), -1);
    assert_eq!(Duration::hours(23).whole_days(), 0);
    assert_eq!(Duration::hours(-23).whole_days(), 0);
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn num_days() {
    assert_eq!(Duration::days(1).num_days(), 1);
    assert_eq!(Duration::days(-1).num_days(), -1);
    assert_eq!(Duration::hours(23).num_days(), 0);
    assert_eq!(Duration::hours(-23).num_days(), 0);
}

#[test]
fn hours() {
    assert_eq!(Duration::hours(1), 3_600.seconds());
    assert_eq!(Duration::hours(2), (2 * 3_600).seconds());
    assert_eq!(Duration::hours(-1), (-3_600).seconds());
    assert_eq!(Duration::hours(-2), (2 * -3_600).seconds());
}

#[test]
fn whole_hours() {
    assert_eq!(Duration::hours(1).whole_hours(), 1);
    assert_eq!(Duration::hours(-1).whole_hours(), -1);
    assert_eq!(Duration::minutes(59).whole_hours(), 0);
    assert_eq!(Duration::minutes(-59).whole_hours(), 0);
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn num_hours() {
    assert_eq!(Duration::hours(1).num_hours(), 1);
    assert_eq!(Duration::hours(-1).num_hours(), -1);
    assert_eq!(Duration::minutes(59).num_hours(), 0);
    assert_eq!(Duration::minutes(-59).num_hours(), 0);
}

#[test]
fn minutes() {
    assert_eq!(Duration::minutes(1), 60.seconds());
    assert_eq!(Duration::minutes(2), (2 * 60).seconds());
    assert_eq!(Duration::minutes(-1), (-60).seconds());
    assert_eq!(Duration::minutes(-2), (2 * -60).seconds());
}

#[test]
fn whole_minutes() {
    assert_eq!(1.minutes().whole_minutes(), 1);
    assert_eq!((-1).minutes().whole_minutes(), -1);
    assert_eq!(59.seconds().whole_minutes(), 0);
    assert_eq!((-59).seconds().whole_minutes(), 0);
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn num_minutes() {
    assert_eq!(1.minutes().num_minutes(), 1);
    assert_eq!((-1).minutes().num_minutes(), -1);
    assert_eq!(59.seconds().num_minutes(), 0);
    assert_eq!((-59).seconds().num_minutes(), 0);
}

#[test]
fn seconds() {
    assert_eq!(Duration::seconds(1), 1_000.milliseconds());
    assert_eq!(Duration::seconds(2), (2 * 1_000).milliseconds());
    assert_eq!(Duration::seconds(-1), (-1_000).milliseconds());
    assert_eq!(Duration::seconds(-2), (2 * -1_000).milliseconds());
}

#[test]
fn whole_seconds() {
    assert_eq!(1.seconds().whole_seconds(), 1);
    assert_eq!((-1).seconds().whole_seconds(), -1);
    assert_eq!(1.minutes().whole_seconds(), 60);
    assert_eq!((-1).minutes().whole_seconds(), -60);
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn num_seconds() {
    assert_eq!(1.seconds().num_seconds(), 1);
    assert_eq!((-1).seconds().num_seconds(), -1);
    assert_eq!(1.minutes().num_seconds(), 60);
    assert_eq!((-1).minutes().num_seconds(), -60);
}

#[test]
fn seconds_f64() {
    assert_eq!(Duration::seconds_f64(0.5), 0.5.seconds());
    assert_eq!(Duration::seconds_f64(-0.5), (-0.5).seconds());
}

#[test]
#[allow(clippy::float_cmp)]
fn as_seconds_f64() {
    assert_eq!(1.seconds().as_seconds_f64(), 1.0);
    assert_eq!((-1).seconds().as_seconds_f64(), -1.0);
    assert_eq!(1.minutes().as_seconds_f64(), 60.0);
    assert_eq!((-1).minutes().as_seconds_f64(), -60.0);
    assert_eq!(1.5.seconds().as_seconds_f64(), 1.5);
    assert_eq!((-1.5).seconds().as_seconds_f64(), -1.5);
}

#[test]
fn seconds_f32() {
    assert_eq!(Duration::seconds_f32(0.5), 0.5.seconds());
    assert_eq!(Duration::seconds_f32(-0.5), (-0.5).seconds());
}

#[test]
#[allow(clippy::float_cmp)]
fn as_seconds_f32() {
    assert_eq!(1.seconds().as_seconds_f32(), 1.0);
    assert_eq!((-1).seconds().as_seconds_f32(), -1.0);
    assert_eq!(1.minutes().as_seconds_f32(), 60.0);
    assert_eq!((-1).minutes().as_seconds_f32(), -60.0);
    assert_eq!(1.5.seconds().as_seconds_f32(), 1.5);
    assert_eq!((-1.5).seconds().as_seconds_f32(), -1.5);
}

#[test]
fn milliseconds() {
    assert_eq!(Duration::milliseconds(1), 1_000.microseconds());
    assert_eq!(Duration::milliseconds(-1), (-1000).microseconds());
}

#[test]
fn whole_milliseconds() {
    assert_eq!(1.seconds().whole_milliseconds(), 1_000);
    assert_eq!((-1).seconds().whole_milliseconds(), -1_000);
    assert_eq!(1.milliseconds().whole_milliseconds(), 1);
    assert_eq!((-1).milliseconds().whole_milliseconds(), -1);
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn num_milliseconds() {
    assert_eq!(1.seconds().num_milliseconds(), 1_000);
    assert_eq!((-1).seconds().num_milliseconds(), -1_000);
    assert_eq!(1.milliseconds().num_milliseconds(), 1);
    assert_eq!((-1).milliseconds().num_milliseconds(), -1);
    assert_eq!(Duration::max_value().num_milliseconds(), i64::MAX);
    assert_eq!(Duration::min_value().num_milliseconds(), i64::MIN);
}

#[test]
fn subsec_milliseconds() {
    assert_eq!(1.4.seconds().subsec_milliseconds(), 400);
    assert_eq!((-1.4).seconds().subsec_milliseconds(), -400);
}

#[test]
fn microseconds() {
    assert_eq!(Duration::microseconds(1), 1_000.nanoseconds());
    assert_eq!(Duration::microseconds(-1), (-1_000).nanoseconds());
}

#[test]
fn whole_microseconds() {
    assert_eq!(1.milliseconds().whole_microseconds(), 1_000);
    assert_eq!((-1).milliseconds().whole_microseconds(), -1_000);
    assert_eq!(1.microseconds().whole_microseconds(), 1);
    assert_eq!((-1).microseconds().whole_microseconds(), -1);
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn num_microseconds() {
    assert_eq!(1.milliseconds().num_microseconds(), Some(1_000));
    assert_eq!((-1).milliseconds().num_microseconds(), Some(-1_000));
    assert_eq!(1.microseconds().num_microseconds(), Some(1));
    assert_eq!((-1).microseconds().num_microseconds(), Some(-1));
    assert_eq!(Duration::max_value().num_microseconds(), None);
    assert_eq!(Duration::min_value().num_microseconds(), None);
}

#[test]
fn subsec_microseconds() {
    assert_eq!(1.0004.seconds().subsec_microseconds(), 400);
    assert_eq!((-1.0004).seconds().subsec_microseconds(), -400);
}

#[test]
fn nanoseconds() {
    assert_eq!(Duration::nanoseconds(1), 1.microseconds() / 1_000);
    assert_eq!(Duration::nanoseconds(-1), (-1).microseconds() / 1_000);
}

#[test]
fn whole_nanoseconds() {
    assert_eq!(1.microseconds().whole_nanoseconds(), 1_000);
    assert_eq!((-1).microseconds().whole_nanoseconds(), -1_000);
    assert_eq!(1.nanoseconds().whole_nanoseconds(), 1);
    assert_eq!((-1).nanoseconds().whole_nanoseconds(), -1);
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn num_nanoseconds() {
    assert_eq!(1.microseconds().num_nanoseconds(), Some(1_000));
    assert_eq!((-1).microseconds().num_nanoseconds(), Some(-1_000));
    assert_eq!(1.nanoseconds().num_nanoseconds(), Some(1));
    assert_eq!((-1).nanoseconds().num_nanoseconds(), Some(-1));
    assert_eq!(Duration::max_value().num_nanoseconds(), None);
    assert_eq!(Duration::min_value().num_nanoseconds(), None);
}

#[test]
fn subsec_nanoseconds() {
    assert_eq!(1.000_000_4.seconds().subsec_nanoseconds(), 400);
    assert_eq!((-1.000_000_4).seconds().subsec_nanoseconds(), -400);
}

#[test]
#[allow(deprecated)]
fn checked_add() {
    assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
    assert_eq!(Duration::max_value().checked_add(1.nanoseconds()), None);
    assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
}

#[test]
#[allow(deprecated)]
fn checked_sub() {
    assert_eq!(5.seconds().checked_sub(5.seconds()), Some(0.seconds()));
    assert_eq!(Duration::min_value().checked_sub(1.nanoseconds()), None);
    assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
}

#[test]
#[allow(deprecated)]
fn checked_mul() {
    assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
    assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
    assert_eq!(5.seconds().checked_mul(0), Some(Duration::zero()));
    assert_eq!(Duration::max_value().checked_mul(2), None);
    assert_eq!(Duration::min_value().checked_mul(2), None);
}

#[test]
fn checked_div() {
    assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
    assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
    assert_eq!(1.seconds().checked_div(0), None);
}

#[test]
#[cfg(feature = "std")]
fn time_fn() {
    let (time, value) = Duration::time_fn(|| {
        std::thread::sleep(1.std_milliseconds());
        0
    });

    assert!(time >= 1.milliseconds());
    assert_eq!(value, 0);
}

#[test]
#[cfg(all(feature = "std", feature = "deprecated"))]
#[allow(deprecated)]
fn span() {
    let time = Duration::span(|| {
        std::thread::sleep(1.std_milliseconds());
    });
    assert!(time >= 1.milliseconds());
}

#[test]
fn try_from_std_duration() {
    assert_eq!(Duration::try_from(0.std_seconds()), Ok(0.seconds()));
    assert_eq!(Duration::try_from(1.std_seconds()), Ok(1.seconds()));
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn from_std() {
    assert_eq!(Duration::from_std(0.std_seconds()), Ok(0.seconds()));
    assert_eq!(Duration::from_std(1.std_seconds()), Ok(1.seconds()));
}

#[test]
fn try_to_std_duration() {
    assert_eq!(StdDuration::try_from(0.seconds()), Ok(0.std_seconds()));
    assert_eq!(StdDuration::try_from(1.seconds()), Ok(1.std_seconds()));
    assert!(StdDuration::try_from((-1).seconds()).is_err());
}

#[test]
#[cfg(feature = "deprecated")]
#[allow(deprecated)]
fn to_std() {
    assert_eq!(0.seconds().to_std(), Ok(0.std_seconds()));
    assert_eq!(1.seconds().to_std(), Ok(1.std_seconds()));
    assert!((-1).seconds().to_std().is_err());
}

#[test]
fn add() {
    assert_eq!(1.seconds() + 1.seconds(), 2.seconds());
    assert_eq!(500.milliseconds() + 500.milliseconds(), 1.seconds());
    assert_eq!(1.seconds() + (-1).seconds(), 0.seconds());
}

#[test]
fn add_std() {
    assert_eq!(1.seconds() + 1.std_seconds(), 2.seconds());
    assert_eq!(500.milliseconds() + 500.std_milliseconds(), 1.seconds());
    assert_eq!((-1).seconds() + 1.std_seconds(), 0.seconds());
}

#[test]
fn std_add() {
    assert_eq!(1.std_seconds() + 1.seconds(), 2.seconds());
    assert_eq!(500.std_milliseconds() + 500.milliseconds(), 1.seconds());
    assert_eq!(1.std_seconds() + (-1).seconds(), 0.seconds());
}

#[test]
fn add_assign() {
    let mut duration = 1.seconds();
    duration += 1.seconds();
    assert_eq!(duration, 2.seconds());

    let mut duration = 500.milliseconds();
    duration += 500.milliseconds();
    assert_eq!(duration, 1.seconds());

    let mut duration = 1.seconds();
    duration += (-1).seconds();
    assert_eq!(duration, 0.seconds());
}

#[test]
fn add_assign_std() {
    let mut duration = 1.seconds();
    duration += 1.std_seconds();
    assert_eq!(duration, 2.seconds());

    let mut duration = 500.milliseconds();
    duration += 500.std_milliseconds();
    assert_eq!(duration, 1.seconds());

    let mut duration = (-1).seconds();
    duration += 1.std_seconds();
    assert_eq!(duration, 0.seconds());
}

#[test]
fn neg() {
    assert_eq!(-(1.seconds()), (-1).seconds());
    assert_eq!(-(-1).seconds(), 1.seconds());
    assert_eq!(-(0.seconds()), 0.seconds());
}

#[test]
fn sub() {
    assert_eq!(1.seconds() - 1.seconds(), 0.seconds());
    assert_eq!(1_500.milliseconds() - 500.milliseconds(), 1.seconds());
    assert_eq!(1.seconds() - (-1).seconds(), 2.seconds());
}

#[test]
fn sub_std() {
    assert_eq!(1.seconds() - 1.std_seconds(), 0.seconds());
    assert_eq!(1_500.milliseconds() - 500.std_milliseconds(), 1.seconds());
    assert_eq!((-1).seconds() - 1.std_seconds(), (-2).seconds());
}

#[test]
fn std_sub() {
    assert_eq!(1.std_seconds() - 1.seconds(), 0.seconds());
    assert_eq!(1_500.std_milliseconds() - 500.milliseconds(), 1.seconds());
    assert_eq!(1.std_seconds() - (-1).seconds(), 2.seconds());
}

#[test]
fn sub_assign() {
    let mut duration = 1.seconds();
    duration -= 1.seconds();
    assert_eq!(duration, 0.seconds());

    let mut duration = 1_500.milliseconds();
    duration -= 500.milliseconds();
    assert_eq!(duration, 1.seconds());

    let mut duration = 1.seconds();
    duration -= (-1).seconds();
    assert_eq!(duration, 2.seconds());
}

#[test]
fn sub_assign_std() {
    let mut duration = 1.seconds();
    duration -= 1.std_seconds();
    assert_eq!(duration, 0.seconds());

    let mut duration = 1_500.milliseconds();
    duration -= 500.std_milliseconds();
    assert_eq!(duration, 1.seconds());

    let mut duration = (-1).seconds();
    duration -= 1.std_seconds();
    assert_eq!(duration, (-2).seconds());
}

#[test]
fn std_sub_assign() {
    let mut duration = 1.std_seconds();
    duration -= 1.seconds();
    assert_eq!(duration, 0.seconds());

    let mut duration = 1_500.std_milliseconds();
    duration -= 500.milliseconds();
    assert_eq!(duration, 1.seconds());

    let mut duration = 1.std_seconds();
    assert_panics!(duration -= 2.seconds());
}

#[test]
fn mul_int() {
    assert_eq!(1.seconds() * 2, 2.seconds());
    assert_eq!(1.seconds() * -2, (-2).seconds());
}

#[test]
fn mul_int_assign() {
    let mut duration = 1.seconds();
    duration *= 2;
    assert_eq!(duration, 2.seconds());

    let mut duration = 1.seconds();
    duration *= -2;
    assert_eq!(duration, (-2).seconds());
}

#[test]
fn int_mul() {
    assert_eq!(2 * 1.seconds(), 2.seconds());
    assert_eq!(-2 * 1.seconds(), (-2).seconds());
}

#[test]
fn div_int() {
    assert_eq!(1.seconds() / 2, 500.milliseconds());
    assert_eq!(1.seconds() / -2, (-500).milliseconds());
}

#[test]
fn div_int_assign() {
    let mut duration = 1.seconds();
    duration /= 2;
    assert_eq!(duration, 500.milliseconds());

    let mut duration = 1.seconds();
    duration /= -2;
    assert_eq!(duration, (-500).milliseconds());
}

#[test]
#[allow(clippy::float_cmp)]
fn div() {
    assert_eq!(1.seconds() / 0.5.seconds(), 2.);
    assert_eq!(1.std_seconds() / 0.5.seconds(), 2.);
    assert_eq!(1.seconds() / 0.5.std_seconds(), 2.);
}

#[test]
fn mul_float() {
    assert_eq!(1.seconds() * 1.5_f32, 1_500.milliseconds());
    assert_eq!(1.seconds() * 2.5_f32, 2_500.milliseconds());
    assert_eq!(1.seconds() * -1.5_f32, (-1_500).milliseconds());
    assert_eq!(1.seconds() * 0_f32, 0.seconds());

    assert_eq!(1.seconds() * 1.5_f64, 1_500.milliseconds());
    assert_eq!(1.seconds() * 2.5_f64, 2_500.milliseconds());
    assert_eq!(1.seconds() * -1.5_f64, (-1_500).milliseconds());
    assert_eq!(1.seconds() * 0_f64, 0.seconds());
}

#[test]
fn float_mul() {
    assert_eq!(1.5_f32 * 1.seconds(), 1_500.milliseconds());
    assert_eq!(2.5_f32 * 1.seconds(), 2_500.milliseconds());
    assert_eq!(-1.5_f32 * 1.seconds(), (-1_500).milliseconds());
    assert_eq!(0_f32 * 1.seconds(), 0.seconds());

    assert_eq!(1.5_f64 * 1.seconds(), 1_500.milliseconds());
    assert_eq!(2.5_f64 * 1.seconds(), 2_500.milliseconds());
    assert_eq!(-1.5_f64 * 1.seconds(), (-1_500).milliseconds());
    assert_eq!(0_f64 * 1.seconds(), 0.seconds());
}

#[test]
fn mul_float_assign() {
    let mut duration = 1.seconds();
    duration *= 1.5_f32;
    assert_eq!(duration, 1_500.milliseconds());

    let mut duration = 1.seconds();
    duration *= 2.5_f32;
    assert_eq!(duration, 2_500.milliseconds());

    let mut duration = 1.seconds();
    duration *= -1.5_f32;
    assert_eq!(duration, (-1_500).milliseconds());

    let mut duration = 1.seconds();
    duration *= 0_f32;
    assert_eq!(duration, 0.seconds());

    let mut duration = 1.seconds();
    duration *= 1.5_f64;
    assert_eq!(duration, 1_500.milliseconds());

    let mut duration = 1.seconds();
    duration *= 2.5_f64;
    assert_eq!(duration, 2_500.milliseconds());

    let mut duration = 1.seconds();
    duration *= -1.5_f64;
    assert_eq!(duration, (-1_500).milliseconds());

    let mut duration = 1.seconds();
    duration *= 0_f64;
    assert_eq!(duration, 0.seconds());
}

#[test]
fn div_float() {
    assert_eq!(1.seconds() / 1_f32, 1.seconds());
    assert_eq!(1.seconds() / 2_f32, 500.milliseconds());
    assert_eq!(1.seconds() / -1_f32, (-1).seconds());

    assert_eq!(1.seconds() / 1_f64, 1.seconds());
    assert_eq!(1.seconds() / 2_f64, 500.milliseconds());
    assert_eq!(1.seconds() / -1_f64, (-1).seconds());
}

#[test]
fn div_float_assign() {
    let mut duration = 1.seconds();
    duration /= 1_f32;
    assert_eq!(duration, 1.seconds());

    let mut duration = 1.seconds();
    duration /= 2_f32;
    assert_eq!(duration, 500.milliseconds());

    let mut duration = 1.seconds();
    duration /= -1_f32;
    assert_eq!(duration, (-1).seconds());

    let mut duration = 1.seconds();
    duration /= 1_f64;
    assert_eq!(duration, 1.seconds());

    let mut duration = 1.seconds();
    duration /= 2_f64;
    assert_eq!(duration, 500.milliseconds());

    let mut duration = 1.seconds();
    duration /= -1_f64;
    assert_eq!(duration, (-1).seconds());
}

#[test]
fn partial_eq() {
    assert_eq!(1.seconds(), 1.seconds());
    assert_eq!(0.seconds(), 0.seconds());
    assert_eq!((-1).seconds(), (-1).seconds());
    assert_ne!(1.minutes(), (-1).minutes());
    assert_ne!(40.seconds(), 1.minutes());
}

#[test]
fn partial_eq_std() {
    assert_eq!(1.seconds(), 1.std_seconds());
    assert_eq!(0.seconds(), 0.std_seconds());
    assert_ne!((-1).seconds(), 1.std_seconds());
    assert_ne!((-1).minutes(), 1.std_minutes());
    assert_ne!(40.seconds(), 1.std_minutes());
}

#[test]
fn std_partial_eq() {
    assert_eq!(1.std_seconds(), 1.seconds());
    assert_eq!(0.std_seconds(), 0.seconds());
    assert_ne!(1.std_seconds(), (-1).seconds());
    assert_ne!(1.std_minutes(), (-1).minutes());
    assert_ne!(40.std_seconds(), 1.minutes());
}

#[test]
fn partial_ord() {
    use Ordering::*;
    assert_eq!(0.seconds().partial_cmp(&0.seconds()), Some(Equal));
    assert_eq!(1.seconds().partial_cmp(&0.seconds()), Some(Greater));
    assert_eq!(1.seconds().partial_cmp(&(-1).seconds()), Some(Greater));
    assert_eq!((-1).seconds().partial_cmp(&1.seconds()), Some(Less));
    assert_eq!(0.seconds().partial_cmp(&(-1).seconds()), Some(Greater));
    assert_eq!(0.seconds().partial_cmp(&1.seconds()), Some(Less));
    assert_eq!((-1).seconds().partial_cmp(&0.seconds()), Some(Less));
    assert_eq!(1.minutes().partial_cmp(&1.seconds()), Some(Greater));
    assert_eq!((-1).minutes().partial_cmp(&(-1).seconds()), Some(Less));
}

#[test]
fn partial_ord_std() {
    use Ordering::*;
    assert_eq!(0.seconds().partial_cmp(&0.std_seconds()), Some(Equal));
    assert_eq!(1.seconds().partial_cmp(&0.std_seconds()), Some(Greater));
    assert_eq!((-1).seconds().partial_cmp(&1.std_seconds()), Some(Less));
    assert_eq!(0.seconds().partial_cmp(&1.std_seconds()), Some(Less));
    assert_eq!((-1).seconds().partial_cmp(&0.std_seconds()), Some(Less));
    assert_eq!(1.minutes().partial_cmp(&1.std_seconds()), Some(Greater));
    assert_eq!(0.seconds().partial_cmp(&u64::MAX.std_seconds()), Some(Less));
}

#[test]
fn std_partial_ord() {
    use Ordering::*;
    assert_eq!(0.std_seconds().partial_cmp(&0.seconds()), Some(Equal));
    assert_eq!(1.std_seconds().partial_cmp(&0.seconds()), Some(Greater));
    assert_eq!(1.std_seconds().partial_cmp(&(-1).seconds()), Some(Greater));
    assert_eq!(0.std_seconds().partial_cmp(&(-1).seconds()), Some(Greater));
    assert_eq!(0.std_seconds().partial_cmp(&1.seconds()), Some(Less));
    assert_eq!(1.std_minutes().partial_cmp(&1.seconds()), Some(Greater));
}

#[test]
fn ord() {
    assert_eq!(0.seconds(), 0.seconds());
    assert!(1.seconds() > 0.seconds());
    assert!(1.seconds() > (-1).seconds());
    assert!((-1).seconds() < 1.seconds());
    assert!(0.seconds() > (-1).seconds());
    assert!(0.seconds() < 1.seconds());
    assert!((-1).seconds() < 0.seconds());
    assert!(1.minutes() > 1.seconds());
    assert!((-1).minutes() < (-1).seconds());
}

#[test]
fn arithmetic_regression() {
    let added = 1.6.seconds() + 1.6.seconds();
    assert_eq!(added.whole_seconds(), 3);
    assert_eq!(added.subsec_milliseconds(), 200);

    let subtracted = 1.6.seconds() - (-1.6).seconds();
    assert_eq!(subtracted.whole_seconds(), 3);
    assert_eq!(subtracted.subsec_milliseconds(), 200);
}
