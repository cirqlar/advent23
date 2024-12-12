use rust12_24::part2;

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 4;
    // let input = include_bytes!("../../../input/part4_example.txt");
    // let grid_size = 5;
    // let input = include_bytes!("../../../input/part3_example.txt");
    // let grid_size = 10;
    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 140;

    let answer = part2::process(input, grid_size);

    println!("Answer: {answer}");
}
