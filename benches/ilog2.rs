//! Criterion Benchmarks for ilog2 with and without the use of invariant! macro.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use invariant_rs::invariant;

#[inline(never)]
fn ilog2_with_invariant(x: usize) -> u32 {
    invariant!(x > 0, "x must be positive");
    x.ilog2()
}

#[inline(never)]
fn ilog2_without_invariant(x: usize) -> u32 {
    debug_assert!(x > 0, "x must be positive");
    x.ilog2()
}

#[inline(never)]
fn ilog2_with_only_assert(x: usize) -> u32 {
    assert!(x > 0, "x must be positive");
    x.ilog2()
}

#[inline]
fn ilog2_with_invariant_inlined(x: usize) -> u32 {
    invariant!(x > 0, "x must be positive");
    x.ilog2()
}

#[inline]
fn ilog2_without_invariant_inlined(x: usize) -> u32 {
    debug_assert!(x > 0, "x must be positive");
    x.ilog2()
}

#[inline]
fn ilog2_with_only_assert_inlined(x: usize) -> u32 {
    assert!(x > 0, "x must be positive");
    x.ilog2()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ilog2_without_invariant", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_without_invariant(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_with_only_assert", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_with_only_assert(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_with_invariant", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_with_invariant(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_without_invariant_inlined", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_without_invariant_inlined(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_with_only_assert_inlined", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_with_only_assert_inlined(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_with_invariant_inlined", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_with_invariant_inlined(black_box(i));
            }
            xor
        })
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
