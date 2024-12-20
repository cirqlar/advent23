use crate::NEWLINE_OFFSET;
use pathfinding::prelude::bfs;
use rustc_hash::FxHashSet;

fn successors(n: usize, grid_size: usize, input: &[u8]) -> Vec<usize> {
    let actual_grid_size = grid_size + NEWLINE_OFFSET;

    let nx = n % actual_grid_size;
    let ny = n / actual_grid_size;

    let mut nexts = Vec::with_capacity(4);

    if nx > 0 {
        let next = n - 1;
        if input[next] != b'#' {
            nexts.push(next);
        }
    }
    if nx < grid_size - 1 {
        let next = n + 1;
        if input[next] != b'#' {
            nexts.push(next);
        }
    }

    if ny > 0 {
        let next = n - actual_grid_size;
        if input[next] != b'#' {
            nexts.push(next);
        }
    }
    if ny < grid_size - 1 {
        let next = n + actual_grid_size;
        if input[next] != b'#' {
            nexts.push(next);
        }
    }

    nexts
}

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
        // .iter()
        .enumerate()
        .take(og_path.len() - save)
        .map(|(index, start)| {
            let mut x_away = vec![];
            let mut empties = vec![];

            let mut reached = FxHashSet::default();
            reached.insert(*start);

            let start_x = start % actual_grid_size;
            let start_y = start / actual_grid_size;

            if start_x > 0 {
                let next = start - 1;
                x_away.push(next);
                reached.insert(next);
            }
            if start_x < grid_size - 1 {
                let next = start + 1;
                x_away.push(next);
                reached.insert(next);
            }

            if start_y > 0 {
                let next = start - actual_grid_size;
                x_away.push(next);
                reached.insert(next);
            }
            if start_y < grid_size - 1 {
                let next = start + actual_grid_size;
                x_away.push(next);
                reached.insert(next);
            }

            for i in 2..=20 {
                x_away = x_away
                    .into_iter()
                    .flat_map(|n| {
                        let mut nexts = vec![];

                        let nx = n % actual_grid_size;
                        let ny = n / actual_grid_size;

                        if nx > 0 {
                            let next = n - 1;
                            if !reached.contains(&next) {
                                nexts.push(next);
                                if input[next] != b'#' {
                                    empties.push((next, i));
                                }
                                reached.insert(next);
                            }
                        }
                        if nx < grid_size - 1 {
                            let next = n + 1;
                            if !reached.contains(&next) {
                                nexts.push(next);
                                if input[next] != b'#' {
                                    empties.push((next, i));
                                }
                                reached.insert(next);
                            }
                        }

                        if ny > 0 {
                            let next = n - actual_grid_size;
                            if !reached.contains(&next) {
                                nexts.push(next);
                                if input[next] != b'#' {
                                    empties.push((next, i));
                                }
                                reached.insert(next);
                            }
                        }
                        if ny < grid_size - 1 {
                            let next = n + actual_grid_size;
                            if !reached.contains(&next) {
                                nexts.push(next);
                                if input[next] != b'#' {
                                    empties.push((next, i));
                                }
                                reached.insert(next);
                            }
                        }

                        nexts
                    })
                    .collect::<Vec<_>>();
            }

            let mut paths = 0;

            // Check if any is short enough
            for (empty, reached_after) in empties {
                if let Some(len) = lengths[empty] {
                    // let len = res.unwrap().len();

                    if index + reached_after + len < og_path.len() - save {
                        paths += 1;
                    }
                }
            }

            paths
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");
        let grid_size = 15;
        let save = 50;

        let answer = super::process(input, grid_size, save);

        assert_eq!(answer, 285);
    }
}
