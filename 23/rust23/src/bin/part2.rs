use itertools::{izip, Itertools};
use petgraph::{algo, graph::UnGraph};
use std::collections::{HashMap, HashSet};

use rust23::Vec2;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn is_to(ch: &char, dir: &Vec2) -> bool {
    matches!(
        (*ch, *dir),
        ('^', Vec2 { x: 0, y: -1 })
            | ('>', Vec2 { x: 1, y: 0 })
            | ('v', Vec2 { x: 0, y: 1 })
            | ('<', Vec2 { x: -1, y: 0 })
    )
}

#[derive(Debug)]
struct Path {
    end: Vec2,
    count: usize,
}

#[derive(Debug)]
struct InOut {
    pos: Vec2,
    r#in: Vec<usize>,
    out: Vec<usize>,
}

#[derive(Debug)]
enum GraphNode {
    Path(Path),
    InOut(InOut),
}

fn explore<'a>(
    grid: &'a HashMap<Vec2, char>,
    from_pos: &'a Vec2,
    bounds: &'a Vec2,
) -> Vec<GraphNode> {
    let mut seen = HashSet::new();
    seen.insert(*from_pos);
    let all_dirs = [
        Vec2::new(1, 0),
        Vec2::new(0, -1),
        Vec2::new(-1, 0),
        Vec2::new(0, 1),
    ];

    let mut graph = vec![GraphNode::Path(Path {
        end: *from_pos,
        count: 0,
    })];
    let mut index = 0;

    while index < graph.len() {
        let GraphNode::Path(ref curr) = graph[index] else {
            index += 1;
            continue;
        };
        let mut current_pos = curr.end;
        loop {
            seen.insert(current_pos);
            let valid_dirs = all_dirs
                .iter()
                .filter(|dir| {
                    let next_pos = current_pos + **dir;
                    if !next_pos.valid(bounds) {
                        return false;
                    }
                    if seen.contains(&next_pos) {
                        if let Some(GraphNode::InOut(x)) = graph.iter_mut().find(|n| {
                            if let GraphNode::InOut(inout) = n {
                                inout.pos == next_pos
                                    && !inout.r#in.contains(&index)
                                    && !inout.out.contains(&index)
                            } else {
                                false
                            }
                        }) {
                            x.r#in.push(index);
                        }

                        return false;
                    }
                    let ch = grid.get(&next_pos).unwrap();
                    if *ch == '#' {
                        return false;
                    }
                    true
                })
                .collect::<Vec<_>>();

            if valid_dirs.is_empty() {
                if let GraphNode::Path(ref mut curr) = graph[index] {
                    curr.end = current_pos;
                    curr.count += 1;
                };
                break;
            } else if valid_dirs.len() == 1 {
                if let GraphNode::Path(ref mut curr) = graph[index] {
                    curr.end = current_pos;
                    curr.count += 1;
                };
                current_pos = current_pos + *valid_dirs[0];
            } else if valid_dirs.len() >= 2 {
                let next_pos = valid_dirs
                    .iter()
                    .map(|dir| current_pos + **dir)
                    .collect_vec();
                let ch = next_pos
                    .iter()
                    .map(|pos| grid.get(pos).unwrap())
                    .collect_vec();

                let mut inout = InOut {
                    pos: current_pos,
                    r#in: vec![index],
                    out: vec![],
                };

                for (ch, pos, dir) in izip!(ch, next_pos, valid_dirs) {
                    if is_to(ch, dir) {
                        let path = Path {
                            // start: pos,
                            end: pos,
                            count: 0,
                        };
                        inout.out.push(graph.len());
                        graph.push(GraphNode::Path(path));
                    }
                }

                graph.push(GraphNode::InOut(inout));

                break;
            }
        }
        index += 1;
    }

    graph
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
    let end_pos = Vec2::new(ending_x as isize, (grid_height - 1) as isize);

    let graph = explore(&grid, &start_pos, &grid_bounds);

    let mut new_graph = UnGraph::new_undirected();
    let mut node_map = graph
        .iter()
        .enumerate()
        .filter_map(|(index, gn)| {
            if let GraphNode::Path(_) = gn {
                Some((index, new_graph.add_node(gn)))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    graph.iter().enumerate().for_each(|(index, gn)| {
        if let GraphNode::InOut(inout) = gn {
            let inout_index = new_graph.add_node(gn);
            node_map.insert(index, inout_index);

            for a in inout.r#in.iter() {
                let a_index = node_map.get(a).unwrap();
                new_graph.add_edge(*a_index, inout_index, 1);
            }

            for b in inout.out.iter() {
                let b_index = node_map.get(b).unwrap();
                new_graph.add_edge(inout_index, *b_index, 1);
            }
        }
    });

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

    let ways = algo::all_simple_paths::<Vec<_>, _>(
        &new_graph,
        *node_map.get(&0).unwrap(),
        *node_map.get(&end_index).unwrap(),
        0,
        None,
    )
    .collect_vec();

    let sizes = ways
        .iter()
        .map(|v| {
            v.iter()
                .map(|ind| {
                    let p_ind = node_map
                        .iter()
                        .find_map(|(k, val)| if val == ind { Some(*k) } else { None })
                        .unwrap();
                    if let GraphNode::Path(p) = &graph[p_ind] {
                        p.count
                    } else {
                        1
                    }
                })
                .sum::<usize>()
        })
        .collect_vec();

    *sizes.iter().max().unwrap() - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d23_part_2_is_correct() {
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

        assert_eq!(process(input), 154);
    }
}
