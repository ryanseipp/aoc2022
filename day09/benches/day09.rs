use std::fs;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day09::*;

fn benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("day09/input.txt").expect("File input.txt should exist");

    let mut group = c.benchmark_group("day_nine");
    group.bench_with_input(BenchmarkId::new("part_one", "iterator"), &input, |b, i| {
        b.iter(|| part_one(i))
    });
    group.bench_with_input(BenchmarkId::new("part_two", "iterator"), &input, |b, i| {
        b.iter(|| part_two(i))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
