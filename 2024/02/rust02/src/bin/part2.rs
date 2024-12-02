use rust02_24::part2;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = part2::process(input);

    println!("Answer: {answer}");
}
