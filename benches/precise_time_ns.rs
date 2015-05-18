#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_precise_time_ns(b: &mut Bencher) {
    b.iter(|| precise_time_ns())
}
