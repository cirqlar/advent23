use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn gcd(a: usize, b: usize) -> usize {
    // println!("comparying {} and {}", a, b);
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
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

    let start_keys = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();
    let mut move_counts = vec![0_usize; start_keys.len()];
    for (index, start_key) in start_keys.iter().enumerate() {
        let mut next_key = **start_key;
        loop {
            let next_step = steps[move_counts[index] % steps.len()];
            let next_keys = map.get(next_key).expect("should exist");
            match next_step {
                'L' => next_key = next_keys.0,
                'R' => next_key = next_keys.1,
                _ => unreachable!(),
            }
            move_counts[index] += 1;
            if next_key.ends_with('Z') {
                break;
            }
        }
    }
    lcm(&move_counts)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d08_part_2_is_correct() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(process(input), 6);
    }
}
