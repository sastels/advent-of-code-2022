use std::collections::HashSet;

pub fn parse_line(data: &str) -> (i32, i32, i32) {
    let v = data
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (v[0], v[1], v[2])
}

pub fn parse_input(data: &[String]) -> Vec<(i32, i32, i32)> {
    data.iter().map(|s| parse_line(s)).collect()
}

pub fn solve_a(data: &[String]) -> usize {
    let cubes = parse_input(data);
    let cube_clone = cubes.clone();
    let cube_set = cube_clone.iter().collect::<HashSet<_>>();

    let mut surface_area = 0;
    for cube in cubes {
        surface_area += 6;

        for offset in (-1..=1).step_by(2) {
            if cube_set.contains(&(cube.0 + offset, cube.1, cube.2)) {
                surface_area -= 1;
            }
            if cube_set.contains(&(cube.0, cube.1 + offset, cube.2)) {
                surface_area -= 1;
            }
            if cube_set.contains(&(cube.0, cube.1, cube.2 + offset)) {
                surface_area -= 1;
            }
        }
    }
    surface_area
}

pub fn solve_b(data: &[String]) -> usize {
    data.len()
}
