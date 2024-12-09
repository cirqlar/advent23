use rust09_24::part2;

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    let input = include_bytes!("../../../input/part1.txt");

    let answer = part2::process_2(input);

    println!("Answer: {answer}");
}
