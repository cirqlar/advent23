use crate::NEWLINE_OFFSET;

pub fn process(input: &[u8], grid_size: usize) -> usize {
    let actual_grid_size = grid_size + NEWLINE_OFFSET;

    let grid_end = actual_grid_size * grid_size;
    let mut grid = input[..grid_end].to_vec();
    let commands = &input[grid_end..];

    let mut starting_index = grid
        .iter()
        .position(|ch| ch == &b'@')
        .expect("Starting Position");

    commands
        .iter()
        .filter(|ch| !ch.is_ascii_whitespace())
        .for_each(|ch| match *ch {
            b'<' => {
                let a = (starting_index / actual_grid_size) * actual_grid_size;

                let pos = starting_index
                    - grid[a..=starting_index]
                        .iter()
                        .rev()
                        .position(|ch| ch == &b'.' || ch == &b'#')
                        .expect("Should always find something");

                if grid[pos] == b'.' {
                    let mut prev = b'.';
                    for i in (pos..=starting_index).rev() {
                        std::mem::swap(&mut grid[i], &mut prev);
                    }
                    starting_index -= 1;
                }
            }
            b'>' => {
                let a = ((starting_index / actual_grid_size) + 1) * actual_grid_size;

                let pos = starting_index
                    + grid[starting_index..a]
                        .iter()
                        .position(|ch| ch == &b'.' || ch == &b'#')
                        .expect("Should always find something");

                if grid[pos] == b'.' {
                    let mut prev = b'.';
                    #[expect(clippy::needless_range_loop)]
                    for i in starting_index..=pos {
                        std::mem::swap(&mut grid[i], &mut prev);
                    }
                    starting_index += 1;
                }
            }
            b'^' => {
                let pos = starting_index
                    - (actual_grid_size
                        * grid[..=starting_index]
                            .iter()
                            .rev()
                            .step_by(actual_grid_size)
                            .position(|ch| ch == &b'.' || ch == &b'#')
                            .expect("Should always find something"));

                if grid[pos] == b'.' {
                    let mut prev = b'.';
                    for i in (pos..=starting_index).rev().step_by(actual_grid_size) {
                        std::mem::swap(&mut grid[i], &mut prev);
                    }
                    starting_index -= actual_grid_size;
                }
            }
            b'v' => {
                let pos = starting_index
                    + (actual_grid_size
                        * grid[starting_index..]
                            .iter()
                            .step_by(actual_grid_size)
                            .position(|ch| ch == &b'.' || ch == &b'#')
                            .expect("Should always find something"));

                if grid[pos] == b'.' {
                    let mut prev = b'.';
                    for i in (starting_index..=pos).step_by(actual_grid_size) {
                        std::mem::swap(&mut grid[i], &mut prev);
                    }
                    starting_index += actual_grid_size;
                }
            }
            _ => unreachable!("We should only have those chars"),
        });

    // for y in 0..grid_size {
    //     for x in 0..grid_size {
    //         print!("{}", grid[y * actual_grid_size + x] as char);
    //     }
    //     println!();
    // }

    grid.into_iter()
        .enumerate()
        .map(|(index, ch)| {
            if ch == b'O' {
                let y = index / actual_grid_size;
                let x = index % actual_grid_size;

                (100 * y) + x
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");
        let grid_size = 10;

        let answer = super::process(input, grid_size);

        assert_eq!(answer, 10092);
    }
}
