use rust17_24::part2;

fn main() {
    // let input = include_bytes!("../../../input/part2_example.txt");
    let input = include_bytes!("../../../input/part1.txt");

    let answer = part2::process(input);

    println!("Answer: {answer}");
}
