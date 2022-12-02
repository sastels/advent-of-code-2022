pub fn decode_move(m: char) -> u32 {
    if m as u32 >= 'X' as u32 {
        m as u32 - 'X' as u32
    } else {
        m as u32 - 'A' as u32
    }
}

pub fn score_moves(my_move: u32, their_move: u32) -> u32 {
    if my_move == their_move {
        my_move + 1 + 3
    } else if (3 + my_move - their_move) % 3 == 1 {
        my_move + 1 + 6
    } else {
        my_move + 1
    }
}

pub fn score_round_a(round: &str) -> u32 {
    let my_move = decode_move(round.chars().last().unwrap());
    let their_move = decode_move(round.chars().next().unwrap());
    score_moves(my_move, their_move)
}

pub fn score_round_b(round: &str) -> u32 {
    let my_option = decode_move(round.chars().last().unwrap());
    let their_move = decode_move(round.chars().next().unwrap());
    let mut my_move = their_move;
    if my_option == 0 {
        my_move = (3 + their_move - 1) % 3;
    } else if my_option == 2 {
        my_move = (3 + their_move + 1) % 3;
    }
    score_moves(my_move, their_move)
}

pub fn solve_a(data: &[String]) -> u32 {
    data.iter().map(|round| score_round_a(round)).sum()
}

pub fn solve_b(data: &[String]) -> u32 {
    data.iter().map(|round| score_round_b(round)).sum()
}
