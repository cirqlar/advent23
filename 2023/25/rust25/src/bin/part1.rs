use std::collections::HashMap;

use slotmap::{new_key_type, SlotMap};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

new_key_type! {
    struct VIndex;
}

#[derive(Debug, Clone)]
struct Graph<'a> {
    verts: SlotMap<VIndex, (&'a str, u32)>,
    edges: Vec<(VIndex, VIndex)>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph {
            verts: SlotMap::with_key(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, entry: &'a str) -> VIndex {
        self.verts.insert((entry, 1))
    }

    fn add_edge(&mut self, a: VIndex, b: VIndex) {
        self.edges.push((a, b));
    }

    fn num_verts(&self) -> usize {
        self.verts.len()
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn combine_edge(&mut self, edge_index: usize) {
        let (a, b) = self.edges[edge_index];

        for i in (0..self.edges.len()).rev() {
            if self.edges[i].0 == b {
                if self.edges[i].1 == a {
                    self.edges.remove(i);
                } else {
                    self.edges[i].0 = a;
                }
            } else if self.edges[i].1 == b {
                if self.edges[i].0 == a {
                    self.edges.remove(i);
                } else {
                    self.edges[i].1 = a;
                }
            }
        }

        let (_, b) = self.verts.remove(b).unwrap();
        let a = self.verts.get_mut(a).unwrap();
        a.1 += b;
    }

    fn get_count_mutliplied(&self) -> u32 {
        self.verts.values().map(|(_, count)| count).product()
    }
}

fn process(input: &str) -> u32 {
    let mut node_map = HashMap::new();
    let mut graph = Graph::new();

    for line in input.lines() {
        let (first, rest) = line.split_once(": ").unwrap();

        let first_key;
        if node_map.contains_key(first) {
            first_key = *node_map.get(first).unwrap();
        } else {
            first_key = graph.add_node(first);
            node_map.insert(first, first_key);
        }

        for key in rest.split(' ') {
            let second_key;
            if node_map.contains_key(key) {
                second_key = *node_map.get(key).unwrap();
            } else {
                second_key = graph.add_node(key);
                node_map.insert(key, second_key);
            }

            graph.add_edge(first_key, second_key);
        }
    }

    let mut rand = fastrand::Rng::new();

    let mut min_edges = 10000000000000;
    let mut product = 0;

    // Algorithm isn't always correct so do it a bunch of times to make sure we get the right answer
    for _ in 0..1000 {
        let mut test_graph = graph.clone();

        // Using Karger's Algorithm. Source: https://www.scaler.com/topics/data-structures/kargers-algorithm-for-minimum-cut/
        while test_graph.num_verts() > 2 {
            let edge_index = rand.usize(..test_graph.num_edges());

            test_graph.combine_edge(edge_index);
        }
        if min_edges > test_graph.num_edges() {
            min_edges = test_graph.num_edges();
            product = test_graph.get_count_mutliplied();
        }
    }

    product
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d25_part_1_is_correct() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

        assert_eq!(process(input), 54);
    }
}
