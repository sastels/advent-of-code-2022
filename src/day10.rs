pub fn parse_op(op_str: &str) -> (String, i32) {
    let mut val = 0;
    let mut op_split = op_str.split(' ');
    let op = op_split.next().unwrap();
    if op != "noop" {
        val = op_split.next().unwrap().parse::<i32>().unwrap();
    }
    (op.to_string(), val)
}

pub fn boost_signal(cycle: i32, register: i32) -> i32 {
    if cycle % 40 == 20 {
        register * cycle
    } else {
        0
    }
}

pub fn run_instructions(data: &[String]) -> Vec<i32> {
    let mut register = vec![];
    let mut current_register = 1;
    for line in data {
        let (op, val) = parse_op(line);
        if op == "noop" {
            register.push(current_register);
        } else if op == "addx" {
            register.push(current_register);
            register.push(current_register);
            current_register += val;
        }
    }
    register
}

pub fn get_pixels(registers: &[i32]) -> Vec<bool> {
    registers
        .iter()
        .enumerate()
        .map(|(i, reg)| ((i as i32) % 40 - *reg).abs() <= 1)
        .collect()
}

pub fn solve_a(data: &[String]) -> i32 {
    run_instructions(data)
        .iter()
        .enumerate()
        .map(|(i, reg)| boost_signal(i as i32 + 1, *reg))
        .sum()
}

pub fn solve_b(data: &[String]) -> Vec<bool> {
    let registers = run_instructions(data);
    let pixels = get_pixels(&registers);

    pixels.iter().enumerate().for_each(|(i, b)| {
        print!("{}", if *b { '#' } else { '.' });
        if i % 40 == 39 {
            println!();
        }
    });
    pixels
}
