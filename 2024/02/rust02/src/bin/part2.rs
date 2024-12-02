use rust02_24::part2;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = part2::process_par_iter_windows(input);

    println!("Answer: {answer}");
}
