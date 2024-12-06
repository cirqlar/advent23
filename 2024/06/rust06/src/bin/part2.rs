use rust06_24::{parse, part2};

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 10;

    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 130;

    let input = parse::process(input);
    let possibles = parse::process_two(&input);

    let answer = part2::process(&input, grid_size, &possibles);

    println!("Answer: {answer}");
}
