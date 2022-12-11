use regex::Regex;
use std::cmp::Reverse;
struct Monkey {
    num: usize,
    op: Box<dyn Fn(u128) -> u128>,
    items: Vec<u128>,
    test: u128,
    true_dest: usize,
    false_dest: usize,
}
fn parse_monkey(input: &str) -> Vec<Monkey> {
    //screw carriage returns
    let string = r"Monkey (?P<num>\d+):\r?
  Starting items: (?P<items>[\d, ]+)\r?
  Operation: new = old (?P<op>[\+\*]) (?P<rhs>[\dold]+)\r?
  Test: divisible by (?P<test>\d+)\r?
    If true: throw to monkey (?P<true_dest>\d+)\r?
    If false: throw to monkey (?P<false_dest>\d+)";
    let regex = Regex::new(string).unwrap();
    let caps = regex.captures_iter(input);
    let mut monkeys = Vec::new();
    for cap in caps {
        let num = cap["num"].parse().unwrap();
        let op = match (&cap["op"], &cap["rhs"]) {
            ("+", "old") => Box::new(|x: u128| x + x) as Box<dyn Fn(u128) -> u128>,
            ("*", "old") => Box::new(|x: u128| x * x),
            ("+", rhs) => {
                let rhs = rhs.parse::<u128>().unwrap();
                Box::new(move |x: u128| x + rhs)
            }
            ("*", rhs) => {
                let rhs = rhs.parse::<u128>().unwrap();
                Box::new(move |x: u128| x * rhs)
            }
            _ => panic!("Unknown op"),
        };
        let test = cap["test"].trim().parse().unwrap();
        let true_dest = cap["true_dest"].parse().unwrap();
        let false_dest = cap["false_dest"].parse().unwrap();
        let items = cap["items"]
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();
        monkeys.push(Monkey {
            num,
            op,
            test,
            items,
            true_dest,
            false_dest,
        });
    }
    monkeys
}

pub fn part1(input: &str) -> u128 {
    let mut monkeys = parse_monkey(input);
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let (ref mut true_items, ref mut false_items) = monkey
                .items
                .drain(..)
                .map(|n| {
                    inspections[i] += 1;
                    (monkey.op)(n)
                })
                .partition(|n| n % monkey.test == 0);
            let true_dest = monkey.true_dest;
            let false_dest = monkey.false_dest;
            monkeys[true_dest].items.append(true_items);
            monkeys[false_dest].items.append(false_items);
        }
    }
    monkeys.sort_by_key(|m| Reverse(inspections[m.num]));
    inspections[monkeys[0].num] as u128 * inspections[monkeys[1].num] as u128
}

pub fn part2(input: &str) -> u128 {
    let mut monkeys = parse_monkey(input);
    let mut inspections = vec![0; monkeys.len()];
    let lcm: u128 = monkeys.iter().map(|m| m.test).product();
    for _i in 0..10_000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let (ref mut true_items, ref mut false_items) = monkey
                .items
                .drain(..)
                .map(|n| {
                    inspections[i] += 1;
                    (monkey.op)(n) % lcm
                })
                .partition(|n| n % monkey.test == 0);
            let true_dest = monkey.true_dest;
            let false_dest = monkey.false_dest;
            monkeys[true_dest].items.append(true_items);
            monkeys[false_dest].items.append(false_items);
        }
    }
    monkeys.sort_by_key(|m| Reverse(inspections[m.num]));
    inspections[monkeys[0].num] as u128 * inspections[monkeys[1].num] as u128
}
