use rust18_24::part2;

fn main() {
    let input = include_bytes!("../../../input/part1.txt");
    let grid_size = 70;
    let count = 1024;

    let _answer = part2::process(input, grid_size, count);
}
