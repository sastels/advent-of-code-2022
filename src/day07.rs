use regex::Regex;
use std::collections::HashMap;

pub fn parse_chdir(line: &str) -> Option<String> {
    if line.contains("$ cd ") {
        let dir = line.split("$ cd ").last().unwrap();
        Some(dir.to_string())
    } else {
        None
    }
}

pub fn parse_dir_listing(line: &str) -> Option<(usize, String)> {
    let dir_list_regex = Regex::new(r"(\d+) (.*)").unwrap();
    if let Some(captures) = dir_list_regex.captures(line) {
        let size = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let name = captures.get(2).unwrap().as_str().to_string();
        Some((size, name))
    } else {
        None
    }
}

pub fn digest_data(data: &[String]) -> (Vec<String>, HashMap<String, usize>) {
    let mut dir_stack = vec![];
    let mut dirs = vec!["root".to_string()];
    let mut file_size: HashMap<String, usize> = HashMap::new();

    for line in data {
        if line == "$ cd /" {
            dir_stack = vec!["root".to_string()];
        } else if line == "$ cd .." {
            dir_stack.pop();
        } else if let Some(dir) = parse_chdir(line) {
            dir_stack.push(dir);
            dirs.push(dir_stack.join("/"));
        } else if let Some((size, name)) = parse_dir_listing(line) {
            let mut path = dir_stack.join("/");
            path.push('/');
            path.push_str(&name);
            file_size.insert(path.clone(), size);
        }
    }
    (dirs, file_size)
}

pub fn dir_size(dir: &str, file_size: &HashMap<String, usize>) -> usize {
    let mut total_size = 0;
    for (path, size) in file_size {
        if path.starts_with(dir) {
            total_size += size;
        }
    }
    total_size
}

pub fn solve_a(data: &[String]) -> usize {
    let (dirs, file_size) = digest_data(data);
    let mut small_dir_sum = 0;
    for dir in dirs {
        if dir_size(&dir, &file_size) < 100000 {
            small_dir_sum += dir_size(&dir, &file_size);
        }
    }
    small_dir_sum
}

pub fn solve_b(data: &[String]) -> usize {
    let (dirs, file_size) = digest_data(data);
    let size_needed = 30000000 - (70000000 - dir_size("root", &file_size));
    let mut smallest_dir_to_delete = dir_size("root", &file_size);
    for dir in dirs {
        let size = dir_size(&dir, &file_size);
        if size > size_needed && size < smallest_dir_to_delete {
            smallest_dir_to_delete = dir_size(&dir, &file_size);
        }
    }
    smallest_dir_to_delete
}
