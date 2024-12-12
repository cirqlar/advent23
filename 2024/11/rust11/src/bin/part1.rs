use rust11_24::part1;

fn main() {
    let input = include_bytes!("../../../input/part1_example.txt");
    // let input = include_bytes!("../../../input/part1.txt");

    let answer = part1::process(input, 25);

    println!("Answer: {answer}");
}
