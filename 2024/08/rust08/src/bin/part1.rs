use rust08_24::{parse, part1};

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 12;
    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 50;

    let (antenna_keys, antenna_vals) = parse::process_2(input);

    let answer = part1::process(&antenna_keys, &antenna_vals, grid_size, input.len());

    println!("Answer: {answer}");
}
