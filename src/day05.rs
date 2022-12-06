pub fn parse_stacks(data: &[String]) -> Vec<Vec<char>> {
    let num_stacks = (data[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    for line in data {
        if line.contains('1') {
            break;
        }
        for (n, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if chunk[1] != ' ' {
                stacks[n].push(chunk[1]);
            }
        }
    }
    stacks
        .iter()
        .map(|v| v.iter().rev().cloned().collect())
        .collect()
}

pub fn parse_move_line(line: &str) -> (usize, usize, usize) {
    let iter = line.split_whitespace().map(|s| s.to_string());

    let line_split: Vec<String> = iter.collect();
    let num = line_split[1].parse::<usize>().unwrap();
    let from = line_split[3].parse::<usize>().unwrap();
    let to = line_split[5].parse::<usize>().unwrap();
    (num, from, to)
}

pub fn move_crate(stacks: Vec<Vec<char>>, num: usize, from: usize, to: usize) -> Vec<Vec<char>> {
    let mut new_stacks = stacks;
    for _i in 0..num {
        let card = new_stacks[from - 1].pop().unwrap();
        new_stacks[to - 1].push(card);
    }
    new_stacks
}

pub fn move_crate_9001(
    stacks: Vec<Vec<char>>,
    num: usize,
    from: usize,
    to: usize,
) -> Vec<Vec<char>> {
    let mut new_stacks = stacks;
    new_stacks.push(vec![]);
    let extra_stack = new_stacks.len();
    new_stacks = move_crate(new_stacks, num, from, extra_stack);
    new_stacks = move_crate(new_stacks, num, extra_stack, to);
    new_stacks.pop();
    new_stacks
}

pub fn get_top_crates(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .filter(|v| !v.is_empty())
        .map(|v| v.last().unwrap())
        .cloned()
        .collect()
}

pub fn solve_a(data: &[String]) -> String {
    let mut stacks = parse_stacks(data);
    for line in data {
        if line.contains("move") {
            let (num, from, to) = parse_move_line(line);
            stacks = move_crate(stacks, num, from, to);
        }
    }
    get_top_crates(&stacks)
}

pub fn solve_b(data: &[String]) -> String {
    let mut stacks = parse_stacks(data);
    for line in data {
        if line.contains("move") {
            let (num, from, to) = parse_move_line(line);
            stacks = move_crate_9001(stacks, num, from, to);
        }
    }
    get_top_crates(&stacks)
}
