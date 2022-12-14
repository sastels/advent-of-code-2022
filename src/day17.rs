use std::cmp::max;

const CAVE_WIDTH: usize = 7;

pub fn new_rock(rock_index: usize, bottom: i32) -> Vec<(i32, i32)> {
    match rock_index {
        0 => vec![(2, bottom), (3, bottom), (4, bottom), (5, bottom)],
        1 => vec![
            (3, bottom),
            (2, bottom + 1),
            (3, bottom + 1),
            (4, bottom + 1),
            (3, bottom + 2),
        ],
        2 => vec![
            (2, bottom),
            (3, bottom),
            (4, bottom),
            (4, bottom + 1),
            (4, bottom + 2),
        ],
        3 => vec![
            (2, bottom),
            (2, bottom + 1),
            (2, bottom + 2),
            (2, bottom + 3),
        ],
        4 => vec![(2, bottom), (3, bottom), (2, bottom + 1), (3, bottom + 1)],
        _ => panic!("Invalid rock_index: {}", rock_index),
    }
}

pub fn move_rock_left(rock: &[(i32, i32)], rock_pile: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut new_rock = Vec::new();
    for p in rock {
        if p.0 == 0 || rock_pile[p.1 as usize][p.0 as usize - 1] != '.' {
            return rock.to_vec();
        }
        new_rock.push((p.0 - 1, p.1));
    }
    new_rock
}

pub fn move_rock_right(rock: &[(i32, i32)], rock_pile: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut new_rock = Vec::new();
    for p in rock {
        if p.0 == CAVE_WIDTH as i32 - 1 || rock_pile[p.1 as usize][p.0 as usize + 1] != '.' {
            return rock.to_vec();
        }
        new_rock.push((p.0 + 1, p.1));
    }
    new_rock
}

pub fn move_rock_down(rock: &[(i32, i32)], rock_pile: &[Vec<char>]) -> (Vec<(i32, i32)>, bool) {
    let mut new_rock = Vec::new();
    for p in rock {
        if p.1 == 0 || rock_pile[p.1 as usize - 1][p.0 as usize] != '.' {
            return (rock.to_vec(), false);
        }
        new_rock.push((p.0, p.1 - 1));
    }
    (new_rock, true)
}

pub fn add_rock_to_pile(rock: &[(i32, i32)], rock_pile: &mut Vec<Vec<char>>) {
    let rock_height = rock.iter().map(|p| p.1).max().unwrap();
    let pile_height = rock_pile.len() as i32;
    for _ in 0..(rock_height - pile_height) {
        rock_pile.push(vec!['.'; CAVE_WIDTH]);
    }
    for p in rock {
        rock_pile[p.1 as usize][p.0 as usize] = '#';
    }
}
pub fn display_rocks(rock_pile: &[Vec<char>]) {
    for row in rock_pile.iter().rev() {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

pub fn solve_a(data: &[String], num_rocks: usize) -> usize {
    let jets = &data[0];
    let mut rock_pile = vec![vec!['-'; CAVE_WIDTH]];
    let mut jet_index = 0;
    let mut height_of_pile = 0;

    for rock_index in 0..num_rocks {
        let rock_starting_height = height_of_pile + 4;
        for _ in rock_pile.len()..(rock_starting_height + 5) {
            rock_pile.push(vec!['.'; CAVE_WIDTH]);
        }
        let mut rock = new_rock(rock_index % 5, rock_starting_height as i32);

        loop {
            if jets.chars().nth(jet_index) == Some('<') {
                rock = move_rock_left(&rock, &rock_pile);
            } else {
                rock = move_rock_right(&rock, &rock_pile);
            }
            jet_index = (jet_index + 1) % jets.len();
            let (new_rock, moved) = move_rock_down(&rock, &rock_pile);
            if moved {
                rock = new_rock.clone();
            } else {
                add_rock_to_pile(&rock, &mut rock_pile);
                height_of_pile = max(
                    height_of_pile,
                    rock.iter().map(|p| p.1).max().unwrap() as usize,
                );
                break;
            }
        }
        // display_rocks(&rock_pile);
    }
    height_of_pile
}

pub fn find_cycle(data: &[String], num_rocks: usize, diff_sum_cycle_start: u64) -> usize {
    let jets = &data[0];
    let mut rock_pile = vec![vec!['-'; CAVE_WIDTH]];
    let mut jet_index = 0;
    let mut height_of_pile = 0;
    let mut old_height_of_pile = 0;
    let mut diffs = vec![];
    let mut all_diffs = vec![];
    let mut in_prefix = true;
    let mut in_cycle = false;
    let mut prefix = vec![];
    let mut cycle = vec![];

    for rock_index in 0..num_rocks {
        let rock_starting_height = height_of_pile + 4;
        for _ in rock_pile.len()..(rock_starting_height + 5) {
            rock_pile.push(vec!['.'; CAVE_WIDTH]);
        }
        let mut rock = new_rock(rock_index % 5, rock_starting_height as i32);

        loop {
            if jets.chars().nth(jet_index) == Some('<') {
                rock = move_rock_left(&rock, &rock_pile);
            } else {
                rock = move_rock_right(&rock, &rock_pile);
            }
            jet_index = (jet_index + 1) % jets.len();
            let (new_rock, moved) = move_rock_down(&rock, &rock_pile);
            if moved {
                rock = new_rock.clone();
            } else {
                add_rock_to_pile(&rock, &mut rock_pile);

                height_of_pile = max(
                    height_of_pile,
                    rock.iter().map(|p| p.1).max().unwrap() as usize,
                );
                break;
            }
        }
        let diff: u64 = height_of_pile as u64 - old_height_of_pile as u64;
        diffs.push(diff);
        all_diffs.push(diff);
        old_height_of_pile = height_of_pile;

        if diff == 4 {
            let diff_sum = diffs.iter().sum::<u64>();
            println!("{}: diff_sum {}", rock_index, diff_sum);
            if in_prefix {
                prefix.extend(diffs);
            } else if in_cycle {
                cycle.extend(diffs);
            }
            if diff_sum == diff_sum_cycle_start && in_prefix {
                in_prefix = false;
                in_cycle = true;
            } else if diff_sum == diff_sum_cycle_start && in_cycle {
                in_cycle = false;
            }
            diffs = vec![];
        }
    }
    println!("prefix: {:?}", prefix);
    println!("cycle: {:?}", cycle);

    height_of_pile
}

pub fn solve_b(num_rocks: u64, prefix: &[u64], cycle: &[u64]) -> u64 {
    let prefix_sum = prefix.iter().sum::<u64>();
    let cycle_sum = cycle.iter().sum::<u64>();
    let num_cycles = (num_rocks - prefix.len() as u64) / cycle.len() as u64;
    let num_remaining = (num_rocks - prefix.len() as u64) % cycle.len() as u64;
    let remaining_sum = cycle[0..num_remaining as usize].iter().sum::<u64>();
    prefix_sum + num_cycles * cycle_sum + remaining_sum
}
