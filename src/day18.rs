use std::cmp::max;
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

fn find_surface_area(cubes: &[(i32, i32, i32)]) -> usize {
    let cube_clone = Vec::from(cubes);
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

pub fn find_interior(cubes: &[(i32, i32, i32)]) -> Vec<(i32, i32, i32)> {
    let cube_clone = Vec::from(cubes);
    let cube_set = cube_clone.iter().collect::<HashSet<_>>();
    let lava_size = cubes.iter().map(|c| max(c.0, max(c.1, c.2))).max().unwrap();
    let mut steam_done = HashSet::new();
    let mut steam_not_done = HashSet::new();
    steam_not_done.insert((-1, -1, -1));
    while !steam_not_done.is_empty() {
        let mut new_steam = HashSet::new();
        for cube in steam_not_done {
            for offset in (-1..=1).step_by(2) {
                let new_cube = (cube.0 + offset, cube.1, cube.2);
                if new_cube.0 >= -1
                    && new_cube.0 <= lava_size + 1
                    && !cube_set.contains(&new_cube)
                    && !steam_done.contains(&new_cube)
                {
                    new_steam.insert(new_cube);
                }
                let new_cube = (cube.0, cube.1 + offset, cube.2);
                if new_cube.1 >= -1
                    && new_cube.1 <= lava_size + 1
                    && !cube_set.contains(&new_cube)
                    && !steam_done.contains(&new_cube)
                {
                    new_steam.insert(new_cube);
                }
                let new_cube = (cube.0, cube.1, cube.2 + offset);
                if new_cube.2 >= -1
                    && new_cube.2 <= lava_size + 1
                    && !cube_set.contains(&new_cube)
                    && !steam_done.contains(&new_cube)
                {
                    new_steam.insert(new_cube);
                }
            }
            steam_done.insert(cube);
        }
        steam_not_done = new_steam;
    }
    let mut interior_cubes = vec![];
    for x in 0..=lava_size {
        for y in 0..=lava_size {
            for z in 0..=lava_size {
                if !steam_done.contains(&(x, y, z)) && !cube_set.contains(&(x, y, z)) {
                    interior_cubes.push((x, y, z));
                }
            }
        }
    }
    interior_cubes
}

pub fn solve_a(data: &[String]) -> usize {
    let cubes = parse_input(data);
    find_surface_area(&cubes)
}

pub fn solve_b(data: &[String]) -> usize {
    let mut cubes = parse_input(data);
    let interior = find_interior(&cubes);
    cubes.extend(interior);
    find_surface_area(&cubes)
}
