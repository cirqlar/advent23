use rust05_24::{parse, part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");
    let split_line_num = 1177;

    let mut group = c.benchmark_group("day05_parsing");

    group.bench_function("initial", |b| {
        b.iter(|| parse::parse_rules::<u16>(input, split_line_num))
    });
    group.bench_function("vec", |b| {
        b.iter(|| parse::parse_rules_2::<u16>(input, split_line_num))
    });

    drop(group);

    let rule_map = parse::parse_rules(input, split_line_num);
    let rule_map_2 = parse::parse_rules_2(input, split_line_num);

    let mut group = c.benchmark_group("day05_part1");

    group.bench_function("initial", |b| {
        b.iter(|| part1::process(input, split_line_num, &rule_map))
    });
    group.bench_function("par", |b| {
        b.iter(|| part1::process_par(input, split_line_num, &rule_map))
    });
    group.bench_function("par2", |b| {
        b.iter(|| part1::process_par_2(input, split_line_num, &rule_map))
    });
    group.bench_function("par2vec", |b| {
        b.iter(|| part1::process_par_2_vec(input, split_line_num, &rule_map_2))
    });

    drop(group);

    let mut group = c.benchmark_group("day05_part2");

    group.bench_function("initial", |b| {
        b.iter(|| part2::process(input, split_line_num, &rule_map))
    });
    group.bench_function("par", |b| {
        b.iter(|| part2::process_par(input, split_line_num, &rule_map))
    });
    group.bench_function("par2", |b| {
        b.iter(|| part2::process_par_2(input, split_line_num, &rule_map))
    });
    group.bench_function("par2vec", |b| {
        b.iter(|| part2::process_par_2_vec(input, split_line_num, &rule_map_2))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
