use rust25_24::part1;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");

    {
        let mut group = c.benchmark_group("day25_part1");

        group.bench_function("default", |b| b.iter(|| part1::process(input)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
