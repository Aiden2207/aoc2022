use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);
    let mut visited = HashSet::new();
    visited.insert(tail);
    for line in input.lines() {
        let (dir, num) = line.split_whitespace().next_tuple().unwrap();
        let num = num.parse::<i32>().unwrap();
        match dir {
            "U" => {
                for _i in 0..num {
                    let new_head = (head.0, head.1 + 1);
                    if (tail.0 as f64 - new_head.0 as f64).hypot(tail.1 as f64 - new_head.1 as f64)
                        >= 1.9
                    {
                        tail = head;
                    }
                    head = new_head;
                    visited.insert(tail);
                }
            }
            "D" => {
                for _i in 0..num {
                    let new_head = (head.0, head.1 - 1);
                    if (tail.0 as f64 - new_head.0 as f64).hypot(tail.1 as f64 - new_head.1 as f64)
                        >= 1.9
                    {
                        tail = head;
                    }
                    head = new_head;
                    visited.insert(tail);
                }
            }
            "L" => {
                for _i in 0..num {
                    let new_head = (head.0 - 1, head.1);
                    if (tail.0 as f64 - new_head.0 as f64).hypot(tail.1 as f64 - new_head.1 as f64)
                        >= 1.9
                    {
                        tail = head;
                    }
                    head = new_head;
                    visited.insert(tail);
                }
            }
            "R" => {
                for _i in 0..num {
                    let new_head = (head.0 + 1, head.1);
                    if (tail.0 as f64 - new_head.0 as f64).hypot(tail.1 as f64 - new_head.1 as f64)
                        >= 1.9
                    {
                        tail = head;
                    }
                    head = new_head;
                    visited.insert(tail);
                }
            }
            _ => panic!("unknown direction"),
        }
    }
    visited.len()
}

pub fn part2(input: &str) -> usize {
    let mut rope = vec![(0i32, 0i32); 10];
    let mut visited = HashSet::new();
    visited.insert((0i32, 0i32));
    for line in input.lines() {
        let (dir, num) = line.split_whitespace().next_tuple().unwrap();
        let num = num.parse::<i32>().unwrap();
        let change;
        match dir {
            "U" => {
                change = (0, 1);
            }
            "D" => {
                change = (0, -1);
            }
            "L" => {
                change = (-1, 0);
            }
            "R" => {
                change = (1, 0);
            }
            _ => panic!("unknown direction"),
        }
        for _ in 0..num {
            rope[0].0 += change.0;
            rope[0].1 += change.1;
            for i in 1..rope.len() {
                let head = rope[i - 1];
                let tail = rope[i];
                if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                    rope[i].0 += (head.0 - tail.0).signum();
                    rope[i].1 += (head.1 - tail.1).signum();
                }
            }
            visited.insert(rope[9]);
        }
    }
    visited.len()
}
