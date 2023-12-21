use pathfinding::prelude::dijkstra_all;
use std::collections::HashMap;

use rust21::{Node, Vec2};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input, 26501365);

    println!("Answer {answer}");
}

fn process(input: &str, count: usize) -> usize {
    let grid_width_usize = input.lines().next().unwrap().len();
    let grid_height_usize = input.lines().count();
    let grid_width = grid_width_usize as isize;
    let grid_height = grid_height_usize as isize;

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

    let rem = count % (grid_width_usize);
    let max = rem + ((grid_width_usize) * 2);
    let reachable = dijkstra_all(&starting_node, |n| {
        if n.dist >= max {
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

                if let Some(ch) = grid.get(&new_pos.map_to(grid_width, grid_height)) {
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

    let mut nums = [0, 0, 0];
    nums.iter_mut().enumerate().for_each(|(index, num)| {
        *num = reachable
            .values()
            .filter(|(_, cost)| *cost == (index * grid_width_usize + rem))
            .count()
    });

    let growth = (nums[2] - nums[1]) - (nums[1] - nums[0]);

    let mut steps = max;
    let mut score = nums[2];
    let mut increment = nums[2] - nums[1];

    while steps != count {
        increment += growth;
        score += increment;
        steps += grid_width_usize;
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;

    //     #[test]
    //     fn d20_part_2_is_correct_01() {
    //         let input = "...........
    // .....###.#.
    // .###.##..#.
    // ..#.#...#..
    // ....#.#....
    // .##..S####.
    // .##..#...#.
    // .......##..
    // .##.#.####.
    // .##..##.##.
    // ...........";

    //         assert_eq!(process(input, 6), 16);
    //     }

    #[test]
    fn d20_part_2_is_correct_02() {
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

        assert_eq!(process(input, 5000), 16733044);
    }
}
