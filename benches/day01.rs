use criterion::{criterion_group, criterion_main, Criterion};

extern crate advent_of_code_2019;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part1", |b| {
        b.iter(|| advent_of_code_2019::days::day01::part1)
    });
    c.bench_function("part2", |b| {
        b.iter(|| advent_of_code_2019::days::day01::part2)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
