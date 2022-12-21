use pathfinding::prelude::bfs;
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

pub fn solve_a(data: &[String]) -> usize {
    let rooms = parse_rooms(data);
    let mut closed_valves = get_closed_valves(&rooms);
    let mut open_valves = vec![];

    let mut total_flow = 0;
    let mut flow_rate = 0;
    let mut current_room = "AA".to_string();
    let mut dist_to_next_room = 0;
    let mut path_to_next_room = vec![];
    let mut next_room = "".to_string();

    let mut time = 1;

    while time <= 30 {
        total_flow += flow_rate;
        println!("\n== Minute {} : {} ==", time, total_flow);
        time += 1;

        if open_valves.is_empty() {
            println!("No valves are open.");
        } else {
            println!(
                "Valves {} are open, releasing {} pressure.",
                open_valves.join(", "),
                flow_rate
            );
        }
        if closed_valves.is_empty() {
            continue;
        }

        if dist_to_next_room == 0 && !next_room.is_empty() {
            println!("You open valve {}.", next_room);
            flow_rate += rooms.get(&next_room).unwrap().flow_rate;
            closed_valves.retain(|r| r != &next_room);
            open_valves.push(next_room.clone());
            current_room = next_room.clone();
            next_room = "".to_string();
            continue;
        }

        if dist_to_next_room > 0 {
            println!("You move to valve {}.", path_to_next_room[0]);
            path_to_next_room = path_to_next_room[1..].to_vec();
            dist_to_next_room -= 1;
            continue;
        }

        // we need to find the next room to go to

        let distances = closed_valves
            .iter()
            .map(|r2| (r2.clone(), dist(&current_room, r2, &rooms)))
            .collect::<HashMap<_, _>>();

        next_room = distances
            .iter()
            .map(|(r2, d)| (r2, rooms.get(r2).unwrap().flow_rate - d))
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap()
            .clone();

        dist_to_next_room = *distances.get(&next_room).unwrap();
        path_to_next_room = path(&current_room, &next_room, &rooms);
        path_to_next_room = path_to_next_room[1..].to_vec(); // already at the first position

        println!("You move to valve {}.", path_to_next_room[0]);
        path_to_next_room = path_to_next_room[1..].to_vec();
        dist_to_next_room -= 1;
    }

    total_flow as usize
}

pub fn solve_b(data: &[String]) -> usize {
    data.len()
}
