use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use itertools::{izip, Itertools};
use petgraph::{algo, prelude::NodeIndex, EdgeType, Graph};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2 {
    pub y: isize,
    pub x: isize,
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn valid(&self, bounds: &Vec2) -> bool {
        (0..bounds.x).contains(&self.x) && (0..bounds.y).contains(&self.y)
    }

    pub fn map_to(&self, x_max: isize, y_max: isize) -> Vec2 {
        Vec2 {
            x: self.x.rem_euclid(x_max),
            y: self.y.rem_euclid(y_max),
        }
    }

    pub fn to_one_dimensional(&self, width: usize) -> usize {
        (self.y as usize) * width + (self.x as usize)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a> Add for &'a Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from(value: (isize, isize)) -> Self {
        Vec2::new(value.0, value.1)
    }
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
pub struct Path {
    pub end: Vec2,
    pub count: usize,
}

#[derive(Debug)]
pub struct InOut {
    pub pos: Vec2,
    pub r#in: Vec<usize>,
    pub out: Vec<usize>,
}

#[derive(Debug)]
pub enum GraphNode {
    Path(Path),
    InOut(InOut),
}

pub fn explore<'a>(grid: &'a [char], from_pos: &'a Vec2, bounds: &'a Vec2) -> Vec<GraphNode> {
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
                    let ch = grid[next_pos.to_one_dimensional(bounds.y as usize)];
                    if ch == '#' {
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
                    .map(|pos| grid[pos.to_one_dimensional(bounds.y as usize)])
                    .collect_vec();

                let mut inout = InOut {
                    pos: current_pos,
                    r#in: vec![index],
                    out: vec![],
                };

                for (ch, pos, dir) in izip!(ch, next_pos, valid_dirs) {
                    if is_to(&ch, dir) {
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

pub fn fill_petgraph<'a, T: EdgeType>(
    graph: &'a [GraphNode],
    p_graph: &mut Graph<&'a GraphNode, i32, T>,
) -> HashMap<usize, NodeIndex> {
    let mut node_map = graph
        .iter()
        .enumerate()
        .filter_map(|(index, gn)| {
            if let GraphNode::Path(_) = gn {
                Some((index, p_graph.add_node(gn)))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    graph.iter().enumerate().for_each(|(index, gn)| {
        if let GraphNode::InOut(inout) = gn {
            let inout_index = p_graph.add_node(gn);
            node_map.insert(index, inout_index);

            for a in inout.r#in.iter() {
                let a_index = node_map.get(a).unwrap();
                p_graph.add_edge(*a_index, inout_index, 1);
            }

            for b in inout.out.iter() {
                let b_index = node_map.get(b).unwrap();
                p_graph.add_edge(inout_index, *b_index, 1);
            }
        }
    });

    node_map
}

pub fn get_max_ways<T: EdgeType>(
    p_graph: &Graph<&GraphNode, i32, T>,
    from: NodeIndex,
    to: NodeIndex,
    graph: &[GraphNode],
    node_map: &HashMap<usize, NodeIndex>,
) -> usize {
    let ways = algo::all_simple_paths::<Vec<_>, _>(p_graph, from, to, 0, None).collect_vec();

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
