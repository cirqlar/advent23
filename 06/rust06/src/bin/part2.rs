fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> usize {
    let mut lines: std::str::Lines<'_> = input.lines();
    let time = lines
        .next()
        .expect("Should have time")
        .split(':')
        .nth(1)
        .expect("should have values")
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .expect("should be parsable");
    let distance = lines
        .next()
        .expect("Should have time")
        .split(':')
        .nth(1)
        .expect("should have values")
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .expect("should be parsable");

    let root: f64 = ((time.pow(2) - (4 * distance)) as f64).sqrt();
    let bound_1_f64 = ((time as f64) - root) / 2.0;
    let mut bound_1 = bound_1_f64.ceil() as usize;
    let bound_2_f64 = ((time as f64) + root) / 2.0;
    let bound_2 = bound_2_f64.ceil() as usize;

    if bound_1_f64 % (bound_1 as f64) == 0.0 {
        bound_1 += 1;
    }
    (bound_1..bound_2).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d06_part_2_is_correct() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(process(input), 71503);
    }
}
