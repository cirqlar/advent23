use rust20_24::part1;

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 15;
    // let save = 20;
    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 141;
    let save = 100;

    let answer = part1::process(input, grid_size, save);

    println!("Answer: {answer}");
}
