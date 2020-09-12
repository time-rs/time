#[test]
#[cfg(all(feature = "std", feature = "deprecated"))]
#[allow(deprecated)]
fn precise_time_s() {
    let _: f64 = time::precise_time_s();
}

#[test]
#[cfg(all(feature = "std", feature = "deprecated"))]
#[allow(deprecated)]
fn precise_time_ns() {
    let _: u64 = time::precise_time_ns();
}
