use std::collections::HashSet;

pub fn move_head(head: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        'U' => (head.0, head.1 + 1),
        'D' => (head.0, head.1 - 1),
        'R' => (head.0 + 1, head.1),
        'L' => (head.0 - 1, head.1),
        _ => panic!("Invalid direction"),
    }
}

pub fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head == tail
        || (head.0 == tail.0 && (head.1.abs_diff(tail.1) == 1))
        || (head.1 == tail.1 && (head.0.abs_diff(tail.0) == 1))
        || (head.0.abs_diff(tail.0) == 1 && head.1.abs_diff(tail.1) == 1)
    {
        // No movement
        tail
    } else if head.0 != tail.0 && head.1 != tail.1 {
        // Diagonal
        if head.0 > tail.0 {
            if head.1 > tail.1 {
                (tail.0 + 1, tail.1 + 1)
            } else {
                (tail.0 + 1, tail.1 - 1)
            }
        } else if head.1 > tail.1 {
            (tail.0 - 1, tail.1 + 1)
        } else {
            (tail.0 - 1, tail.1 - 1)
        }
    } else if head.0 > tail.0 {
        // not diagonal
        (tail.0 + 1, tail.1)
    } else if head.0 < tail.0 {
        (tail.0 - 1, tail.1)
    } else if head.1 > tail.1 {
        (tail.0, tail.1 + 1)
    } else if head.1 < tail.1 {
        (tail.0, tail.1 - 1)
    } else {
        panic!("Something went very wrong")
    }
}

pub fn parse_move(m: &str) -> (char, usize) {
    let dir = m.chars().next().unwrap();
    let dist = m[2..].parse::<usize>().unwrap();
    (dir, dist)
}

pub fn move_tails(head: (i32, i32), tails: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut new_tails = Vec::new();
    new_tails.push(move_tail(head, tails[0]));
    for i in 1..tails.len() {
        new_tails.push(move_tail(new_tails[i - 1], tails[i]));
    }
    new_tails
}

pub fn solve_a(data: &[String]) -> usize {
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    data.iter().for_each(|s| {
        let (dir, dist) = parse_move(s);
        for _ in 0..dist {
            head = move_head(head, dir);
            tail = move_tail(head, tail);
            tail_positions.insert(tail);
        }
    });
    tail_positions.len()
}

pub fn solve_b(data: &[String]) -> usize {
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tails = vec![(0, 0); 9];

    data.iter().for_each(|s| {
        let (dir, dist) = parse_move(s);
        for _ in 0..dist {
            head = move_head(head, dir);
            tails = move_tails(head, &tails);
            tail_positions.insert(tails[8]);
        }
    });
    tail_positions.len()
}
