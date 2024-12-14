use rust14_24::{part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");
    let wide = 101;
    let tall = 103;

    {
        let mut group = c.benchmark_group("day14_part1");

        group.bench_function("default", |b| b.iter(|| part1::process(input, wide, tall)));
    }

    {
        let mut group = c.benchmark_group("day14_part2");

        group.bench_function("default", |b| b.iter(|| part2::process(input, wide, tall)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
