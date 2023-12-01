fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer: {}", answer);
}

fn process(input: &str) -> u32 {
    let nums_as_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let answer = input
        .lines()
        .map(|l| {
            let mut nums = Vec::new();

            for (i, c) in l.chars().enumerate() {
                if let Some(num) = c.to_digit(10) {
                    nums.push(num);
                    continue;
                }

                let pos = nums_as_words.iter().position(|a| l[i..].starts_with(a));
                if let Some(pos) = pos {
                    nums.push((pos + 1).try_into().unwrap());
                }
            }

            if !nums.is_empty() {
                nums[0] * 10 + nums[nums.len() - 1]
            } else {
                println!("We got a 0");
                0
            }
        })
        .sum();

    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_2_is_correct() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(process(input), 281)
    }
}
