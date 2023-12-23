use itertools::Itertools;

use petgraph::graph::DiGraph;
use std::collections::BTreeSet;

use rust23::{explore, fill_petgraph, get_max_ways, GraphNode, Vec2};

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
    let grid = input.lines().flat_map(|line| line.chars()).collect_vec();

    let starting_x = input.lines().next().unwrap().find('.').unwrap();
    let ending_x = input.lines().next_back().unwrap().find('.').unwrap();

    let start_pos = Vec2::new(starting_x as isize, 0);
    let end_pos = Vec2::new(ending_x as isize, (grid_height - 1) as isize);

    let graph = explore(&grid, &start_pos, &grid_bounds);
    let end_index = graph
        .iter()
        .position(|gn| {
            if let GraphNode::Path(path) = gn {
                path.end == end_pos
            } else {
                false
            }
        })
        .unwrap();

    let mut new_graph = DiGraph::new();
    let node_map = fill_petgraph(&graph, &mut new_graph);

    get_max_ways(
        &new_graph,
        *node_map.get(&0).unwrap(),
        *node_map.get(&end_index).unwrap(),
        &graph,
        &node_map,
    )
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
