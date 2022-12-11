use advent_2022::day11::{monkey_round, one_monkey_round, solve_a, solve_b, Monkey};

fn test_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            operation: |old| old * 19,
            test: (23, 2, 3),
            num_inspected: 0,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |old| old + 6,
            test: (19, 2, 0),
            num_inspected: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: |old| old * old,
            test: (13, 1, 3),
            num_inspected: 0,
        },
        Monkey {
            items: vec![74],
            operation: |old| old + 3,
            test: (17, 0, 1),
            num_inspected: 0,
        },
    ]
}

#[test]
fn test_one_monkey_round() {
    let mut monkeys = test_monkeys();
    monkeys = one_monkey_round(&monkeys, 0, 3);
    assert_eq!(monkeys[0].items, vec![]);
    assert_eq!(monkeys[3].items, vec![74, 500, 620]);
}

#[test]
fn test_monkey_round() {
    let mut monkeys = test_monkeys();
    monkeys = monkey_round(&monkeys, 3);
    assert_eq!(monkeys[0].items, vec![20, 23, 27, 26]);
    assert_eq!(monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
    assert_eq!(monkeys[2].items, vec![]);
    assert_eq!(monkeys[3].items, vec![]);

    for _ in 0..4 {
        monkeys = monkey_round(&monkeys, 3);
    }
    assert_eq!(monkeys[0].items, vec![15, 17, 16, 88, 1037]);
    assert_eq!(monkeys[1].items, vec![20, 110, 205, 524, 72]);
    assert_eq!(monkeys[2].items, vec![]);
    assert_eq!(monkeys[3].items, vec![]);
}

#[test]
fn test_num_inspected() {
    let mut monkeys = test_monkeys();
    for _ in 0..20 {
        monkeys = monkey_round(&monkeys, 3);
    }
    assert_eq!(monkeys[0].num_inspected, 101);
    assert_eq!(monkeys[1].num_inspected, 95);
    assert_eq!(monkeys[2].num_inspected, 7);
    assert_eq!(monkeys[3].num_inspected, 105);
}

#[test]
fn test_solve_a() {
    let monkeys = test_monkeys();
    assert_eq!(solve_a(&monkeys), 10605);
}

#[test]
fn test_solve_b() {
    let monkeys = test_monkeys();
    assert_eq!(solve_b(&monkeys), 2713310158);
}
