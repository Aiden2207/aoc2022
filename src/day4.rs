use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (l, r) = s
                .split(',')
                .map(|r| {
                    let (start, end) = r
                        .split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .next_tuple()
                        .unwrap();
                    start..=end
                })
                .next_tuple()
                .unwrap();
            (l.contains(r.start()) && l.contains(r.end()))
                || (r.contains(l.end()) && r.contains(l.start()))
        })
        .filter(|b| *b)
        .count() as _
}
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (l, r) = s
                .split(',')
                .map(|r| {
                    let (start, end) = r
                        .split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .next_tuple()
                        .unwrap();
                    start..=end
                })
                .next_tuple()
                .unwrap();
            l.contains(r.start())
                || l.contains(r.end())
                || r.contains(l.end())
                || r.contains(l.start())
        })
        .filter(|b| *b)
        .count() as _
}
