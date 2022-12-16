pub fn parse_line(line: &str) -> Vec<(usize, usize)> {
    line.split(" -> ")
        .map(|s| {
            let s01 = s.split_once(',').unwrap();
            (
                s01.0.parse::<usize>().unwrap(),
                s01.1.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

pub fn add_rock_to_cave(cave: &mut [Vec<char>], rock: &[(usize, usize)]) {
    let mut start_point = rock[0];
    for end_point in rock.iter().skip(1) {
        let (x1, y1) = start_point;
        let (x2, y2) = *end_point;
        if x1 == x2 {
            let (ymin, ymax) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            (ymin..=ymax).for_each(|y| cave[x1][y] = '#');
        } else {
            let (xmin, xmax) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            (xmin..=xmax).for_each(|x| cave[x][y1] = '#');
        }
        start_point = *end_point;
    }
}

pub fn drop_sand(cave: &mut [Vec<char>], x: usize, y: usize) -> bool {
    if y > 500 {
        false
    } else if cave[x][y + 1] == '.' {
        drop_sand(cave, x, y + 1)
    } else if cave[x - 1][y + 1] == '.' {
        drop_sand(cave, x - 1, y + 1)
    } else if cave[x + 1][y + 1] == '.' {
        drop_sand(cave, x + 1, y + 1)
    } else {
        cave[x][y] = 'o';
        true
    }
}

pub fn make_cave(rocks: &[Vec<(usize, usize)>]) -> Vec<Vec<char>> {
    let mut cave = vec![vec!['.'; 1000]; 1000];
    rocks
        .iter()
        .for_each(|rock| add_rock_to_cave(&mut cave, rock));
    cave
}

pub fn solve_a(data: &[String]) -> usize {
    let rocks = data.iter().map(|line| parse_line(line)).collect::<Vec<_>>();
    let mut cave = make_cave(&rocks);
    let mut sand_counter = 0;
    while drop_sand(&mut cave, 500, 0) {
        sand_counter += 1;
    }
    sand_counter
}

pub fn solve_b(data: &[String]) -> usize {
    let rocks = data.iter().map(|line| parse_line(line)).collect::<Vec<_>>();
    let mut cave = make_cave(&rocks);

    let lowest_rock = rocks
        .iter()
        .map(|rock| rock.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap();
    (0..1000).for_each(|x| cave[x][lowest_rock + 2] = '#');

    let mut sand_counter = 0;
    while cave[500][0] != 'o' && drop_sand(&mut cave, 500, 0) {
        sand_counter += 1;
    }
    sand_counter
}
