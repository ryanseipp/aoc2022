use std::fs;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day_two::{part_one, part_two};

fn benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("File input.txt should exist");

    c.bench_with_input(BenchmarkId::new("part_one", "real"), &input, |b, i| {
        b.iter(|| part_one(i))
    });
    c.bench_with_input(BenchmarkId::new("part_two", "real"), &input, |b, i| {
        b.iter(|| part_two(i))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
