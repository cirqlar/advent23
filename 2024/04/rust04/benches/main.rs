use rust04_24::{part1, part2};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/part1.txt");

    let mut group = c.benchmark_group("day04_part1");

    group.bench_function("intial", |b| b.iter(|| part1::process(input)));
    group.bench_function("flatpar", |b| b.iter(|| part1::process_flatpar(input)));
    group.bench_function("flatter", |b| b.iter(|| part1::process_flatter(input)));

    drop(group);

    let mut group = c.benchmark_group("day04_part2");

    group.bench_function("intial", |b| b.iter(|| part2::process(input)));
    group.bench_function("flatpar", |b| b.iter(|| part2::process_flatpar(input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// fn main() {
//     divan::main();
// }

// #[divan::bench()]
// fn part1() -> i32 {
//     let input = include_bytes!("../../input/part1.txt");

//     part1::process(input)
// }

// #[divan::bench()]
// fn part1_flatpar() -> i32 {
//     let input = include_bytes!("../../input/part1.txt");

//     part1::process_flatpar(input)
// }

// #[divan::bench()]
// fn part2() -> i32 {
//     let input = include_bytes!("../../input/part1.txt");

//     part2::process(input)
// }

// #[divan::bench()]
// fn part2_flatpar() -> i32 {
//     let input = include_bytes!("../../input/part1.txt");

//     part2::process_flatpar(input)
// }

// #[divan::bench()]
// fn part1_bencher(bencher: divan::Bencher) {
//     let input = include_bytes!("../../input/part1.txt");

//     bencher.bench_local(|| part1::process(input));
// }

// #[divan::bench()]
// fn part1_flatpar_bencher(bencher: divan::Bencher) {
//     let input = include_bytes!("../../input/part1.txt");

//     bencher.bench_local(|| part1::process_flatpar(input));
// }

// #[divan::bench()]
// fn part2_bencher(bencher: divan::Bencher) {
//     let input = include_bytes!("../../input/part1.txt");

//     bencher.bench_local(|| part2::process(input));
// }

// #[divan::bench()]
// fn part2_flatpar_bencher(bencher: divan::Bencher) {
//     let input = include_bytes!("../../input/part1.txt");

//     bencher.bench_local(|| part2::process_flatpar(input));
// }
