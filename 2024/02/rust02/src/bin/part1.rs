use rust02_24::part1;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = part1::process_par_lines_windows(input);

    println!("Answer: {answer}");
}
