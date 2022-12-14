use crate::utils;

fn total_elf_food(data: &[String]) -> Vec<i32> {
    utils::join_lines(data)
        .iter()
        .map(|s| utils::string_to_vec_ints(s))
        .map(|v| v.iter().sum::<i32>())
        .collect()
}

pub fn solve_a(data: &[String]) -> i32 {
    *total_elf_food(data).iter().max().unwrap()
}

pub fn solve_b(data: &[String]) -> i32 {
    let mut elf_food = total_elf_food(data);
    elf_food.sort();
    elf_food.reverse();
    elf_food[0] + elf_food[1] + elf_food[2]
}
