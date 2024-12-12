use rust11_24::part2;

fn main() {
    let input = include_bytes!("../../../input/part1_example.txt");
    // let input = include_bytes!("../../../input/part1.txt");

    let times = 11;

    // let answer_part_1 = rust11_24::part1::process_return(input, times);
    // println!("Answer 1: {}", answer_part_1.len());
    let answer = part2::process(input, times);

    println!("Answer: {answer}");
}
