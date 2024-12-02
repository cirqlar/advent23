use rust02_24::part1;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = part1::process(input);

    println!("Answer: {answer}");
}
