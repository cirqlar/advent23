use rust04_24::{part1, part2};

fn main() {
    divan::main();
}

#[divan::bench()]
fn part1() -> i32 {
    let input = include_bytes!("../../input/part1.txt");

    part1::process(input)
}

#[divan::bench()]
fn part1_flatpar() -> i32 {
    let input = include_bytes!("../../input/part1.txt");

    part1::process_flatpar(input)
}

#[divan::bench()]
fn part2() -> i32 {
    let input = include_bytes!("../../input/part1.txt");

    part2::process(input)
}

#[divan::bench()]
fn part2_flatpar() -> i32 {
    let input = include_bytes!("../../input/part1.txt");

    part2::process_flatpar(input)
}
