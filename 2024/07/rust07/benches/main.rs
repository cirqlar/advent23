use rust07_24::{part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");

    {
        let mut group = c.benchmark_group("day07_part1");

        group.bench_function("intial", |b| b.iter(|| part1::process(input)));
        group.bench_function("opt", |b| b.iter(|| part1::process_opt(input)));
    }

    {
        let mut group = c.benchmark_group("day07_part2");

        group.bench_function("intial", |b| b.iter(|| part2::process(input)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
