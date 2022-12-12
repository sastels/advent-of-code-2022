use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub usize, pub usize);

// note that the map is transposed: x goes down, y goes right

pub fn find_start_end(data: &[String]) -> (Pos, Pos) {
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Pos(i, j);
            } else if c == 'E' {
                end = Pos(i, j);
            }
        }
    }
    (start, end)
}

pub fn parse_input(data: &[String]) -> Vec<Vec<usize>> {
    let mut map = Vec::new();
    for line in data {
        let mut row = Vec::new();
        for c in line.chars() {
            if c == 'S' {
                row.push(0);
            } else if c == 'E' {
                row.push(25);
            } else {
                row.push(c as usize - 'a' as usize);
            }
        }
        map.push(row);
    }
    map
}

pub fn neighbours(p: &Pos, map: &Vec<Vec<usize>>) -> Vec<Pos> {
    let max_y = (map[0].len() - 1) as i32;
    let max_x = (map.len() - 1) as i32;
    let height_p = map[p.0][p.1];

    let (x, y) = (p.0 as i32, p.1 as i32);
    vec![(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
        .iter()
        .filter(|&q| q.0 >= 0 && q.0 <= max_x && q.1 >= 0 && q.1 <= max_y)
        .map(|q| Pos(q.0 as usize, q.1 as usize))
        .filter(|q| map[q.0][q.1] <= height_p + 1)
        .collect()
}

pub fn find_zero_height_points(map: &[Vec<usize>]) -> Vec<Pos> {
    let mut result = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if *height == 0 {
                result.push(Pos(i, j));
            }
        }
    }
    result
}
pub fn solve_a(data: &[String]) -> usize {
    let (start, end) = find_start_end(data);
    let map = parse_input(data);
    let result = bfs(&start, |p| neighbours(p, &map), |p| *p == end).unwrap();
    result.len() - 1
}

pub fn solve_b(data: &[String]) -> usize {
    let (_, end) = find_start_end(data);
    let map = parse_input(data);
    let zero_height_points = find_zero_height_points(&map);
    zero_height_points
        .iter()
        .map(|p| {
            bfs(p, |p| neighbours(p, &map), |p| *p == end)
                .unwrap_or_else(|| vec![Pos(0, 0); 100000])
                .len()
                - 1
        })
        .min()
        .unwrap()
}
