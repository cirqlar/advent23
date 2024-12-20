use rust20_24::{part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");
    let grid_size = 141;
    let save = 100;

    {
        let mut group = c.benchmark_group("day20_part1");

        group.bench_function("default", |b| {
            b.iter(|| part1::process(input, grid_size, save))
        });
        group.bench_function("v2", |b| {
            b.iter(|| part1::process_v2(input, grid_size, save))
        });
    }

    {
        let mut group = c.benchmark_group("day20_part2");

        group.bench_function("default", |b| {
            b.iter(|| part2::process(input, grid_size, save))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
