use rust22::{bricks_from_str, get_undisintigratable, to_stacked_bricks};
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> usize {
    let bricks = bricks_from_str(input);

    let bricks = to_stacked_bricks(bricks);
    let undisint = get_undisintigratable(&bricks);

    let mut fallen_total = 0;
    for b in undisint.into_iter() {
        let mut fallen: HashSet<_> = [b].into();

        for (ob, sb) in bricks.iter() {
            if !sb.is_empty() && sb.iter().all(|b| fallen.contains(b)) {
                fallen.insert(ob);
            }
        }

        fallen_total += fallen.len() - 1;
    }

    /* 519
    109531 */
    fallen_total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d22_part_2_is_correct() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(process(input), 7);
    }
}
