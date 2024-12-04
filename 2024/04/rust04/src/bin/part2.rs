use rust04_24::part2;

fn main() {
    let input = include_bytes!("../../../input/part1.txt");

    let answer = part2::process(input);

    println!("Answer: {answer}");
}
