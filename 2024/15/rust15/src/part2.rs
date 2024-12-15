use std::fmt::Write;

use crate::NEWLINE_OFFSET;

fn move_up(indices: &[usize], grid: &mut [u8], grid_size: usize) -> bool {
    let mut our_next_indices = vec![];

    let mut must_check = false;

    for index in indices {
        let next = index - grid_size;
        if !our_next_indices.contains(&next) {
            match grid[next] {
                b'#' => return false,
                b'[' => {
                    must_check = true;
                    our_next_indices.push(next);
                    our_next_indices.push(next + 1);
                }
                b']' => {
                    must_check = true;
                    our_next_indices.push(next);
                    our_next_indices.push(next - 1);
                }
                b'.' => {
                    // our_next_indices.push(next);
                    // do nothing
                }
                _ => unreachable!("Should have handled all chars"),
            }
        }
    }

    let mut can_move = true;

    if must_check {
        can_move = move_up(&our_next_indices, grid, grid_size);
    }

    if can_move {
        for index in indices {
            let next = index - grid_size;

            grid.swap(*index, next);
        }
    }

    can_move
}

fn move_down(indices: &[usize], grid: &mut [u8], grid_size: usize) -> bool {
    let mut our_next_indices = vec![];

    let mut must_check = false;

    for index in indices {
        let next = index + grid_size;
        if !our_next_indices.contains(&next) {
            match grid[next] {
                b'#' => return false,
                b'[' => {
                    must_check = true;
                    our_next_indices.push(next);
                    our_next_indices.push(next + 1);
                }
                b']' => {
                    must_check = true;
                    our_next_indices.push(next);
                    our_next_indices.push(next - 1);
                }
                b'.' => {
                    // our_next_indices.push(next);
                    // do nothing
                }
                _ => unreachable!("Should have handled all chars"),
            }
        }
    }

    let mut can_move = true;

    if must_check {
        can_move = move_down(&our_next_indices, grid, grid_size);
    }

    if can_move {
        for index in indices {
            let next = index + grid_size;

            grid.swap(*index, next);
        }
    }

    can_move
}

fn do_command(ch: &u8, grid: &mut [u8], grid_size: usize, starting_index: &mut usize) {
    match *ch {
        b'<' => {
            let a = (*starting_index / grid_size) * grid_size;

            let pos = *starting_index
                - grid[a..=*starting_index]
                    .iter()
                    .rev()
                    .position(|ch| ch == &b'.' || ch == &b'#')
                    .expect("Should always find something");

            if grid[pos] == b'.' {
                let mut prev = b'.';
                for i in (pos..=*starting_index).rev() {
                    std::mem::swap(&mut grid[i], &mut prev);
                }
                *starting_index -= 1;
            }
        }
        b'>' => {
            let a = ((*starting_index / grid_size) + 1) * grid_size;

            let pos = *starting_index
                + grid[*starting_index..a]
                    .iter()
                    .position(|ch| ch == &b'.' || ch == &b'#')
                    .expect("Should always find something");

            if grid[pos] == b'.' {
                let mut prev = b'.';
                #[expect(clippy::needless_range_loop)]
                for i in *starting_index..=pos {
                    std::mem::swap(&mut grid[i], &mut prev);
                }
                *starting_index += 1;
            }
        }
        b'^' => {
            let start = [*starting_index];
            let did_move = move_up(&start, grid, grid_size);

            if did_move {
                *starting_index -= grid_size;
            }
        }
        b'v' => {
            let start = [*starting_index];
            let did_move = move_down(&start, grid, grid_size);

            if did_move {
                *starting_index += grid_size;
            }
        }
        _ => unreachable!("We should only have those chars"),
    }
}

pub fn process(input: &[u8], grid_size: usize) -> usize {
    let actual_grid_size = grid_size + NEWLINE_OFFSET;

    let grid_end = actual_grid_size * grid_size;
    let mut grid = input[..grid_end]
        .iter()
        .filter(|ch| !ch.is_ascii_whitespace())
        .flat_map(|ch| match ch {
            b'#' => [b'#', b'#'],
            b'O' => [b'[', b']'],
            b'.' => [b'.', b'.'],
            b'@' => [b'@', b'.'],
            _ => unreachable!("Should have handled all chars"),
        })
        .collect::<Vec<_>>();
    let new_grid_size = grid_size * 2;

    let commands = &input[grid_end..];

    let mut starting_index = grid
        .iter()
        .position(|ch| ch == &b'@')
        .expect("Starting Position");

    commands
        .iter()
        .filter(|ch| !ch.is_ascii_whitespace())
        .for_each(|ch| do_command(ch, &mut grid, new_grid_size, &mut starting_index));

    // Vis grid
    // for y in 0..grid_size {
    //     for x in 0..new_grid_size {
    //         print!("{}", grid[y * new_grid_size + x] as char);
    //     }
    //     println!();
    // }

    grid.into_iter()
        .enumerate()
        .map(|(index, ch)| {
            if ch == b'[' {
                let y = index / new_grid_size;
                let x = index % new_grid_size;

                (100 * y) + x
            } else {
                0
            }
        })
        .sum()
}

pub fn process_vis(input: &[u8], grid_size: usize) -> usize {
    let actual_grid_size = grid_size + NEWLINE_OFFSET;

    let grid_end = actual_grid_size * grid_size;
    let mut grid = input[..grid_end]
        .iter()
        .filter(|ch| !ch.is_ascii_whitespace())
        .flat_map(|ch| match ch {
            b'#' => [b'#', b'#'],
            b'O' => [b'[', b']'],
            b'.' => [b'.', b'.'],
            b'@' => [b'@', b'.'],
            _ => unreachable!("Should have handled all chars"),
        })
        .collect::<Vec<_>>();
    let new_grid_size = grid_size * 2;

    let commands = &input[grid_end..];

    let mut starting_index = grid
        .iter()
        .position(|ch| ch == &b'@')
        .expect("Starting Position");

    // Clear Screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for ch in commands.iter().filter(|ch| !ch.is_ascii_whitespace()) {
        do_command(ch, &mut grid, new_grid_size, &mut starting_index);

        // Move cursor to start
        print!("\x1b[1;1H");

        // Vis Grid
        let mut strrr = String::new();
        for y in 0..grid_size {
            for x in 0..new_grid_size {
                write!(&mut strrr, "{}", grid[y * new_grid_size + x] as char).unwrap();
            }
            writeln!(&mut strrr).unwrap();
        }
        print!("{}", strrr);

        // sleep 66ms (15fps)
        std::thread::sleep(std::time::Duration::from_millis(66));
    }

    grid.into_iter()
        .enumerate()
        .map(|(index, ch)| {
            if ch == b'[' {
                let y = index / new_grid_size;
                let x = index % new_grid_size;

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

        assert_eq!(answer, 9021);
    }
}
