use std::iter;

fn main() {
    let input = include_str!("../../../input/part1.txt");
    let answer = process(input);
    println!("Answer {answer}");
}

fn process(input: &str) -> usize {
    let mut iters = input.lines().map(|line| line.chars()).collect::<Vec<_>>();

    let mut columns = iter::from_fn(|| {
        let mut items = Vec::new();

        for iter in &mut iters {
            match iter.next() {
                Some(item) => items.push(item),
                None => return None,
            }
        }

        Some(items)
    })
    .collect::<Vec<_>>();

    columns.iter_mut().for_each(|v| {
        let mut first_availabe = 0;

        for index in 0..v.len() {
            if v[index] == '.' {
                continue;
            }
            if v[index] == 'O' {
                if first_availabe != index {
                    v[first_availabe] = 'O';
                    v[index] = '.';
                }
                first_availabe += 1;
            }
            if v[index] == '#' {
                first_availabe = index + 1;
            }
        }
    });

    columns
        .iter()
        .flat_map(|v| {
            let length = v.len();
            v.iter().enumerate().filter_map(move |(index, ch)| {
                if *ch != 'O' {
                    None
                } else {
                    Some(length - index)
                }
            })
        })
        .sum()

    // for col in columns.iter() {
    //     for ch in col {
    //         print!("{ch}");
    //     }
    //     println!();
    // }

    // 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d14_part_1_is_correct() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(process(input), 136);
    }
}
