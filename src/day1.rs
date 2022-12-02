use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;

pub fn part1(s: &str) -> u32 {
    s.lines()
        .map(|s| s.parse::<u32>().ok())
        .group_by(Option::is_some)
        .into_iter()
        .step_by(2)
        .map(|(_, g)| g.flatten().sum::<u32>())
        .max()
        .unwrap() as _
}

pub fn part2(s: &str) -> u32 {
    s.lines()
        .map(|s| s.parse::<u32>().ok())
        .group_by(Option::is_some)
        .into_iter()
        .step_by(2)
        .map(|(_, g)| g.flatten().sum::<u32>())
        .fold(
            {
                let mut heap = BinaryHeap::new();
                heap.extend([Reverse(0); 3]);
                heap
            },
            |mut h, i| {
                h.push(Reverse(i));
                h.pop();
                h
            },
        )
        .into_iter()
        .map(|Reverse(i)| i)
        .sum()
}
