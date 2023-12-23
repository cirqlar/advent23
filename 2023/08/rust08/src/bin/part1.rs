use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> usize {
    let mut lines = input.lines();
    let steps = lines
        .next()
        .expect("first line")
        .chars()
        .collect::<Vec<_>>();
    lines.next();
    let map = lines
        .map(|line| {
            line.split_once(" = ")
                .map(|(key, value)| {
                    (
                        key,
                        value[1..(value.len() - 1)]
                            .split_once(", ")
                            .expect("left and right"),
                    )
                })
                .expect("key value pair")
        })
        .collect::<HashMap<_, _>>();

    let mut i = 0;
    let mut next_key = "AAA";
    loop {
        let next_step = steps[i % steps.len()];
        let next_keys = map.get(next_key).expect("should exist");
        match next_step {
            'L' => next_key = next_keys.0,
            'R' => next_key = next_keys.1,
            _ => unreachable!(),
        }
        i += 1;
        if next_key == "ZZZ" {
            break;
        }
    }

    i
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d08_part_1_is_correct() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(process(input), 2);
    }

    #[test]
    fn d08_part_1_is_correct_02() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(process(input), 6);
    }
}
