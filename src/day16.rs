use pathfinding::prelude::bfs;
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Room {
    pub name: String,
    pub flow_rate: i32,
    pub tunnels: Vec<String>,
}

pub fn parse_room(line: &str) -> Room {
    let rx = Regex::new(r"Valve (.+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)")
        .unwrap();
    let captures = rx.captures(line).unwrap();

    let name = captures.get(1).unwrap().as_str().to_string();
    let flow_rate = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let tunnels = captures
        .get(3)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    Room {
        name,
        flow_rate,
        tunnels,
    }
}

pub fn parse_rooms(data: &[String]) -> HashMap<String, Room> {
    data.iter()
        .map(|line| parse_room(line))
        .map(|room| (room.name.clone(), room))
        .collect()
}

pub fn get_closed_valves(rooms: &HashMap<String, Room>) -> Vec<String> {
    rooms
        .iter()
        .filter_map(|(_, room)| {
            if room.flow_rate != 0 {
                Some(room.name.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn neighbours(room: &String, rooms: &HashMap<String, Room>) -> Vec<String> {
    rooms.get(room).unwrap().tunnels.clone()
}

fn path(start: &String, end: &String, rooms: &HashMap<String, Room>) -> Vec<String> {
    bfs(start, |p| neighbours(p, rooms), |p| *p == *end).unwrap()
}
pub fn dist(start: &String, end: &String, rooms: &HashMap<String, Room>) -> i32 {
    path(start, end, rooms).len() as i32 - 1
}

// 100K took 5 seconds
pub fn compute_score(order: &[String], rooms: &HashMap<String, Room>, max_time: i32) -> i32 {
    let mut score = 0;
    let mut current_room = "AA".to_string();
    let mut flow_rate = 0;
    let mut time = 0;
    for next_room in order.iter() {
        let d = dist(&current_room, next_room, rooms);
        if time + d + 1 >= max_time {
            break;
        }
        let time_to_next_open_valve = d + 1;
        score += time_to_next_open_valve * flow_rate;
        time += time_to_next_open_valve;
        current_room = next_room.clone();
        flow_rate += rooms.get(&current_room).unwrap().flow_rate;
    }
    score += flow_rate * (max_time - time);
    score
}

pub fn rooms_ordered_by_score(rooms: &HashMap<String, Room>) -> Vec<String> {
    let mut room_names = rooms
        .keys()
        .cloned()
        .filter(|r| rooms.get(r).unwrap().flow_rate > 0)
        .collect::<Vec<_>>();

    room_names.sort_by(|a, b| {
        (rooms.get(a).unwrap().flow_rate)
            .cmp(&rooms.get(b).unwrap().flow_rate)
            .reverse()
    });
    room_names
}

pub fn solve_a(data: &[String], num_valves: usize, bound: usize) -> i32 {
    let rooms = parse_rooms(data);
    let mut rooms_ordered = rooms_ordered_by_score(&rooms);
    rooms_ordered.truncate(num_valves);

    let mut best_score = 0;
    let mut best_order = vec![];

    for i in 0..bound {
        let score = compute_score(&rooms_ordered, &rooms, 30);
        if score > best_score {
            best_score = score;
            best_order = rooms_ordered.clone();
            println!(
                "{}: best score: {} for {}",
                i,
                best_score,
                best_order.join(", ")
            );
        }
        rooms_ordered.shuffle(&mut rand::thread_rng());
    }
    println!("Best score: {} for {}", best_score, best_order.join(", "));
    best_score
}

pub fn solve_b(data: &[String], bound: usize) -> i32 {
    let rooms = parse_rooms(data);
    let mut rooms_ordered = rooms_ordered_by_score(&rooms);
    let mut best_score = 0;
    let mut best_me = vec![];
    let mut best_elephant = vec![];

    let half_rooms_ordered = (rooms_ordered.len() as f64 * 0.5).ceil() as usize;
    for i in 0..bound {
        let num_me = half_rooms_ordered + rand::thread_rng().gen_range(0..2);
        let mut me = vec![];
        let mut elephant = vec![];

        for (i, room) in rooms_ordered.iter().enumerate() {
            if i < num_me {
                me.push(room.clone());
            } else {
                elephant.push(room.clone());
            }
        }
        let score = compute_score(&me, &rooms, 26) + compute_score(&elephant, &rooms, 26);
        if score > best_score {
            best_score = score;
            best_me = me.clone();
            best_elephant = elephant.clone();
            println!(
                "{}: best score: {} for me: {} elephant: {}",
                i,
                best_score,
                best_me.join(", "),
                best_elephant.join(", ")
            );
        }
        rooms_ordered.shuffle(&mut rand::thread_rng());
    }
    println!(
        "Best score: {} for me: {} elephant: {}",
        best_score,
        best_me.join(", "),
        best_elephant.join(", ")
    );
    best_score
}
