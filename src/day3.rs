use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (left, right) = s.split_at(s.len() / 2);
            let left = left.chars().collect::<HashSet<_>>();
            let right = right.chars().collect::<HashSet<_>>();
            let misplaced = left.intersection(&right).next().unwrap();
            match misplaced {
                &c @ 'a'..='z' => c as u32 - 'a' as u32 + 1,
                &c @ 'A'..='Z' => c as u32 - 'A' as u32 + 1 + 26,
                _ => unreachable!(),
            }
        })
        .sum()
}
use itertools::Itertools;
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            let c = c.chars().collect::<HashSet<_>>();
            let misplaced = &(&a & &b) & &c;
            match misplaced.iter().next().unwrap() {
                &c @ 'a'..='z' => c as u32 - 'a' as u32 + 1,
                &c @ 'A'..='Z' => c as u32 - 'A' as u32 + 1 + 26,
                _ => unreachable!(),
            }
        })
        .sum()
}
