use std::fs;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day06::*;

fn benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("day06/input.txt").expect("File input.txt should exist");

    let mut group = c.benchmark_group("day_six");
    group.bench_with_input(BenchmarkId::new("part_one", "iterator"), &input, |b, i| {
        b.iter(|| part_one(i))
    });
    group.bench_with_input(BenchmarkId::new("part_one", "bitwise"), &input, |b, i| {
        b.iter(|| part_one_bitwise(i))
    });
    group.bench_with_input(
        BenchmarkId::new("part_one", "bitwise_alt"),
        &input,
        |b, i| b.iter(|| part_one_bitwise_alt(i)),
    );
    group.bench_with_input(BenchmarkId::new("part_two", "iterator"), &input, |b, i| {
        b.iter(|| part_two(i))
    });
    group.bench_with_input(BenchmarkId::new("part_two", "bitwise"), &input, |b, i| {
        b.iter(|| part_two_bitwise(i))
    });
    group.bench_with_input(
        BenchmarkId::new("part_two", "bitwise_alt"),
        &input,
        |b, i| b.iter(|| part_two_bitwise_alt(i)),
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
