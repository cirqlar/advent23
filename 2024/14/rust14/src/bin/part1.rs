use rust14_24::part1;

fn main() {
    // let input = include_bytes!("../../../input/part1_example.txt");
    // let wide = 11;
    // let tall = 7;
    let input = include_bytes!("../../../input/part1.txt");
    let wide = 101;
    let tall = 103;

    let answer = part1::process(input, wide, tall);

    println!("Answer: {answer}");
}
