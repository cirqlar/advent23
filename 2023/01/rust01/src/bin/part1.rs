fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer: {}", answer);
}

fn process(input: &str) -> u32 {
    let answer = input
        .lines()
        .map(|l| {
            let nums: Vec<_> = l.chars().filter_map(|cur| cur.to_digit(10)).collect();
            if !nums.is_empty() {
                nums[0] * 10 + nums[nums.len() - 1]
            } else {
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
    fn d01_part_1_is_correct() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(process(input), 142)
    }
}
