use rust02_24::{part1, part2};

fn main() {
    divan::main();
}

#[divan::bench()]
fn part1() -> u32 {
    let input = include_str!("../../input/part1.txt");

    part1::process(input)
}

#[divan::bench()]
fn part1_par_lines_windows() -> u32 {
    let input = include_str!("../../input/part1.txt");

    part1::process_par_lines_windows(input)
}

#[divan::bench()]
fn part2() -> u32 {
    let input = include_str!("../../input/part1.txt");

    part2::process(input)
}

#[divan::bench()]
fn part2_par_iter() -> u32 {
    let input = include_str!("../../input/part1.txt");

    part2::process_par_iter(input)
}

#[divan::bench()]
fn part2_windows() -> u32 {
    let input = include_str!("../../input/part1.txt");

    part2::process_windows(input)
}

#[divan::bench()]
fn part2_par_iter_windows() -> u32 {
    let input = include_str!("../../input/part1.txt");

    part2::process_par_iter_windows(input)
}
