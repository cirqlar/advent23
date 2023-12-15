fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> u32 {
    input
        .split(',')
        .map(|seq| {
            let seq = seq.trim();

            let mut current_value: u32 = 0;
            for c in seq.chars() {
                current_value += (c as u8) as u32;
                current_value *= 17;
                current_value %= 256;
            }

            current_value
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320)]
    fn d15_part_1_is_correct(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(process(input), expected)
    }
}
