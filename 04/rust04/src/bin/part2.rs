fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer: {answer}");
}

fn process(input: &str) -> i32 {
    let mut card_amounts = vec![1; input.lines().count()];
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
        .map(|(wins, have)| have.iter().filter(|num| wins.contains(num)).count())
        .enumerate()
        .for_each(|(index, wins)| {
            let amount_to_add = card_amounts[index];
            for to_add in card_amounts
                .iter_mut()
                .take(index + wins + 1)
                .skip(index + 1)
            {
                *to_add += amount_to_add
            }
        });

    card_amounts.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d04_part_2_is_correct() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(process(input), 30);
    }
}
