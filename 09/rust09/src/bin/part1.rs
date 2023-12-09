fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn do_line(v: &[i32]) -> i32 {
    // println!("do line for {:?}", v);
    let mut next = vec![0_i32; v.len() - 1];
    for (index, window) in v.windows(2).enumerate() {
        next[index] = window[1] - window[0];
    }
    v.last().expect("Should have one")
        + if next.iter().all(|a| *a == next[0]) {
            // println!("Return at end {:?}", next);
            next[0]
        } else {
            do_line(&next)
        }
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().expect("numbers"))
                .collect::<Vec<_>>();

            do_line(&line)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d09_part_1_is_correct() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(process(input), 114)
    }
}
