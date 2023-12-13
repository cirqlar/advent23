use std::iter;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn check_horizontal(v: &[Vec<char>]) -> Option<usize> {
    for loc in 0..(v.len() - 1) {
        let split_a = &v[..(loc + 1)];
        let split_b = &v[(loc + 1)..];

        let mut broke = false;
        let mut changed = false;
        for (v_a, v_b) in split_a.iter().rev().zip(split_b.iter()) {
            for (a, b) in v_a.iter().zip(v_b.iter()) {
                if a != b {
                    if changed {
                        broke = true;
                        break;
                    }
                    changed = true;
                }
            }
            if broke {
                break;
            }
        }
        if !broke && changed {
            return Some(loc);
        }
    }

    None
}

fn check_vertical(v: &[Vec<char>]) -> Option<usize> {
    let mut iters = v.iter().map(|line| line.iter()).collect::<Vec<_>>();
    let transformed = iter::from_fn(move || {
        let mut items = Vec::new();

        for iter in &mut iters {
            match iter.next() {
                Some(item) => items.push(*item),
                None => return None,
            }
        }

        Some(items)
    })
    .collect::<Vec<_>>();

    check_horizontal(&transformed)
}

fn process(input: &str) -> usize {
    let n_line = if cfg!(windows) & !cfg!(test) {
        "\r\n\r\n"
    } else {
        "\n\n"
    };
    input
        .split(n_line)
        .map(|grid| {
            let grid = grid
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let mut answer = 0;
            let hor = check_horizontal(&grid);
            let ver = check_vertical(&grid);

            if let Some(hor) = hor {
                answer += (hor + 1) * 100;
            }
            if let Some(ver) = ver {
                answer += ver + 1;
            }

            println!("Answer {} for hor {:?} and ver {:?}", answer, hor, ver);

            answer
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d13_part_1_is_correct() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(process(input), 400);
    }
}
