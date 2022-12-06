use std::collections::HashSet;

pub fn solve(window_size: usize, data: &str) -> usize {
    (0..(data.len() - window_size))
        .map(|i| (i + window_size, &data[i..i + window_size]))
        .filter(|(_i, s)| s.chars().collect::<HashSet<_>>().len() == window_size)
        .map(|(i, _s)| i)
        .next()
        .unwrap()
}

pub fn solve_a(data: &str) -> usize {
    solve(4, data)
}

pub fn solve_b(data: &str) -> usize {
    solve(14, data)
}
