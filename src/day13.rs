use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;
use nom::{
    branch::alt, character::complete::char, character::complete::digit0, combinator::map_res,
    multi::separated_list0, sequence::delimited, IResult,
};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
            (Packet::Num(n), Packet::List(l)) => vec![Packet::Num(*n)].cmp(l),
            (Packet::List(l), Packet::Num(n)) => l.cmp(&vec![Packet::Num(*n)]),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let packets = parse_packets(input);
    packets
        .into_iter()
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let two = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    let six = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);
    let mut packets = parse_packets(input)
        .into_iter()
        .flat_map(|(l, r)| [l, r])
        .chain([two.clone(), six.clone()])
        .collect_vec();
    packets.sort();
    let (first, second) = packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| packet == &two || packet == &six)
        .collect_tuple()
        .unwrap();
    (first.0 + 1) * (second.0 + 1)
}

fn parse_packets(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\r\n\r\n")
        .map(|s| {
            let mut it = s.lines();
            let p1 = it.next().unwrap();
            let p2 = it.next().unwrap();
            (parse_packet(p1).unwrap().1, parse_packet(p2).unwrap().1)
        })
        .collect()
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (s, vec) = delimited(
        char('['),
        separated_list0(
            char(','),
            alt((
                map_res(digit0, |n: &str| -> Result<_, <u32 as FromStr>::Err> {
                    Ok(Packet::Num(n.parse()?))
                }),
                parse_packet,
            )),
        ),
        char(']'),
    )(input)?;
    Ok((s, Packet::List(vec)))
}
