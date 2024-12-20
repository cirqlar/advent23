use crate::{successors, NEWLINE_OFFSET};
use pathfinding::prelude::bfs;

pub fn process(input: &[u8], grid_size: usize, save: usize) -> usize {
    use rayon::prelude::*;

    let actual_grid_size = grid_size + NEWLINE_OFFSET;

    let start_index = input.iter().position(|ch| ch == &b'S').unwrap();
    let end_index = input.iter().position(|ch| ch == &b'E').unwrap();

    let mut lengths = vec![None; input.len()];
    let og_path = bfs(
        &start_index,
        |n| successors(*n, grid_size, input),
        |n| n == &end_index,
    )
    .expect("Og path");

    for (index, n) in og_path.iter().rev().enumerate() {
        lengths[*n] = Some(index);
    }

    og_path
        .par_iter()
        .enumerate()
        .take(og_path.len() - save)
        .map(|(index, start)| {
            let start_x = start % actual_grid_size;
            let start_y = start / actual_grid_size;

            // find walls
            let mut walls = [None; 4];

            if start_x > 0 {
                let next = start - 1;
                if input[next] == b'#' {
                    walls[0] = Some(next);
                }
            }
            if start_x < grid_size - 1 {
                let next = start + 1;
                if input[next] == b'#' {
                    walls[1] = Some(next);
                }
            }

            if start_y > 0 {
                let next = start - actual_grid_size;
                if input[next] == b'#' {
                    walls[2] = Some(next);
                }
            }
            if start_y < grid_size - 1 {
                let next = start + actual_grid_size;
                if input[next] == b'#' {
                    walls[3] = Some(next);
                }
            }

            // Find empties (that aren't us)
            let mut empties = vec![];

            walls.into_iter().flatten().for_each(|wall| {
                let wall_x = wall % actual_grid_size;
                let wall_y = wall / actual_grid_size;

                if wall_x > 0 {
                    let next = wall - 1;
                    if input[next] != b'#' && next != *start && !empties.contains(&next) {
                        empties.push(next);
                    }
                }
                if wall_x < grid_size - 1 {
                    let next = wall + 1;
                    if input[next] != b'#' && next != *start && !empties.contains(&next) {
                        empties.push(next);
                    }
                }

                if wall_y > 0 {
                    let next = wall - actual_grid_size;
                    if input[next] != b'#' && next != *start && !empties.contains(&next) {
                        empties.push(next);
                    }
                }
                if wall_y < grid_size - 1 {
                    let next = wall + actual_grid_size;
                    if input[next] != b'#' && next != *start && !empties.contains(&next) {
                        empties.push(next);
                    }
                }
            });

            let mut paths = 0;

            // Check if any is short enough
            for empty in empties {
                if let Some(len) = lengths[empty] {
                    // let len = res.unwrap().len();

                    if index + 1 + len < og_path.len() - save {
                        paths += 1;
                    }
                }
            }

            paths
        })
        .sum()
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_1() {
//         let input = include_bytes!("../../input/part1_example.txt");
//         let grid_size = 15;
//         let

//         let answer = super::process(input, grid_size);

//         assert_eq!(answer, 6);
//     }
// }
