use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

fn is_list(s: &str) -> bool {
    s.starts_with('[')
}

pub fn get_pairs(data: &[String]) -> Vec<(String, String)> {
    (0..data.len())
        .step_by(3)
        .map(|i| (data[i].to_string(), data[i + 1].to_string()))
        .collect()
}

pub fn pop_first(s: &str) -> (String, String) {
    // get rid of the brackets
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    // extract first element
    let mut first = String::new();
    let mut rest = String::from("[");
    let mut brackets = 0;
    let mut already_found_first = false;
    for c in chars {
        if already_found_first {
            rest.push(c);
        } else if brackets == 0 && c == ',' {
            already_found_first = true;
        } else {
            first.push(c);
            if c == '[' {
                brackets += 1;
            } else if c == ']' {
                brackets -= 1;
            }
        }
    }
    (first, rest + "]")
}

pub fn compare(a: &str, b: &str) -> Ordering {
    if is_list(a) && is_list(b) {
        if a == "[]" && b == "[]" {
            return Equal;
        } else if a == "[]" {
            return Less;
        } else if b == "[]" {
            return Greater;
        }

        let (a_first, a_rest) = pop_first(a);
        let (b_first, b_rest) = pop_first(b);

        let first_ordering = compare(&a_first, &b_first);
        if first_ordering == Equal {
            compare(&a_rest, &b_rest)
        } else {
            first_ordering
        }
    } else if is_list(a) {
        let b_list = "[".to_string() + b + "]";
        compare(a, &b_list)
    } else if is_list(b) {
        let a_list = "[".to_string() + a + "]";
        compare(&a_list, b)
    } else {
        let a_num = a.parse::<i32>().unwrap();
        let b_num = b.parse::<i32>().unwrap();
        a_num.cmp(&b_num)
    }
}

pub fn solve_a(data: &[String]) -> usize {
    get_pairs(data)
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| compare(a, b) != Greater)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solve_b(data: &[String]) -> usize {
    let mut data_b = Vec::from(data);
    data_b.retain(|s| !s.is_empty());
    data_b.push(String::from("[[2]]"));
    data_b.push(String::from("[[6]]"));

    data_b.sort_by(|a, b| compare(a, b));

    (data_b.iter().position(|s| s == "[[2]]").unwrap() + 1)
        * (data_b.iter().position(|s| s == "[[6]]").unwrap() + 1)
}
