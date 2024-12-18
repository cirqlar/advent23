use rust18_24::{part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");
    let grid_size = 70;
    let count = 1024;

    {
        let mut group = c.benchmark_group("day18_part1");

        group.bench_function("default", |b| {
            b.iter(|| part1::process_bfs(input, grid_size, count))
        });
    }

    {
        let mut group = c.benchmark_group("day18_part2");

        group.bench_function("default", |b| {
            b.iter(|| part2::process(input, grid_size, count))
        });
        group.bench_function("find_first", |b| {
            b.iter(|| part2::process_bfs(input, grid_size, count))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
