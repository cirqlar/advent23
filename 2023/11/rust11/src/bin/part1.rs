use rust11_23::process;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input, 2);

    println!("Answer {answer}");
}

#[cfg(test)]
mod test {
    use rust11_23::process;

    #[test]
    fn d11_part_1_is_correct() {
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

        assert_eq!(process(input, 2), 374)
    }
}
