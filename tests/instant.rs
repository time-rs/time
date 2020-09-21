#![cfg(feature = "std")]

use std::{thread, time::Instant as StdInstant};
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    Duration, Instant,
};

#[test]
fn elapsed() {
    let instant = Instant::now();
    thread::sleep(1.std_milliseconds());
    assert!(instant.elapsed() >= 1.milliseconds());
}

#[test]
fn checked_add() {
    let now = Instant::now();
    assert_eq!(now.checked_add(0.seconds()), Some(now));
    assert_eq!(now.checked_add(5.seconds()), Some(now + 5.seconds()));
    assert_eq!(now.checked_add((-5).seconds()), Some(now + (-5).seconds()));
}

#[test]
fn checked_sub() {
    let now = Instant::now();
    assert_eq!(now.checked_sub(0.seconds()), Some(now));
    assert_eq!(now.checked_sub(5.seconds()), Some(now - 5.seconds()));
    assert_eq!(now.checked_sub((-5).seconds()), Some(now - (-5).seconds()));
}

#[test]
fn from_std() {
    let now_time = Instant::now();
    let now_std = StdInstant::from(now_time);
    assert_eq!(now_time, now_std);
}

#[test]
fn to_std() {
    let now_std = StdInstant::now();
    let now_time = Instant::from(now_std);
    assert_eq!(now_time, now_std);
}

#[test]
fn sub() {
    let start = Instant::now();
    thread::sleep(1.std_milliseconds());
    assert!(Instant::now() - start >= 1.milliseconds());
}

#[test]
fn sub_std() {
    let start = StdInstant::now();
    thread::sleep(1.std_milliseconds());
    assert!(Instant::now() - start >= 1.milliseconds());
}

#[test]
fn std_sub() {
    let start = Instant::now();
    thread::sleep(1.std_milliseconds());
    assert!(StdInstant::now() - start >= 1.milliseconds());
}

#[test]
fn add_duration() {
    let start = Instant::now();
    assert!(start + 0.seconds() <= Instant::now());
    thread::sleep(1.std_milliseconds());
    assert!(start + 1.milliseconds() <= Instant::now());
}

#[test]
fn std_add_duration() {
    let start = StdInstant::now();
    thread::sleep(1.std_milliseconds());
    assert!(start + 1.milliseconds() <= StdInstant::now());
}

#[test]
fn add_std_duration() {
    let start = Instant::now();
    thread::sleep(1.std_milliseconds());
    assert!(start + 1.std_milliseconds() <= Instant::now());
}

#[test]
fn add_assign_duration() {
    let mut start = Instant::now();
    thread::sleep(1.std_milliseconds());
    start += 1.milliseconds();
    assert!(start <= Instant::now());
}

#[test]
fn std_add_assign_duration() {
    let mut start = StdInstant::now();
    thread::sleep(1.std_milliseconds());
    start += 1.milliseconds();
    assert!(start <= StdInstant::now());
}

#[test]
fn add_assign_std_duration() {
    let mut start = Instant::now();
    thread::sleep(1.std_milliseconds());
    start += 1.std_milliseconds();
    assert!(start <= Instant::now());
}

#[test]
fn sub_duration() {
    let instant = Instant::now();
    assert!(instant - 100.milliseconds() <= Instant::now());
}

#[test]
fn std_sub_duration() {
    let instant = StdInstant::now();
    assert!(instant - 100.milliseconds() <= StdInstant::now());
}

#[test]
fn sub_std_duration() {
    let instant = Instant::now();
    assert!(instant - 100.std_milliseconds() <= Instant::now());
}

#[test]
fn sub_assign_duration() {
    let mut instant = Instant::now();
    instant -= 100.milliseconds();
    assert!(instant <= Instant::now());
}

#[test]
fn std_sub_assign_duration() {
    let mut instant = StdInstant::now();
    instant -= 100.milliseconds();
    assert!(instant <= StdInstant::now());
}

#[test]
fn sub_assign_std_duration() {
    let mut instant = Instant::now();
    instant -= 100.std_milliseconds();
    assert!(instant <= Instant::now());
}

#[test]
fn eq_std() {
    let now_time = Instant::now();
    let now_std = StdInstant::from(now_time);
    assert_eq!(now_time, now_std);
}

#[test]
fn std_eq() {
    let now_time = Instant::now();
    let now_std = StdInstant::from(now_time);
    assert_eq!(now_std, now_time);
}

#[test]
fn ord_std() {
    let now_time = Instant::now();
    let now_std = StdInstant::from(now_time) + 1.seconds();
    assert!(now_time < now_std);

    let now_time = Instant::now();
    let now_std = StdInstant::from(now_time) - 1.seconds();
    assert!(now_time > now_std);
}

#[test]
fn std_ord() {
    let now_time = Instant::now();
    let now_std = StdInstant::from(now_time) + 1.seconds();
    assert!(now_std > now_time);

    let now_time = Instant::now();
    let now_std = StdInstant::from(now_time) - 1.seconds();
    assert!(now_std < now_time);
}

#[test]
fn sub_regression() {
    let now = Instant::now();
    let future = now + Duration::seconds(5);
    let past = now - Duration::seconds(5);

    assert_eq!(future - now, Duration::seconds(5));
    assert_eq!(now - past, Duration::seconds(5));
    assert_eq!(future - past, Duration::seconds(10));

    assert_eq!(now - future, Duration::seconds(-5));
    assert_eq!(past - now, Duration::seconds(-5));
    assert_eq!(past - future, Duration::seconds(-10));
}
