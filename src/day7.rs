use itertools::Itertools;
use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction, Graph};
type Dir<'a> = Graph<(&'a str, u32), ()>;
pub fn part1(input: &str) -> u32 {
    let (graph, head) = build_dir(input);
    let mut cache = Vec::new();
    sum(&graph, &mut cache, head);
    cache.into_iter().filter(|n| n <= &100000).sum()
}

pub fn part2(input: &str) -> u32 {
    let (graph, head) = build_dir(input);
    let mut cache = Vec::new();
    let total = sum(&graph, &mut cache, head);
    let empty = 70_000_000 - total;
    let target = 30_000_000 - empty;
    cache.into_iter().filter(|n| n >= &target).min().unwrap()
}

fn build_dir(input: &str) -> (Dir<'_>, NodeIndex) {
    let mut graph = Dir::new();
    let mut node = graph.add_node(("/", 0));
    let head = node;
    for line in input.lines().skip(1) {
        match line.trim() {
            "$ ls" => {}
            "$ cd .." => {
                node = graph
                    .edges_directed(node, Direction::Incoming)
                    .next()
                    .unwrap()
                    .source();
            }
            s if s.starts_with("dir") => {
                let name = s.split_whitespace().nth(1).unwrap();
                let child = graph.add_node((name, 0));
                graph.add_edge(node, child, ());
            }
            s if s.starts_with("$ cd") => {
                let name = s.split_whitespace().nth(2).unwrap();
                node = graph
                    .edges_directed(node, Direction::Outgoing)
                    .find(|e| graph.node_weight(e.target()).unwrap().0 == name)
                    .unwrap()
                    .target();
            }
            s => {
                let (size, name) = s.split_whitespace().collect_tuple().unwrap();
                let size = size.parse().unwrap();
                let child = graph.add_node((name, size));
                graph.add_edge(node, child, ());
            }
        }
    }
    (graph, head)
}

fn sum(map: &Dir<'_>, cache: &mut Vec<u32>, idx: NodeIndex) -> u32 {
    let node = map.node_weight(idx).unwrap();
    if node.1 > 0 {
        return node.1;
    }
    let edges = map.edges(idx);
    edges
        .map(|e| {
            let s = sum(map, cache, e.target());
            cache.push(s);
            s
        })
        .sum()
}
