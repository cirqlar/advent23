use rust08_24::{parse, part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");
    let grid_size = 50;

    {
        let mut group = c.benchmark_group("day08_parse");

        group.bench_function("default", |b| b.iter(|| parse::process_2(input)));
    }

    let (antenna_keys, antenna_vals) = parse::process_2(input);

    {
        let mut group = c.benchmark_group("day08_part1");

        group.bench_function("default", |b| {
            b.iter(|| part1::process(&antenna_keys, &antenna_vals, grid_size, input.len()))
        });
    }

    {
        let mut group = c.benchmark_group("day08_part2");

        group.bench_function("default", |b| {
            b.iter(|| part2::process(&antenna_keys, &antenna_vals, grid_size, input.len()))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
