#[derive(Clone)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: fn(usize) -> usize,
    pub test: (usize, usize, usize),
    pub num_inspected: usize,
}

pub fn real_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![54, 53],
            operation: |old| old * 3,
            test: (2, 2, 6),
            num_inspected: 0,
        },
        Monkey {
            items: vec![95, 88, 75, 81, 91, 67, 65, 84],
            operation: |old| old * 11,
            test: (7, 3, 4),
            num_inspected: 0,
        },
        Monkey {
            items: vec![76, 81, 50, 93, 96, 81, 83],
            operation: |old| old + 6,
            test: (3, 5, 1),
            num_inspected: 0,
        },
        Monkey {
            items: vec![83, 85, 85, 63],
            operation: |old| old + 4,
            test: (11, 7, 4),
            num_inspected: 0,
        },
        Monkey {
            items: vec![85, 52, 64],
            operation: |old| old + 8,
            test: (17, 0, 7),
            num_inspected: 0,
        },
        Monkey {
            items: vec![57],
            operation: |old| old + 2,
            test: (5, 1, 3),
            num_inspected: 0,
        },
        Monkey {
            items: vec![60, 95, 76, 66, 91],
            operation: |old| old * old,
            test: (13, 2, 5),
            num_inspected: 0,
        },
        Monkey {
            items: vec![65, 84, 76, 72, 79, 65],
            operation: |old| old + 5,
            test: (19, 6, 0),
            num_inspected: 0,
        },
    ]
}

pub fn one_monkey_round(monkeys: &[Monkey], index: usize, divide_by: usize) -> Vec<Monkey> {
    let mut new_monkeys = monkeys.to_vec();
    let monkey = &monkeys[index];

    monkey.items.iter().for_each(|old| {
        let mut new = (monkey.operation)(*old) / divide_by;
        // new = new % (23 * 19 * 13 * 17);
        let new_monkey_index = if new % monkey.test.0 == 0 {
            monkey.test.1
        } else {
            monkey.test.2
        };
        new_monkeys[new_monkey_index].items.push(new);
    });
    new_monkeys[index].items = vec![];
    new_monkeys[index].num_inspected += monkeys[index].items.len();
    new_monkeys
}

pub fn monkey_round(monkeys: &[Monkey], divide_by: usize) -> Vec<Monkey> {
    let mut new_monkeys = monkeys.to_vec();
    for i in 0..monkeys.len() {
        new_monkeys = one_monkey_round(&new_monkeys, i, divide_by);
    }
    new_monkeys
}

pub fn solve_a(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_vec();
    for _ in 0..20 {
        monkeys = monkey_round(&monkeys, 3);
    }
    monkeys.sort_by(|a, b| b.num_inspected.cmp(&a.num_inspected));
    monkeys[0].num_inspected * monkeys[1].num_inspected
}

pub fn solve_b(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_vec();
    for _ in 0..10000 {
        monkeys = monkey_round(&monkeys, 1);
    }
    monkeys.sort_by(|a, b| b.num_inspected.cmp(&a.num_inspected));
    monkeys[0].num_inspected * monkeys[1].num_inspected
}
