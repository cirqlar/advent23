use rust06_24::{parse, part1};

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 10;

    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 130;

    let input = parse::process(input);

    let answer = part1::process(&input, grid_size);

    println!("Answer: {answer}");
}
