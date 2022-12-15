use std::cmp::Ordering;

use regex::Regex;
use roaring::RoaringBitmap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SensorInfo {
    beacon: (i32, i32),
    sensor: (i32, i32),
    distance: u32,
}

pub fn part1(input: &str, y: i32) -> u32 {
    let sensors = parse_input(input);
    search_row(&sensors, y).0 - 1
}

fn search_row(sensors: &[SensorInfo], y: i32) -> (u32, RoaringBitmap) {
    let mut filled = RoaringBitmap::new();
    for sensor in sensors {
        let diff = match sensor.sensor.1.cmp(&y) {
            Ordering::Less => sensor.sensor.1 + sensor.distance as i32 - y,
            Ordering::Greater => y - (sensor.sensor.1 - sensor.distance as i32),
            Ordering::Equal => sensor.distance as _,
        } as i64;
        if diff >= 0 {
            let offset = (u32::MAX / 2) as i64;
            let start = (sensor.sensor.0 as i64 + offset - diff) as u32;
            let end = (sensor.sensor.0 as i64 + offset + diff) as u32;
            filled.insert_range(start..=end);
        }
    }
    (filled.len() as u32, filled)
}

fn parse_input(input: &str) -> Vec<SensorInfo> {
    let regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    regex
        .captures_iter(input)
        .map(|cap| {
            let sensor = (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            );
            let beacon = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
            let distance = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
            SensorInfo {
                sensor,
                beacon,
                distance,
            }
        })
        .collect()
}

pub fn part2(input: &str, max: u32) -> u64 {
    let sensors = parse_input(input);
    let mut mask = RoaringBitmap::new();
    let offset = u32::MAX / 2;

    mask.insert_range(offset..offset + max);

    for y in 0..=max {
        if y % 100000 == 0 {
            println!("y: {}", y);
        }
        let (_len, mut map) = search_row(&sensors, y as _);
        if !map.contains_range(offset..offset + max) {
            map.remove_range(..offset);
            map.remove_range(offset + max..);
            map ^= &mask;
            let idx = map.iter().next().unwrap();
            // dbg!(union.iter().count());
            return y as u64 + (idx - offset) as u64 * 4_000_000;
        }
    }
    unreachable!()
}
