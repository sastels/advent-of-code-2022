use std::collections::HashSet;

pub fn split_rucksack(s: &str) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

pub fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

pub fn solve_a(data: &[String]) -> u32 {
    let rucksacks = data.iter().map(|s| split_rucksack(s)).collect::<Vec<_>>();
    let mut total = 0;
    for r in rucksacks.iter() {
        let (left, right) = *r;
        let both = left.chars().find(|e| right.contains(*e)).unwrap();
        total += priority(both);
    }
    total
}

pub fn intersect_sets(sets: &[HashSet<char>]) -> HashSet<char> {
    let mut iter = sets.iter();
    let mut result = iter.next().unwrap().clone();
    for s in iter {
        result = result.intersection(s).cloned().collect();
    }
    result
}

pub fn solve_b(data: &[String]) -> u32 {
    let mut total = 0;
    for elves in data.chunks(3) {
        let sets: Vec<HashSet<char>> = vec![
            elves[0].chars().collect(),
            elves[1].chars().collect(),
            elves[2].chars().collect(),
        ];
        let badge = *intersect_sets(&sets).iter().next().unwrap();
        total += priority(badge)
    }
    total
}
