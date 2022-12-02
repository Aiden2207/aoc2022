pub fn part1(s: &str) -> u32 {
    s.lines()
        .map(|s| {
            let mut iter = s.chars();
            let left = iter.next().unwrap();
            let right = iter.last().unwrap();
            let me = match right {
                'X' => 0,
                'Y' => 1,
                'Z' => 2,
                _ => unreachable!(),
            };
            let opp = match left {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => unreachable!(),
            };
            let res = match (me, opp) {
                (1, 1) | (2, 2) | (3, 3) => 3,
                (1, 2) | (2, 3) | (3, 1) => 0,
                (1, 3) | (2, 1) | (3, 2) => 6,
                _ => unreachable!(),
            };
            res + me
        })
        .sum()
}

pub fn part2(s: &str) -> u32 {
    s.lines()
        .map(|s| {
            let mut iter = s.chars();
            let left = iter.next().unwrap();
            let right = iter.last().unwrap();
            let opp = match left {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => unreachable!(),
            };
            let me = match (opp, right) {
                (0, 'X') | (2, 'Y') | (1, 'Z') => 3,
                (0, 'Z') | (1, 'Y') | (2, 'X') => 2,
                (0, 'Y') | (1, 'X') | (2, 'Z') => 1,
                _ => unreachable!(),
            };
            let res = match right {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => unreachable!(),
            };
            me + res
        })
        .sum()
}
