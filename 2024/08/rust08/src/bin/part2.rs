use rust08_24::{parse, part2};

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 12;
    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 50;

    let (antenna_keys, antenna_vals) = parse::process_2(input);

    let answer = part2::process(&antenna_keys, &antenna_vals, grid_size);

    println!("Answer: {answer}");
}
