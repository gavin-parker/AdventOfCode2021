extern crate puzzles;
use criterion::{criterion_group, criterion_main, Criterion};

fn aoc_benches(c: &mut Criterion)
{
    c.bench_function("test", |b| {b.iter(|| ); });
}

criterion_group!(benches, aoc_benches);
criterion_main!(benches);