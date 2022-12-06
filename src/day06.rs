use std::collections::HashSet;

pub fn solve(tuple_size: usize, data: &str) -> usize {
    (0..(data.len() - tuple_size))
        .map(|i| (i + tuple_size, &data[i..i + tuple_size]))
        .filter(|(_i, s)| s.chars().collect::<HashSet<_>>().len() == tuple_size)
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
