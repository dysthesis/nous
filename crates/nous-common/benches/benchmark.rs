use criterion::{criterion_group, criterion_main, Criterion};
use nous_common::add;
use std::hint::black_box;

fn add_benchmark(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| {
            let x = black_box(42i64);
            let y = black_box(24i64);
            add(x, y)
        })
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
