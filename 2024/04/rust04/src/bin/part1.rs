use rust04_24::part1;

fn main() {
    let input = include_bytes!("../../../input/part1.txt");

    let answer = part1::process(input);
    let answerb = part1::process_flatpar(input);

    println!("Answer: {answer} and {answerb}");
}
