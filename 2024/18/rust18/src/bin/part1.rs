use rust18_24::part1;

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 6;
    // let count = 12;
    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 70;
    let count = 1024;

    let answer = part1::process(input, grid_size, count);

    println!("Answer: {answer}");
}
