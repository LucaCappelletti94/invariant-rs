//! Criterion Benchmarks for ilog2 with and without the use of invariant! macro.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use invariant_rs::invariant;

fn ilog2_with_invariant(x: usize) -> u32 {
    invariant!(x > 0, "x must be positive");
    x.ilog2()
}

fn ilog2_without_invariant(x: usize) -> u32 {
    debug_assert!(x > 0);
    x.ilog2()
}

fn ilog2_with_only_assert(x: usize) -> u32 {
    assert!(x > 0);
    x.ilog2()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ilog2_with_invariant", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 1..1000 {
                sum += ilog2_with_invariant(black_box(i));
            }
            sum
        })
    });
    c.bench_function("ilog2_without_invariant", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 1..1000 {
                sum += ilog2_without_invariant(black_box(i));
            }
            sum
        })
    });
    c.bench_function("ilog2_with_only_assert", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 1..1000 {
                sum += ilog2_with_only_assert(black_box(i));
            }
            sum
        })
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
