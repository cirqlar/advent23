use rust11_23::process;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input, 1_000_000);

    println!("Answer {answer}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d11_part_2_is_correct() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(process(input, 100), 8410)
    }
}
