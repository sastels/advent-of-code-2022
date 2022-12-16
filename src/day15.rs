use crate::utils::{l1_distance, Pos};
use itertools::Itertools;
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

pub fn solve_b(data: &[String]) -> usize {
    data.len()
}
