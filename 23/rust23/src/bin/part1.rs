use pathfinding::prelude::dijkstra_all;
use std::collections::{BTreeSet, HashMap};

use rust23::Vec2;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Vec2,
    visited: BTreeSet<Vec2>,
}

fn process(input: &str) -> usize {
    let grid_width = input.lines().next().unwrap().len();
    let grid_height = input.lines().count();
    let grid_bounds = Vec2::new(grid_width as isize, grid_height as isize);
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (Vec2::new(x as isize, y as isize), ch))
        })
        .collect::<HashMap<_, _>>();

    let starting_x = input.lines().next().unwrap().find('.').unwrap();
    let ending_x = input.lines().next_back().unwrap().find('.').unwrap();

    let start_pos = Vec2::new(starting_x as isize, 0);
    let start_node = Node {
        pos: start_pos,
        visited: BTreeSet::new(),
    };
    let end_pos = Vec2::new(ending_x as isize, (grid_height - 1) as isize);

    let all_dirs = [
        Vec2::new(1, 0),
        Vec2::new(0, -1),
        Vec2::new(-1, 0),
        Vec2::new(0, 1),
    ];
    let reached = dijkstra_all(&start_node, |n| {
        if n.pos == end_pos {
            return vec![];
        }

        let mut n_visited = n.visited.clone();
        n_visited.insert(n.pos);

        let mut dirs = &all_dirs[..];

        let ch = grid.get(&n.pos).unwrap();
        match *ch {
            '^' => dirs = &all_dirs[1..2],
            '>' => dirs = &all_dirs[0..1],
            '<' => dirs = &all_dirs[2..3],
            'v' => dirs = &all_dirs[3..4],
            _ => {}
        }

        dirs.iter()
            .filter_map(|dir| {
                let next_pos = n.pos + *dir;
                if !next_pos.valid(&grid_bounds) {
                    return None;
                }
                if n_visited.contains(&next_pos) {
                    return None;
                }

                let ch = grid.get(&next_pos).unwrap();

                if (*dir == all_dirs[0] && *ch == '<')
                    || (*dir == all_dirs[1] && *ch == 'v')
                    || (*dir == all_dirs[2] && *ch == '>')
                    || (*dir == all_dirs[3] && *ch == '^')
                    || *ch == '#'
                {
                    return None;
                }

                Some((
                    Node {
                        pos: next_pos,
                        visited: n_visited.clone(),
                    },
                    1,
                ))
            })
            .collect()
    });

    *reached
        .iter()
        .filter_map(
            |(n, (_, cost))| {
                if n.pos == end_pos {
                    Some(cost)
                } else {
                    None
                }
            },
        )
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d23_part_1_is_correct() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        assert_eq!(process(input), 94);
    }
}
