use core::u64;
use std::cmp::Ordering;
use std::time::Duration as StdDuration;

use time::ext::{NumericalDuration, NumericalStdDuration};
use time::{error, Duration};

#[test]
fn unit_values() {
    assert_eq!(Duration::ZERO, 0.seconds());
    assert_eq!(Duration::NANOSECOND, 1.nanoseconds());
    assert_eq!(Duration::MICROSECOND, 1.microseconds());
    assert_eq!(Duration::MILLISECOND, 1.milliseconds());
    assert_eq!(Duration::SECOND, 1.seconds());
    assert_eq!(Duration::MINUTE, 60.seconds());
    assert_eq!(Duration::HOUR, 3_600.seconds());
    assert_eq!(Duration::DAY, 86_400.seconds());
    assert_eq!(Duration::WEEK, 604_800.seconds());
}

#[test]
fn default() {
    assert_eq!(Duration::default(), Duration::ZERO);
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

#[test]
fn abs() {
    assert_eq!(1.seconds().abs(), 1.seconds());
    assert_eq!(0.seconds().abs(), 0.seconds());
    assert_eq!((-1).seconds().abs(), 1.seconds());
    assert_eq!(Duration::new(i64::MIN, 0).abs(), Duration::MAX);
}

#[test]
fn unsigned_abs() {
    assert_eq!(1.seconds().unsigned_abs(), 1.std_seconds());
    assert_eq!(0.seconds().unsigned_abs(), 0.std_seconds());
    assert_eq!((-1).seconds().unsigned_abs(), 1.std_seconds());
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

    assert_panic!(Duration::new(i64::MAX, 1_000_000_000));
    assert_panic!(Duration::new(i64::MIN, -1_000_000_000));
}

#[test]
fn weeks() {
    assert_eq!(Duration::weeks(1), 604_800.seconds());
    assert_eq!(Duration::weeks(2), (2 * 604_800).seconds());
    assert_eq!(Duration::weeks(-1), (-604_800).seconds());
    assert_eq!(Duration::weeks(-2), (2 * -604_800).seconds());

    assert_panic!(Duration::weeks(i64::MAX));
    assert_panic!(Duration::weeks(i64::MIN));
}

#[test]
fn whole_weeks() {
    assert_eq!(Duration::weeks(1).whole_weeks(), 1);
    assert_eq!(Duration::weeks(-1).whole_weeks(), -1);
    assert_eq!(Duration::days(6).whole_weeks(), 0);
    assert_eq!(Duration::days(-6).whole_weeks(), 0);
}

#[test]
fn days() {
    assert_eq!(Duration::days(1), 86_400.seconds());
    assert_eq!(Duration::days(2), (2 * 86_400).seconds());
    assert_eq!(Duration::days(-1), (-86_400).seconds());
    assert_eq!(Duration::days(-2), (2 * -86_400).seconds());

    assert_panic!(Duration::days(i64::MAX));
    assert_panic!(Duration::days(i64::MIN));
}

#[test]
fn whole_days() {
    assert_eq!(Duration::days(1).whole_days(), 1);
    assert_eq!(Duration::days(-1).whole_days(), -1);
    assert_eq!(Duration::hours(23).whole_days(), 0);
    assert_eq!(Duration::hours(-23).whole_days(), 0);
}

#[test]
fn hours() {
    assert_eq!(Duration::hours(1), 3_600.seconds());
    assert_eq!(Duration::hours(2), (2 * 3_600).seconds());
    assert_eq!(Duration::hours(-1), (-3_600).seconds());
    assert_eq!(Duration::hours(-2), (2 * -3_600).seconds());

    assert_panic!(Duration::hours(i64::MAX));
    assert_panic!(Duration::hours(i64::MIN));
}

#[test]
fn whole_hours() {
    assert_eq!(Duration::hours(1).whole_hours(), 1);
    assert_eq!(Duration::hours(-1).whole_hours(), -1);
    assert_eq!(Duration::minutes(59).whole_hours(), 0);
    assert_eq!(Duration::minutes(-59).whole_hours(), 0);
}

#[test]
fn minutes() {
    assert_eq!(Duration::minutes(1), 60.seconds());
    assert_eq!(Duration::minutes(2), (2 * 60).seconds());
    assert_eq!(Duration::minutes(-1), (-60).seconds());
    assert_eq!(Duration::minutes(-2), (2 * -60).seconds());

    assert_panic!(Duration::minutes(i64::MAX));
    assert_panic!(Duration::minutes(i64::MIN));
}

#[test]
fn whole_minutes() {
    assert_eq!(1.minutes().whole_minutes(), 1);
    assert_eq!((-1).minutes().whole_minutes(), -1);
    assert_eq!(59.seconds().whole_minutes(), 0);
    assert_eq!((-59).seconds().whole_minutes(), 0);
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
fn seconds_f64() {
    assert_eq!(Duration::seconds_f64(0.5), 0.5.seconds());
    assert_eq!(Duration::seconds_f64(-0.5), (-0.5).seconds());
    assert_eq!(Duration::seconds_f64(123.250), 123.250.seconds());
    assert_eq!(Duration::seconds_f64(0.000_000_000_012), Duration::ZERO);

    assert_panic!(Duration::seconds_f64(f64::MAX));
    assert_panic!(Duration::seconds_f64(f64::MIN));
    assert_panic!(Duration::seconds_f64(f64::NAN));
}

#[test]
fn saturating_seconds_f64() {
    assert_eq!(Duration::saturating_seconds_f64(0.5), 0.5.seconds());
    assert_eq!(Duration::saturating_seconds_f64(-0.5), (-0.5).seconds());
    assert_eq!(Duration::saturating_seconds_f64(123.250), 123.250.seconds());
    assert_eq!(
        Duration::saturating_seconds_f64(0.000_000_000_012),
        Duration::ZERO
    );

    assert_eq!(Duration::saturating_seconds_f64(f64::MAX), Duration::MAX);
    assert_eq!(Duration::saturating_seconds_f64(f64::MIN), Duration::MIN);
    assert_eq!(Duration::saturating_seconds_f64(f64::NAN), Duration::ZERO);
}

#[test]
fn checked_seconds_f64() {
    assert_eq!(Duration::checked_seconds_f64(0.5), Some(0.5.seconds()));
    assert_eq!(Duration::checked_seconds_f64(-0.5), Some((-0.5).seconds()));
    assert_eq!(
        Duration::checked_seconds_f64(123.250),
        Some(123.250.seconds())
    );
    assert_eq!(
        Duration::checked_seconds_f64(0.000_000_000_012),
        Some(Duration::ZERO)
    );

    assert_eq!(Duration::checked_seconds_f64(f64::MAX), None);
    assert_eq!(Duration::checked_seconds_f64(f64::MIN), None);
    assert_eq!(Duration::checked_seconds_f64(f64::NAN), None);
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
    assert_eq!(Duration::seconds_f32(123.250), 123.250.seconds());
    assert_eq!(Duration::seconds_f32(0.000_000_000_012), Duration::ZERO);

    assert_panic!(Duration::seconds_f32(f32::MAX));
    assert_panic!(Duration::seconds_f32(f32::MIN));
    assert_panic!(Duration::seconds_f32(f32::NAN));
}

#[test]
fn saturating_seconds_f32() {
    assert_eq!(Duration::saturating_seconds_f32(0.5), 0.5.seconds());
    assert_eq!(Duration::saturating_seconds_f32(-0.5), (-0.5).seconds());
    assert_eq!(Duration::saturating_seconds_f32(123.250), 123.250.seconds());
    assert_eq!(
        Duration::saturating_seconds_f32(0.000_000_000_012),
        Duration::ZERO
    );

    assert_eq!(Duration::saturating_seconds_f32(f32::MAX), Duration::MAX);
    assert_eq!(Duration::saturating_seconds_f32(f32::MIN), Duration::MIN);
    assert_eq!(Duration::saturating_seconds_f32(f32::NAN), Duration::ZERO);
}

#[test]
fn checked_seconds_f32() {
    assert_eq!(Duration::checked_seconds_f32(0.5), Some(0.5.seconds()));
    assert_eq!(Duration::checked_seconds_f32(-0.5), Some((-0.5).seconds()));
    assert_eq!(
        Duration::checked_seconds_f32(123.250),
        Some(123.250.seconds())
    );
    assert_eq!(
        Duration::checked_seconds_f32(0.000_000_000_012),
        Some(Duration::ZERO)
    );

    assert_eq!(Duration::checked_seconds_f32(f32::MAX), None);
    assert_eq!(Duration::checked_seconds_f32(f32::MIN), None);
    assert_eq!(Duration::checked_seconds_f32(f32::NAN), None);
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
fn subsec_nanoseconds() {
    assert_eq!(1.000_000_4.seconds().subsec_nanoseconds(), 400);
    assert_eq!((-1.000_000_4).seconds().subsec_nanoseconds(), -400);
}

#[test]
fn checked_add() {
    assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
    assert_eq!(Duration::MAX.checked_add(1.nanoseconds()), None);
    assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
    assert_eq!(
        1.seconds().checked_add((-1).milliseconds()),
        Some(999.milliseconds())
    );
}

#[test]
fn checked_sub() {
    assert_eq!(5.seconds().checked_sub(5.seconds()), Some(0.seconds()));
    assert_eq!(Duration::MIN.checked_sub(1.nanoseconds()), None);
    assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
    assert_eq!(Duration::MIN.checked_sub(Duration::MIN), Some(0.seconds()));
}

#[test]
fn checked_mul() {
    assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
    assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
    assert_eq!(5.seconds().checked_mul(0), Some(Duration::ZERO));
    assert_eq!(Duration::MAX.checked_mul(2), None);
    assert_eq!(Duration::MIN.checked_mul(2), None);
}

#[test]
fn checked_div() {
    assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
    assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
    assert_eq!(1.seconds().checked_div(0), None);
    assert_eq!(Duration::MIN.checked_div(-1), None);
}

#[test]
fn saturating_add() {
    assert_eq!(5.seconds().saturating_add(5.seconds()), 10.seconds());
    assert_eq!(Duration::MAX.saturating_add(1.nanoseconds()), Duration::MAX);
    assert_eq!(Duration::MAX.saturating_add(1.seconds()), Duration::MAX);
    assert_eq!(
        Duration::MIN.saturating_add((-1).nanoseconds()),
        Duration::MIN
    );
    assert_eq!(Duration::MIN.saturating_add((-1).seconds()), Duration::MIN);
    assert_eq!((-5).seconds().saturating_add(5.seconds()), Duration::ZERO);
    assert_eq!(
        1_600.milliseconds().saturating_add(1_600.milliseconds()),
        3_200.milliseconds()
    );
    assert_eq!(
        1.seconds().saturating_add((-1).milliseconds()),
        (999).milliseconds()
    );
}

#[test]
fn saturating_sub() {
    assert_eq!(5.seconds().saturating_sub(5.seconds()), Duration::ZERO);
    assert_eq!(Duration::MIN.saturating_sub(1.nanoseconds()), Duration::MIN);
    assert_eq!(
        Duration::MAX.saturating_sub((-1).nanoseconds()),
        Duration::MAX
    );
    assert_eq!(Duration::MAX.saturating_sub((-1).seconds()), Duration::MAX);
    assert_eq!(5.seconds().saturating_sub(10.seconds()), (-5).seconds());
    assert_eq!(
        (-1_600).milliseconds().saturating_sub(1_600.milliseconds()),
        (-3_200).milliseconds()
    );
    assert_eq!(0.seconds().saturating_sub(Duration::MIN), Duration::MIN);
    assert_eq!(Duration::MIN.saturating_sub(5.seconds()), Duration::MIN);
    assert_eq!(
        1_200.milliseconds().saturating_sub(600.milliseconds()),
        600.milliseconds()
    );
    assert_eq!(
        (-1_200)
            .milliseconds()
            .saturating_sub((-600).milliseconds()),
        (-600).milliseconds()
    );
}

#[test]
fn saturating_mul() {
    assert_eq!(5.seconds().saturating_mul(2), 10.seconds());
    assert_eq!(5.seconds().saturating_mul(-2), (-10).seconds());
    assert_eq!(5.seconds().saturating_mul(0), Duration::ZERO);
    assert_eq!(Duration::MAX.saturating_mul(2), Duration::MAX);
    assert_eq!(Duration::MIN.saturating_mul(2), Duration::MIN);
    assert_eq!(Duration::MAX.saturating_mul(-2), Duration::MIN);
    assert_eq!(Duration::MIN.saturating_mul(-2), Duration::MAX);
    assert_eq!(
        Duration::new(1_844_674_407_370_955_161, 600_000_000).saturating_mul(5),
        Duration::MAX
    );
    assert_eq!(
        Duration::new(1_844_674_407_370_955_161, 800_000_000).saturating_mul(-5),
        Duration::MIN
    );
}

#[test]
fn time_fn() {
    let (time, value) = Duration::time_fn(|| {
        std::thread::sleep(1.std_milliseconds());
        0
    });

    assert!(time >= 1.milliseconds());
    assert_eq!(value, 0);
}

#[test]
fn display() {
    assert_eq!(0.seconds().to_string(), "0s");
    assert_eq!(60.days().to_string(), "60d");
    assert_eq!((-48).hours().to_string(), "-2d");
    assert_eq!(48.hours().to_string(), "2d");
    assert_eq!(1.minutes().to_string(), "1m");
    assert_eq!(10.minutes().to_string(), "10m");
    assert_eq!(1.seconds().to_string(), "1s");
    assert_eq!(10.seconds().to_string(), "10s");
    assert_eq!(1.milliseconds().to_string(), "1ms");
    assert_eq!(10.milliseconds().to_string(), "10ms");
    assert_eq!(100.milliseconds().to_string(), "100ms");
    assert_eq!(1.microseconds().to_string(), "1µs");
    assert_eq!(10.microseconds().to_string(), "10µs");
    assert_eq!(100.microseconds().to_string(), "100µs");
    assert_eq!(1.nanoseconds().to_string(), "1ns");
    assert_eq!(10.nanoseconds().to_string(), "10ns");
    assert_eq!(100.nanoseconds().to_string(), "100ns");

    assert_eq!(1.days().to_string(), "1d");
    assert_eq!(26.hours().to_string(), "1d2h");
    assert_eq!(1_563.minutes().to_string(), "1d2h3m");
    assert_eq!(93_784.seconds().to_string(), "1d2h3m4s");
    assert_eq!(93_784_005.milliseconds().to_string(), "1d2h3m4s5ms");
    assert_eq!(93_784_005_006.microseconds().to_string(), "1d2h3m4s5ms6µs");
    assert_eq!(
        93_784_005_006_007.nanoseconds().to_string(),
        "1d2h3m4s5ms6µs7ns"
    );

    assert_eq!(format!("{:.3}", 0.seconds()), "0.000s");
    assert_eq!(format!("{:.3}", 60.days()), "60.000d");
    assert_eq!(format!("{:.3}", (-48).hours()), "-2.000d");
    assert_eq!(format!("{:.3}", 48.hours()), "2.000d");
    assert_eq!(format!("{:.3}", 1.minutes()), "1.000m");
    assert_eq!(format!("{:.3}", 10.minutes()), "10.000m");
    assert_eq!(format!("{:.3}", 1.seconds()), "1.000s");
    assert_eq!(format!("{:.3}", 10.seconds()), "10.000s");
    assert_eq!(format!("{:.3}", 1.milliseconds()), "1.000ms");
    assert_eq!(format!("{:.3}", 10.milliseconds()), "10.000ms");
    assert_eq!(format!("{:.3}", 100.milliseconds()), "100.000ms");
    assert_eq!(format!("{:.3}", 1.microseconds()), "1.000µs");
    assert_eq!(format!("{:.3}", 10.microseconds()), "10.000µs");
    assert_eq!(format!("{:.3}", 100.microseconds()), "100.000µs");
    assert_eq!(format!("{:.3}", 1.nanoseconds()), "1.000ns");
    assert_eq!(format!("{:.3}", 10.nanoseconds()), "10.000ns");
    assert_eq!(format!("{:.3}", 100.nanoseconds()), "100.000ns");

    assert_eq!(format!("{:.3}", 1.days()), "1.000d");
    assert_eq!(format!("{:.3}", 26.hours()), "1.083d");
    assert_eq!(format!("{:.4}", 1_563.minutes()), "1.0854d");
    assert_eq!(format!("{:.5}", 93_784.seconds()), "1.08546d");
    assert_eq!(format!("{:.6}", 93_784_005.milliseconds()), "1.085463d");
    assert_eq!(
        format!("{:.9}", 93_784_005_006.microseconds()),
        "1.085463021d"
    );
    assert_eq!(
        format!("{:.12}", 93_784_005_006_007.nanoseconds()),
        "1.085463020903d"
    );
}

#[test]
fn try_from_std_duration() {
    assert_eq!(Duration::try_from(0.std_seconds()), Ok(0.seconds()));
    assert_eq!(Duration::try_from(1.std_seconds()), Ok(1.seconds()));
    assert_eq!(
        Duration::try_from(u64::MAX.std_seconds()),
        Err(error::ConversionRange)
    );
}

#[test]
fn try_to_std_duration() {
    assert_eq!(StdDuration::try_from(0.seconds()), Ok(0.std_seconds()));
    assert_eq!(StdDuration::try_from(1.seconds()), Ok(1.std_seconds()));
    assert!(StdDuration::try_from((-1).seconds()).is_err());
    assert_eq!(
        StdDuration::try_from((-500).milliseconds()),
        Err(error::ConversionRange)
    );
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
fn std_add_assign() {
    let mut duration = 1.std_seconds();
    duration += 1.seconds();
    assert_eq!(duration, 2.seconds());

    let mut duration = 500.std_milliseconds();
    duration += 500.milliseconds();
    assert_eq!(duration, 1.seconds());
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
}

#[test]
#[should_panic]
fn std_sub_assign_overflow() {
    let mut duration = 1.std_seconds();
    duration -= 2.seconds();
}

#[test]
fn mul_int() {
    assert_eq!(1.seconds() * 2, 2.seconds());
    assert_eq!(1.seconds() * -2, (-2).seconds());

    assert_panic!(Duration::MAX * 2);
    assert_panic!(Duration::MIN * 2);
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
    assert_eq!(0.seconds().cmp(&0.seconds()), Ordering::Equal);
    assert_eq!(1.seconds().cmp(&0.seconds()), Ordering::Greater);
    assert_eq!(1.seconds().cmp(&(-1).seconds()), Ordering::Greater);
    assert_eq!((-1).seconds().cmp(&1.seconds()), Ordering::Less);
    assert_eq!(0.seconds().cmp(&(-1).seconds()), Ordering::Greater);
    assert_eq!(0.seconds().cmp(&1.seconds()), Ordering::Less);
    assert_eq!((-1).seconds().cmp(&0.seconds()), Ordering::Less);
    assert_eq!(1.minutes().cmp(&1.seconds()), Ordering::Greater);
    assert_eq!((-1).minutes().cmp(&(-1).seconds()), Ordering::Less);
    assert_eq!(100.nanoseconds().cmp(&200.nanoseconds()), Ordering::Less);
    assert_eq!(
        (-100).nanoseconds().cmp(&(-200).nanoseconds()),
        Ordering::Greater
    );
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

#[test]
fn sum_iter_ref() {
    let i = vec![1.6.seconds(), 1.6.seconds()];
    let sum = i.iter().sum::<Duration>();
    assert_eq!(sum, 3.2.seconds());
}

#[test]
fn sum_iter() {
    let i = vec![1.6.seconds(), 1.6.seconds()];
    let sum = i.into_iter().sum::<Duration>();
    assert_eq!(sum, 3.2.seconds());
}
