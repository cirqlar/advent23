use rust10_24::{part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");
    let grid_size = 50;

    {
        let mut group = c.benchmark_group("day10_part1");

        group.bench_function("default", |b| b.iter(|| part1::process(input, grid_size)));
    }

    {
        let mut group = c.benchmark_group("day10_part2");

        group.bench_function("default", |b| b.iter(|| part2::process(input, grid_size)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
