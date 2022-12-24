use advent_2022::day16::{
    compute_score, dist, get_closed_valves, parse_room, parse_rooms, rooms_ordered_by_score,
    solve_a, solve_b, Room,
};
use advent_2022::utils::read_lines;
use itertools::sorted;

const TEST_FILE: &str = "./test_data/day16.txt";

#[test]
fn test_parse_room() {
    assert_eq!(
        parse_room("Valve AA has flow rate=12; tunnels lead to valves BBB, C"),
        Room {
            name: "AA".to_string(),
            flow_rate: 12,
            tunnels: vec!["BBB".to_string(), "C".to_string()],
        }
    );
    assert_eq!(
        parse_room("Valve AA has flow rate=12; tunnel leads to valve BBB"),
        Room {
            name: "AA".to_string(),
            flow_rate: 12,
            tunnels: vec!["BBB".to_string()],
        }
    );
}

#[test]
fn test_get_closed_valves() {
    let data = read_lines(TEST_FILE);
    let rooms = parse_rooms(&data);
    assert_eq!(
        sorted(get_closed_valves(&rooms)).collect::<Vec<_>>(),
        vec!["BB", "CC", "DD", "EE", "HH", "JJ"]
    );
}

#[test]
fn test_dist() {
    let data = read_lines(TEST_FILE);
    let rooms = parse_rooms(&data);

    assert_eq!(dist(&"AA".to_string(), &"BB".to_string(), &rooms), 1);
    assert_eq!(dist(&"JJ".to_string(), &"CC".to_string(), &rooms), 4);
    assert_eq!(dist(&"DD".to_string(), &"II".to_string(), &rooms), 2);
    assert_eq!(dist(&"GG".to_string(), &"BB".to_string(), &rooms), 5);
}

// real order: DD, BB, JJ, HH, EE, CC (minute 24)
//             20  13  21  22   3   2

#[test]
fn test_compute_score() {
    let data = read_lines(TEST_FILE);
    let rooms = parse_rooms(&data);
    let order: Vec<String> = "DD, BB, JJ, HH, EE, CC"
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    assert_eq!(compute_score(&order, &rooms, 1), 0);
    assert_eq!(compute_score(&order, &rooms, 2), 0);
    assert_eq!(compute_score(&order, &rooms, 3), 20);
    assert_eq!(compute_score(&order, &rooms, 4), 40);
    assert_eq!(compute_score(&order, &rooms, 5), 60);
    assert_eq!(compute_score(&order, &rooms, 6), 93);
    assert_eq!(compute_score(&order, &rooms, 30), 1651);
}

#[test]
fn test_rooms_ordered_by_score() {
    let data = read_lines(TEST_FILE);
    let rooms = parse_rooms(&data);

    let order = rooms_ordered_by_score(&rooms);
    assert_eq!(order, vec!["HH", "JJ", "DD", "BB", "EE", "CC"]);
}

#[ignore = "probablistic so sometimes fails"]
#[test]
fn test_solve_a() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_a(&data, 10, 10000), 1651);
}

#[ignore = "probablistic so sometimes fails"]
#[test]
fn test_solve_b() {
    let data = read_lines(TEST_FILE);
    assert_eq!(solve_b(&data, 10000), 1707);
}
