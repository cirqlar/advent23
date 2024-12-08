use rust06_24::{parse, part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");

    let grid_size = 130;

    let mut group = c.benchmark_group("day06_parse");

    group.bench_function("initial", |b| b.iter(|| parse::process(input)));

    drop(group);

    let parsed_input = parse::process(input);

    let mut group = c.benchmark_group("day06_part1");

    group.bench_function("initial", |b| {
        b.iter(|| part1::process(&parsed_input, grid_size))
    });

    group.bench_function("with_return", |b| {
        b.iter(|| part1::process_return(&parsed_input, grid_size))
    });

    drop(group);

    let mut group = c.benchmark_group("day06_parse2");

    group.bench_function("initial", |b| b.iter(|| parse::process_two(&parsed_input)));

    drop(group);

    let possibles = parse::process_two(&parsed_input);
    let possibles2 = part1::process_internal(&parsed_input, grid_size);

    let mut group = c.benchmark_group("day06_part2");

    group.bench_function("initial", |b| {
        b.iter(|| part2::process(&parsed_input, grid_size, &possibles))
    });

    group.bench_function("limited_set", |b| {
        b.iter(|| part2::process_bools(&parsed_input, grid_size, &possibles2))
    });

    group.bench_function("limited_set_and_sum", |b| {
        b.iter(|| part2::process_bools_sum(&parsed_input, grid_size, &possibles2))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
