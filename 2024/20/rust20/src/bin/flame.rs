use rust20_24::part2;

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let grid_size = 15;
    // let save = 50;
    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 141;
    let save = 100;

    let _answer = part2::process(input, grid_size, save);
}
