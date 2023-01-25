use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

type Operation = Box<dyn Fn(u64) -> u64>;

struct Monkey {
    items: VecDeque<u64>,

    operation: Operation,
    test: u64,

    on_true: usize,
    on_false: usize,

    inspections: u64,
}

fn pop(lines: &mut VecDeque<String>) -> String {
    lines.pop_front().unwrap().trim().to_string()
}

fn load_items(item_string: &String) -> VecDeque<u64> {
    // Starting items: 79, 98
    let tokens: Vec<&str> = item_string.split(" ").collect();

    tokens[2..]
        .iter()
        .map(|s| s.replace(",", "").parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>()
}

fn load_operation(operation_string: &String) -> Operation {
    // Operation: new = old * 19
    let tokens: Vec<&str> = operation_string.split(" ").collect();

    let operation = tokens.get(4).unwrap();
    let value = tokens.get(5).unwrap();

    if *value == "old" {
        match *operation {
            "*" => Box::new(|x: u64| x.clone() * x),
            "+" => Box::new(|x: u64| x.clone() + x),
            _ => panic!("unexpected operation"),
        }
    } else {
        let value = value.parse::<u64>().unwrap();

        match *operation {
            "*" => Box::new(move |x: u64| x * value),
            "+" => Box::new(move |x: u64| x + value),
            _ => panic!("unexpected operation"),
        }
    }
}

fn load_test(test_string: &String) -> u64 {
    // Test: divisible by 23
    let tokens: Vec<&str> = test_string.split(" ").collect();

    tokens.get(3).unwrap().parse::<u64>().unwrap()
}

fn load_throw(throw_string: &String) -> usize {
    // If [true|false]: throw to monkey 2
    let tokens: Vec<&str> = throw_string.split(" ").collect();

    tokens.get(5).unwrap().parse::<usize>().unwrap()
}

fn load_monkey(lines: &mut VecDeque<String>) -> Monkey {
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3

    pop(lines);
    let items = load_items(&pop(lines));
    let operation = load_operation(&pop(lines));
    let test = load_test(&pop(lines));
    let on_true = load_throw(&pop(lines));
    let on_false = load_throw(&pop(lines));

    if let Some(s) = lines.front() {
        if s.is_empty() {
            lines.pop_front();
        }
    }

    Monkey {
        items,
        operation,
        test,
        on_true,
        on_false,
        inspections: 0,
    }
}

fn load_monkeys(file: &str) -> Vec<Monkey> {
    let mut result: Vec<Monkey> = Vec::new();

    let file = File::open(file).unwrap();

    let mut lines: VecDeque<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    while !lines.is_empty() {
        result.push(load_monkey(&mut lines));
    }

    result
}

fn process_round(monkeys: &mut Vec<Monkey>, worry_management: &impl Fn(u64) -> u64) {
    for i in 0..monkeys.len() {
        let mut to_throw = vec![];

        let m = monkeys.get_mut(i).unwrap();
        while !m.items.is_empty() {
            let mut item = m.items.pop_front().unwrap();

            m.inspections += 1;

            item = (m.operation)(item);
            item = worry_management(item);

            if item % m.test == 0 {
                to_throw.push((item, m.on_true));
            } else {
                to_throw.push((item, m.on_false));
            }
        }

        for (item, target) in to_throw {
            monkeys.get_mut(target).unwrap().items.push_back(item);
        }
    }
}

fn calculate(file: &str) -> u64 {
    let mut monkeys = load_monkeys(file);

    process_rounds(&mut monkeys, 20, &Box::new(|x: u64| x / 3))
}

fn calculate_part2(file: &str) -> u64 {
    let mut monkeys = load_monkeys(file);

    let mut mod_all: u64 = 1;
    for m in monkeys.iter() {
        mod_all *= m.test;
    }

    process_rounds(&mut monkeys, 10000, move |x: u64| x % mod_all)
}

fn process_rounds(monkeys: &mut Vec<Monkey>, rounds: u32, worry_management: impl Fn(u64) -> u64) -> u64 {
    for _ in 0..rounds {
        process_round(monkeys, &worry_management);
    }

    let mut max: u64 = 0;
    let mut second: u64 = 0;

    for m in monkeys {
        if m.inspections > max {
            second = max;
            max = m.inspections;
        } else if m.inspections > second {
            second = m.inspections;
        }
    }

    println!("max: {} second: {}", max, second);

    max * second
}

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
    println!("result part 2: {}", calculate_part2("input/problem.txt"));
}

#[test]
fn test_example() {
    assert_eq!(10605, calculate("input/example.txt"))
}

#[test]
fn test_example_part2() {
    assert_eq!(2713310158, calculate_part2("input/example.txt"))
}
