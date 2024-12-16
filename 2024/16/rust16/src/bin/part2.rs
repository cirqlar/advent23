use rust16_24::part2;

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 15;
    // let input = include_bytes!("../../../input/part2_example.txt");
    // let grid_size = 17;
    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 141;

    let answer = part2::process(input, grid_size);

    println!("Answer: {answer}");
}
