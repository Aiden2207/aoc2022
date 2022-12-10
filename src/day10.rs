use itertools::Itertools;
pub fn part1(input: &str) -> i32 {
    let list = [20, 60, 100, 140, 180, 220];
    input
        .lines()
        .map(|m| match m.trim() {
            "noop" => None,
            add => add
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap()
                .into(),
        })
        .fold((1, 1, 0), |(cycle, reg, sum), op| match op {
            Some(n) => {
                if list.contains(&cycle) {
                    (cycle + 2, reg + n, sum + reg * cycle)
                } else if list.contains(&(cycle + 1)) {
                    dbg!(cycle + 1, reg);
                    (cycle + 2, reg + n, sum + reg * (cycle + 1))
                } else {
                    (cycle + 2, reg + n, sum)
                }
            }
            None => {
                if list.contains(&cycle) {
                    dbg!(cycle, reg);
                    (cycle + 1, reg, sum + reg * cycle)
                } else {
                    (cycle + 1, reg, sum)
                }
            }
        })
        .2
}

pub fn part2(input: &str) -> String {
    let mut grid = vec![b'.'; 6 * 40 + 2];
    let mut lines = input.lines().map(|m| match m.trim() {
        "noop" => None,
        add => add
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap()
            .into(),
    });
    let mut op = None;
    let mut mid_add = false;
    let mut reg = 1;
    let mut row = 0;
    for (cycle, item) in grid.iter_mut().enumerate().take(240 + 1).skip(1usize) {
        match (op, mid_add) {
            (Some(_), false) => {
                mid_add = true;
            }
            (Some(n), true) => {
                reg += n;
                mid_add = false;
                op = lines.next().unwrap();
            }
            (None, _) => {
                op = lines.next().unwrap();
            }
        }
        if (cycle - 1..=cycle + 1).contains(&((reg + row * 40 + 1) as usize)) {
            *item = b'#';
        }
        if cycle % 40 == 0 {
            row += 1;
        }
    }
    grid.remove(0);
    grid.pop();
    #[allow(unstable_name_collisions)]
    grid.chunks_exact(40)
        .intersperse(b"\n")
        .flatten()
        .map(|&c| c as char)
        .collect()
}
