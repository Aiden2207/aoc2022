use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use petgraph::{Graph, Undirected};
type PathGraph = Graph<u32, (), Undirected>;

pub fn parse_graph(input: &str) -> (PathGraph, NodeIndex, NodeIndex) {
    let mut graph = PathGraph::new_undirected();
    let mut nodes = vec![];
    let mut start = None;
    let mut end = None;
    for (i, line) in input.lines().enumerate() {
        if nodes.get(i).is_none() {
            nodes.push(vec![]);
        }
        for c in line.trim().chars() {
            match c {
                'S' => {
                    start = Some(graph.add_node(1));
                    nodes[i].push(start.unwrap());
                }
                'E' => {
                    end = Some(graph.add_node(26));
                    nodes[i].push(end.unwrap());
                }
                c => {
                    let node = graph.add_node(c as u32 - 'a' as u32 + 1);
                    nodes[i].push(node);
                }
            }
        }
    }
    for i in 0..nodes.len() - 1 {
        for j in 0..nodes[i].len() - 1 {
            graph.update_edge(nodes[i][j], nodes[i + 1][j], ());
            graph.update_edge(nodes[i][j], nodes[i][j + 1], ());
        }
    }
    for i in 0..nodes.len() - 1 {
        graph.update_edge(
            nodes[i][nodes[i].len() - 1],
            nodes[i + 1][nodes[i + 1].len() - 1],
            (),
        );
    }
    let bound = nodes.len() - 1;
    for i in 0..nodes[bound].len() - 1 {
        graph.update_edge(nodes[bound][i], nodes[bound][i + 1], ());
    }
    // dbg!(&graph);

    (graph, start.unwrap(), end.unwrap())
}
pub fn part1(input: &str) -> u32 {
    let (graph, start, end) = parse_graph(input);
    let cost = |edge: <&PathGraph as IntoEdgeReferences>::EdgeRef| {
        let origin = edge.source();
        let dest = edge.target();
        let origin_node = graph.node_weight(origin).unwrap();
        let dest_node = graph.node_weight(dest).unwrap();
        if *dest_node <= *origin_node + 1 {
            1
        } else {
            u32::MAX >> 1
        }
    };
    let results = astar(
        &graph,
        start,
        |n| n == end,
        cost,
        |_node| if _node == end { 0 } else { 1 },
    );
    results.unwrap().0
}

pub fn part2(input: &str) -> u32 {
    let (graph, _, end) = parse_graph(input);
    let cost = |edge: <&PathGraph as IntoEdgeReferences>::EdgeRef| {
        let origin = edge.source();
        let dest = edge.target();
        let origin_node = graph.node_weight(origin).unwrap();
        let dest_node = graph.node_weight(dest).unwrap();
        if *dest_node <= *origin_node + 1 {
            1
        } else {
            u32::MAX >> 4
        }
    };
    graph
        .node_indices()
        .filter(|node| graph.node_weight(*node).unwrap() == &1)
        .filter_map(|start| astar(&graph, start, |n| n == end, cost, |_node| 0))
        .map(|(cost, _)| cost)
        .min()
        .unwrap()
}
