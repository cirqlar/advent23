use rust04_24::{part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");

    {
        let mut group = c.benchmark_group("day04_part1");

        group.bench_function("default", |b| b.iter(|| part1::process(input)));
        group.bench_function("flatpar", |b| b.iter(|| part1::process_flatpar(input)));
        group.bench_function("flatter", |b| b.iter(|| part1::process_flatter(input)));
    }

    {
        let mut group = c.benchmark_group("day04_part2");

        group.bench_function("default", |b| b.iter(|| part2::process(input)));
        group.bench_function("flatpar", |b| b.iter(|| part2::process_flatpar(input)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
