use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;
use petgraph::{
    algo::dijkstra,
    prelude::GraphMap,
    visit::{depth_first_search, Control, DfsEvent},
    Undirected,
};
use regex::Regex;

type PipeGraph<'a> = GraphMap<Valve<'a>, u32, Undirected>;

fn parse_graph(input: &str) -> PipeGraph {
    let mut graph = PipeGraph::new();
    let regex =
        Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)")
            .unwrap();
    let mut map = HashMap::new();
    for captures in regex.captures_iter(input) {
        let rate = captures[2].trim().parse::<u32>().unwrap();
        let n = Valve {
            name: captures.get(1).unwrap().as_str(),
            rate,
        };
        map.insert(n.name, n.rate);
        graph.add_node(n);
    }
    for captures in regex.captures_iter(input) {
        let name = captures.get(1).unwrap().as_str();
        let l = Valve {
            name,
            rate: map[name],
        };
        captures.get(3).unwrap().as_str().split(",").for_each(|v| {
            let r = Valve {
                name: v.trim(),
                rate: map[v.trim()],
            };
            graph.add_edge(l, r, 1);
        });
    }

    graph
}

#[derive(Debug, Clone, Copy)]
struct Valve<'a> {
    name: &'a str,
    rate: u32,
}

impl PartialEq for Valve<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Valve<'_> {}

impl Hash for Valve<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialOrd for Valve<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Valve<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(other.name)
    }
}

pub fn part1(input: &str) -> u32 {
    let graph = parse_graph(input);
    let useful = graph.nodes().filter(|n| n.rate > 0).collect::<HashSet<_>>();
    let mut condensed_graph = PipeGraph::new();
    for node in useful.iter() {
        condensed_graph.add_node(*node);
        let path = dijkstra(&graph, *node, None, |_| 1);
        for path in path {
            if useful.contains(&path.0) {
                condensed_graph.add_node(path.0);
                condensed_graph.add_edge(*node, path.0, path.1);
            }
        }
    }
    let path = dijkstra(
        &graph,
        Valve {
            name: "AA",
            rate: 0,
        },
        None,
        |_| 1,
    );
    let start_paths = path
        .into_iter()
        .filter(|(n, _)| useful.contains(n))
        .collect::<HashMap<_, _>>();
    let mut max = 0;
    for path in start_paths {
        let mut travel_time = path.1;
        let start = path.0;
        let mut stack = vec![start];
        let _res = depth_first_search(&condensed_graph, Some(start), |event| match event {
            DfsEvent::Discover(next, _) => {
                let time = condensed_graph
                    .edge_weight(*stack.last().unwrap(), next)
                    .unwrap();
                dbg!(time);
                dbg!(max);
                let res = stack
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| (a, b, condensed_graph.edge_weight(*a, *b).copied().unwrap()))
                    .fold((travel_time, 0), |(weight, sum), (_l, r, dist)| {
                        if 30 - weight < dist {
                            (weight + dist, 0)
                        } else {
                            (
                                weight + dist,
                                (dbg![30 - weight] - dbg![dist]) * dbg![r].rate + sum,
                            )
                        }
                    });
                max = max.max(res.1 + start.rate * (30 - path.1));

                if travel_time + time > 30 {
                    dbg![&res];
                    Control::<()>::Prune
                } else {
                    travel_time += time;
                    stack.push(next);
                    dbg![&stack];
                    Control::Continue
                }
            }
            DfsEvent::Finish(next, _) => {
                let time = condensed_graph
                    .edge_weight(*stack.last().unwrap(), next)
                    .unwrap();
                travel_time -= time;
                stack.pop();
                Control::Continue
            }
            _ => Control::Continue,
        });
    }
    max
}
