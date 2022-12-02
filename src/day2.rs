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
            let me = match (left, right) {
                ('A', 'X') | ('C', 'Y') | ('B', 'Z') => 3,
                ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2,
                ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
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
