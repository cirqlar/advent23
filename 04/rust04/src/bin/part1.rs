fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer: {answer}");
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut rem = line.split(": ").nth(1).expect("should exist").split(" | ");
            (
                rem.next().expect("should have winning nums"),
                rem.next().expect("should have nums we have"),
            )
        })
        .map(|nums| {
            (
                nums.0
                    .split(' ')
                    .filter(|num| !num.is_empty())
                    .map(|num| num.parse::<i32>().expect("should be a num a"))
                    .collect::<Vec<_>>(),
                nums.1
                    .split(' ')
                    .filter(|num| !num.is_empty())
                    .map(|num| num.parse::<i32>().expect("should be a num b"))
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(wins, have)| {
            let haves = have.iter().filter(|num| wins.contains(num)).count();
            if haves == 0 {
                0
            } else {
                2_i32.pow((haves - 1).try_into().expect("should be small enough"))
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d04_part_1_is_correct() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(process(input), 13);
    }
}
