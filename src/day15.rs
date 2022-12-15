use regex::Regex;
use roaring::RoaringBitmap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SensorInfo {
    beacon: (u32, u32),
    sensor: (u32, u32),
    distance: u32,
}

pub fn part1(input: &str, y: u32) -> u32 {
    let sensors = parse_input(input);
    search_row(&sensors, y).0
}

fn search_row(sensors: &[SensorInfo], y: u32) -> (u32, RoaringBitmap) {
    let mut filled = RoaringBitmap::new();
    for sensor in sensors {
        if sensor.sensor == (20, 1) {
            println!("{:?}", sensor);
        }
        let diff = if sensor.sensor.1 > y {
            sensor
                .sensor
                .1
                .checked_sub(sensor.distance)
                .and_then(|n| y.checked_sub(n))
        } else if sensor.sensor.1 == y {
            Some(sensor.distance)
        } else {
            (sensor.sensor.1 + sensor.distance).checked_sub(y)
        };
        let offset = u32::MAX / 2;
        if let Some(diff) = diff {
            let start = (sensor.sensor.0 + offset - diff) as u32;
            let end = (sensor.sensor.0 + offset + diff) as u32;
            filled.insert_range(start..end);
        }
    }
    (filled.len() as _, filled)
}

fn parse_input(input: &str) -> Vec<SensorInfo> {
    let regex =
        Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(\d+), y=(\d+)").unwrap();
    regex
        .captures_iter(input)
        .map(|cap| {
            let sensor = (
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
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

pub fn part2(input: &str, max: u32) -> u32 {
    let sensors = parse_input(input);
    for y in 0..=max {
        let offset = u32::MAX / 2;
        let (_len, mut map) = search_row(&sensors, y);
        if !map.contains_range(offset..offset + max) {
            let _dbg = map
                .iter()
                .map(|n| n as i64 - offset as i64)
                .collect::<Vec<_>>();
            println!("{:?}", _dbg);

            map.remove_range(..offset);
            map.remove_range(offset + max..);
            let mut mask = RoaringBitmap::new();
            mask.insert_range(offset..offset + max);
            let union = map ^ mask;
            let idx = union.iter().next().unwrap();
            dbg!(y, idx - offset);
            return y + (idx - offset) * 4_000_000;
        }
    }
    unreachable!()
}
