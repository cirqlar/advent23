use std::str::Lines;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process_line<'a>(lines: &mut Lines<'a>) -> impl Iterator<Item = u32> + 'a {
    lines
        .next()
        .expect("Should have time")
        .split(':')
        .nth(1)
        .expect("should have values")
        .split_ascii_whitespace()
        .map(|num| num.parse::<u32>().expect("should be parsable"))
}

fn process(input: &str) -> usize {
    let mut lines: std::str::Lines<'_> = input.lines();
    let times = process_line(&mut lines);
    let distances = process_line(&mut lines);

    times
        .zip(distances)
        .map(|(time, distance)| {
            let root: f64 = ((time.pow(2) - (4 * distance)) as f64).sqrt();
            let bound_1 = (((time as f64) - root) / 2.0) as u32 + 1;
            let bound_2 = (((time as f64) + root) / 2.0).ceil() as u32;

            (bound_1..bound_2).count()
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d06_part_1_is_correct() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(process(input), 288);
    }
}
