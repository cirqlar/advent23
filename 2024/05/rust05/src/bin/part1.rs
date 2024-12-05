use rust05_24::{parse, part1};

fn main() {
    let input = include_bytes!("../../../input/part1.txt");
    let split_line_number = 1177;
    let rule_map = parse::parse_rules(input, split_line_number);

    let max = rule_map.values().map(|v| v.len()).max().unwrap();

    let answer = part1::process(input, split_line_number, &rule_map);

    println!("Answer: {answer} and max {max}");
}
