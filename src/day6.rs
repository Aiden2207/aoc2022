use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .char_indices()
        .tuple_windows()
        .find(|((_, a), (_, b), (_, c), (_, d))| {
            [a, b, c, d].into_iter().collect::<HashSet<_>>().len() == 4
        })
        .unwrap()
        .3
         .0
        + 1
}

pub fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, arr)| arr.into_iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        .0
        + 13
}
