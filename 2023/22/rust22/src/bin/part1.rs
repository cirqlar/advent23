use rust22_23::{bricks_from_str, get_undisintigratable, to_stacked_bricks};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> usize {
    let bricks = bricks_from_str(input);
    let num_bricks = bricks.len();

    let bricks = to_stacked_bricks(bricks);
    let undisint = get_undisintigratable(&bricks);

    num_bricks - undisint.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d22_part_1_is_correct() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(process(input), 5);
    }
}
