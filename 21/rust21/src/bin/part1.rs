use pathfinding::prelude::dijkstra_all;

use rust21::{Node, Vec2};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input, 64);

    println!("Answer {answer}");
}

fn process(input: &str, count: usize) -> usize {
    let grid_width = input.lines().next().unwrap().len() as isize;
    let grid_height = input.lines().count() as isize;

    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (Vec2::new(x as isize, y as isize), ch))
        })
        .collect::<HashMap<_, _>>();

    let starting_position = *grid.iter().find(|(_, v)| *v == &'S').unwrap().0;
    let starting_node = Node {
        pos: starting_position,
        dist: 0,
    };

    let reachable = dijkstra_all(&starting_node, |n| {
        if n.dist >= count {
            return vec![];
        }

        let new_dirs = [
            Vec2::new(1, 0),
            Vec2::new(0, -1),
            Vec2::new(-1, 0),
            Vec2::new(0, 1),
        ];

        new_dirs
            .into_iter()
            .filter_map(|new_dir| {
                let new_pos = n.pos + new_dir;

                if !new_pos.valid(grid_width, grid_height) {
                    return None;
                }

                if let Some(ch) = grid.get(&new_pos) {
                    if *ch == '.' || *ch == 'S' {
                        return Some((
                            Node {
                                pos: new_pos,
                                dist: n.dist + 1,
                            },
                            1,
                        ));
                    }
                }

                None
            })
            .collect::<Vec<_>>()
    });

    reachable
        .values()
        .filter(|(_, cost)| *cost == count)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d20_part_1_is_correct_01() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(process(input, 3), 6);
    }

    #[test]
    fn d20_part_1_is_correct_02() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(process(input, 6), 16);
    }
}
