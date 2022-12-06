use regex::Regex;

pub fn part1(input: &str) -> String {
    let moves = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let crates = Regex::new(r"\[(.)\]").unwrap();
    let mut stacks = vec![];
    let mut iter = input.lines();
    iter.by_ref()
        .take_while(|line| !line.trim().is_empty())
        .for_each(|line| {
            crates.find_iter(line).for_each(|cap| {
                let label = cap.as_str().to_owned();
                let pos = cap.end() / 3;
                while stacks.len() <= pos {
                    stacks.push(vec![]);
                }
                stacks[pos].push(label);
            });
        });
    let mut stacks = stacks
        .into_iter()
        .filter(|stack| !stack.is_empty())
        .map(|mut stack| {
            stack.reverse();
            stack
        })
        .collect::<Vec<_>>();
    for line in iter {
        let cap = moves.captures(line).unwrap();
        let n = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;
        let len = stacks[from].len();
        let mut from = stacks[from].drain(len - n..).collect::<Vec<_>>();
        from.reverse();
        stacks[to].append(&mut from);
    }
    stacks
        .into_iter()
        .map(|mut s| s.pop().unwrap()[1..=1].to_owned())
        .collect()
}

pub fn part2(input: &str) -> String {
    let moves = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let crates = Regex::new(r"\[(.)\]").unwrap();
    let mut stacks = vec![];
    let mut iter = input.lines();
    iter.by_ref()
        .take_while(|line| !line.trim().is_empty())
        .for_each(|line| {
            crates.find_iter(line).for_each(|cap| {
                let label = cap.as_str().to_owned();
                let pos = cap.end() / 3;
                while stacks.len() <= pos {
                    stacks.push(vec![]);
                }
                stacks[pos].push(label);
            });
        });
    let mut stacks = stacks
        .into_iter()
        .filter(|stack| !stack.is_empty())
        .map(|mut stack| {
            stack.reverse();
            stack
        })
        .collect::<Vec<_>>();
    for line in iter {
        let cap = moves.captures(line).unwrap();
        let n = cap[1].parse::<usize>().unwrap();
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;
        let len = stacks[from].len();
        let mut from = stacks[from].drain(len - n..).collect::<Vec<_>>();
        stacks[to].append(&mut from);
    }
    stacks
        .into_iter()
        .map(|mut s| s.pop().unwrap()[1..=1].to_owned())
        .collect()
}
