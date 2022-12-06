pub fn elves_sections(s: &str) -> Vec<u32> {
    s.replace('-', ",")
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn elves_redundant(elves: &[u32]) -> bool {
    (elves[0] >= elves[2] && elves[1] <= elves[3]) || (elves[2] >= elves[0] && elves[3] <= elves[1])
}

pub fn elves_overlap(elves: &[u32]) -> bool {
    (elves[0] >= elves[2] && elves[0] <= elves[3])
        || (elves[1] >= elves[2] && elves[1] <= elves[3])
        || elves_redundant(elves)
}

pub fn solve_a(data: &[String]) -> u32 {
    data.iter()
        .map(|s| elves_redundant(&elves_sections(s)))
        .filter(|b| *b)
        .count() as u32
}

pub fn solve_b(data: &[String]) -> u32 {
    data.iter()
        .map(|s| elves_overlap(&elves_sections(s)))
        .filter(|b| *b)
        .count() as u32
}
