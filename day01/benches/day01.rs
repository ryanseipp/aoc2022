use std::fs;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day01::*;

fn benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("File input.txt should exist");

    let mut group = c.benchmark_group("day_one");
    group.bench_with_input(BenchmarkId::new("part_one", "iterator"), &input, |b, i| {
        b.iter(|| part_one(i))
    });
    group.bench_with_input(
        BenchmarkId::new("part_one", "imperative"),
        &input,
        |b, i| b.iter(|| part_one_imperative(i)),
    );
    group.bench_with_input(BenchmarkId::new("part_one", "bitwise"), &input, |b, i| {
        b.iter(|| part_one_bitwise(i))
    });
    group.bench_with_input(BenchmarkId::new("part_two", "iterator"), &input, |b, i| {
        b.iter(|| part_two(i))
    });
    group.bench_with_input(
        BenchmarkId::new("part_two", "imperative"),
        &input,
        |b, i| b.iter(|| part_two_imperative(i)),
    );
    group.bench_with_input(BenchmarkId::new("part_two", "bitwise"), &input, |b, i| {
        b.iter(|| part_two_bitwise(i))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
