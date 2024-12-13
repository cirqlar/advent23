use rust11_24::part2;

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    let input = include_bytes!("../../../input/part1.txt");

    let times = 75;

    let answer = part2::process(input, times);

    println!("Answer: {answer}");
}
