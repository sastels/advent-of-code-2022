use crate::intervals::merge_intervals;
use crate::utils::{l1_distance, Pos};
use regex::Regex;

#[derive(Clone)]
pub struct Sensor {
    pub pos: Pos,
    pub beacon: Pos,
    pub beacon_dist: u32,
}

pub fn parse_line(line: &str) -> Sensor {
    let rx = Regex::new(r"Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)").unwrap();
    let captures = rx.captures(line).unwrap();
    let pos = Pos(
        captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
    );
    let beacon = Pos(
        captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
    );
    let beacon_dist = l1_distance(&pos, &beacon);
    Sensor {
        pos,
        beacon,
        beacon_dist,
    }
}

pub fn solve_a(data: &[String], y0: i32) -> usize {
    let sensors = data
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<Sensor>>();
    let row_min = sensors
        .iter()
        .map(|sensor| sensor.pos.0 - sensor.beacon_dist as i32)
        .min()
        .unwrap()
        - 10;
    let row_max = sensors
        .iter()
        .map(|sensor| sensor.pos.0 + sensor.beacon_dist as i32)
        .max()
        .unwrap()
        + 10;

    let mut num_no_beacons = 0;
    for x in row_min..=row_max {
        let test_pos = Pos(x, y0);
        for sensor in &sensors {
            if test_pos == sensor.beacon {
                break;
            }
            if l1_distance(&sensor.pos, &test_pos) <= sensor.beacon_dist {
                num_no_beacons += 1;
                break;
            }
        }
    }
    num_no_beacons
}

pub fn solve_b(data: &[String], bound: i32) -> Option<i64> {
    let sensors = data
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<Sensor>>();

    for y in 0..=bound {
        let mut intervals: Vec<(i32, i32)> = sensors
            .iter()
            .filter_map(|sensor| {
                if (y - sensor.pos.1).unsigned_abs() <= sensor.beacon_dist {
                    let dist = sensor.beacon_dist - (y - sensor.pos.1).unsigned_abs();
                    let x_min = sensor.pos.0 - dist as i32;
                    let x_max = sensor.pos.0 + dist as i32;
                    Some((x_min, x_max))
                } else {
                    None
                }
            })
            .collect();
        intervals = merge_intervals(intervals);
        if intervals.len() > 1 {
            return Some((intervals[0].1 + 1) as i64 * 4000000 + y as i64);
        }
    }
    None
}
