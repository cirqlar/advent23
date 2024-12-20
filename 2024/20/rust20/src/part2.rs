use crate::{successors, NEWLINE_OFFSET};
use pathfinding::prelude::bfs;
use rustc_hash::FxHashSet;

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

    let mut reached_20 = FxHashSet::default();
    reached_20.insert(0);

    let isize_grid_size: isize = actual_grid_size.try_into().unwrap();
    let start_positions = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut positions = vec![((0, -1), 1), ((-1, 0), 1), ((0, 1), 1), ((1, 0), 1)];

    let mut current_start = 0;
    let mut current_end = positions.len();
    for i in 2..=20 {
        for ind in current_start..current_end {
            start_positions.iter().for_each(|pos| {
                let news = (positions[ind].0 .0 + pos.0, positions[ind].0 .1 + pos.1);

                let full_news = news.1 * isize_grid_size + news.0;

                if !reached_20.contains(&full_news) {
                    positions.push((news, i));
                    reached_20.insert(full_news);
                }
            });
        }

        current_start = current_end;
        current_end = positions.len();
    }

    og_path
        .par_iter()
        .take(og_path.len() - save)
        .map(|start| {
            positions
                .iter()
                .skip(4)
                .filter(|((distx, disty), cost)| {
                    let start_x: isize = (start % actual_grid_size).try_into().unwrap();
                    let start_y: isize = (start / actual_grid_size).try_into().unwrap();

                    if (distx.is_negative() && start_x < distx.abs())
                        || (disty.is_negative() && start_y < disty.abs())
                    {
                        return false;
                    }

                    let news_x: usize = (start_x + distx).try_into().unwrap();
                    let news_y: usize = (start_y + disty).try_into().unwrap();

                    if news_x >= grid_size || news_y >= grid_size {
                        return false;
                    }

                    let full_news = news_y * actual_grid_size + news_x;

                    if let Some(len) = lengths[full_news] {
                        let start_len = lengths[*start].unwrap();

                        if start_len <= len + cost {
                            false
                        } else {
                            start_len - len - cost >= save
                        }
                    } else {
                        false
                    }
                })
                .count()
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
