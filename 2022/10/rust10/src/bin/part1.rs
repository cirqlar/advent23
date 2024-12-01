fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn should_ret(cycle: i32) -> bool {
    cycle >= 20 && (cycle - 20) % 40 == 0
}

fn process(input: &str) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let command = iter.next().unwrap();
            let mut numtoadd = 0;

            if let Some(num) = iter.next() {
                let num: i32 = num.parse().unwrap();
                numtoadd = num;
            }
            cycle += 1;

            if command == "noop" && should_ret(cycle) {
                return Some(cycle * x);
            } else if command == "addx" && should_ret(cycle) {
                let ret = cycle * x;
                cycle += 1;
                x += numtoadd;
                return Some(ret);
            } else if command == "addx" {
                cycle += 1;
                if should_ret(cycle) {
                    let ret = cycle * x;
                    x += numtoadd;
                    return Some(ret);
                } else {
                    x += numtoadd;
                }
            }

            None
        })
        .inspect(|v| println!("{}", v))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d10_part_1_is_correct() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        assert_eq!(process(input), 13140);
    }
}
