use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction, Graph};
type Dir<'a> = Graph<(&'a str, u32), ()>;
pub fn part1(input: &str) -> u32 {
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
    let mut cache = HashMap::new();
    fn sum(map: &Dir<'_>, cache: &mut HashMap<String, u32>, idx: NodeIndex, path: String) -> u32 {
        let node = map.node_weight(idx).unwrap();
        if node.1 > 0 {
            return node.1;
        }
        let edges = map.edges(idx);
        edges
            .map(|e| {
                let weight = map.node_weight(e.target()).unwrap();
                if let Some(s) = cache.get(&(path.clone() + weight.0)) {
                    return *s;
                } else {
                    let s = sum(map, cache, e.target(), path.clone() + weight.0);
                    if weight.1 == 0 {
                        cache.insert(path.clone() + weight.0, s);
                    }
                    return s;
                }
            })
            .sum()
    }
    sum(&graph, &mut cache, head, "/".to_owned());
    cache
        .into_iter()
        .filter(|(_, n)| n <= &100000)
        .map(|(_, n)| n)
        .sum()
}

pub fn part2(input: &str) -> u32 {
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
    let mut cache = HashMap::new();
    fn sum(map: &Dir<'_>, cache: &mut HashMap<String, u32>, idx: NodeIndex, path: String) -> u32 {
        let node = map.node_weight(idx).unwrap();
        if node.1 > 0 {
            return node.1;
        }
        let edges = map.edges(idx);
        edges
            .map(|e| {
                let weight = map.node_weight(e.target()).unwrap();
                if let Some(s) = cache.get(&(path.clone() + weight.0)) {
                    return *s;
                } else {
                    let s = sum(map, cache, e.target(), path.clone() + weight.0);
                    if weight.1 == 0 {
                        cache.insert(path.clone() + weight.0, s);
                    }
                    return s;
                }
            })
            .sum()
    }
    let total = sum(&graph, &mut cache, head, "/".to_owned());
    let empty = 70_000_000 - total;
    let target = 30_000_000 - empty;
    cache
        .into_iter()
        .filter(|(_, n)| n >= &target)
        .map(|(_, n)| n)
        .min()
        .unwrap()
}
