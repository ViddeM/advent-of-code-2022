use std::{collections::HashMap, ops::Div};

#[derive(Debug, Clone)]
pub struct Test {
    test_divisible_by: u64,
    if_true_throw_to: u64,
    if_false_throw_to: u64,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Plus(u64),
    PlusSelf,
    Times(u64),
    TimesSelf,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    number: u64,
    starting_items: Vec<u64>,
    operation: Operation,
    test: Test,
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Monkey> + 'a {
    input.split("\n\n").map(|monkey_text| {
        let monkey_lines = monkey_text.split('\n').collect::<Vec<&str>>();

        let first = monkey_lines[0].strip_suffix(":").unwrap();
        let (_, number) = first.split_once(" ").unwrap();
        let number: u64 = number.parse().unwrap();

        let (_, items) = monkey_lines[1].split_once(": ").unwrap();
        let starting_items = items
            .trim()
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let (_, operation) = monkey_lines[2].split_once(" = old ").unwrap();
        let (operator, value) = operation.split_once(" ").unwrap();
        let operation = match (operator, value) {
            ("*", "old") => Operation::TimesSelf,
            ("*", num) => Operation::Times(num.parse::<u64>().unwrap()),
            ("+", "old") => Operation::PlusSelf,
            ("+", num) => Operation::Plus(num.parse::<u64>().unwrap()),
            (op, val) => panic!("Operation '{op}' is not supported for value '{val}'"),
        };

        let (_, test) = monkey_lines[3].split_once(": ").unwrap();
        let (op, num) = test.split_once(" by ").unwrap();
        if op != "divisible" {
            panic!("Only divisible operation is supported for test!");
        }
        let if_true = monkey_lines[4]
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let if_false = monkey_lines[5]
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let test = Test {
            test_divisible_by: num.parse().unwrap(),
            if_true_throw_to: if_true,
            if_false_throw_to: if_false,
        };

        Monkey {
            number,
            starting_items,
            operation,
            test,
        }
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Monkey>) -> String {
    let mut monkey_inspections: HashMap<u64, u64> = HashMap::new();
    let mut monkeys: HashMap<u64, Monkey> = input.map(|m| (m.number.clone(), m)).collect();

    for number in monkeys.keys() {
        monkey_inspections.insert(number.clone(), 0);
    }

    for _round in 1..=20 {
        for monkey_index in 0..(monkeys.len() as u64) {
            let monkey = monkeys.get(&monkey_index).unwrap();
            let monkey_items = monkey.starting_items.to_owned();

            let mut throw_to_map: HashMap<u64, Vec<u64>> = HashMap::new();

            for item in monkey_items.into_iter() {
                let num = monkey_inspections.get(&monkey.number).unwrap();
                monkey_inspections.insert(monkey.number, num + 1);

                let new_worry_level = match monkey.operation {
                    Operation::Plus(v) => item + v,
                    Operation::PlusSelf => item + item,
                    Operation::Times(v) => item * v,
                    Operation::TimesSelf => item * item,
                };
                let new_item = (new_worry_level as f64) / 3f64;
                let new_item = new_item as u64;

                let throw_to = if new_item % monkey.test.test_divisible_by == 0 {
                    monkey.test.if_true_throw_to
                } else {
                    monkey.test.if_false_throw_to
                };

                if let Some(vals) = throw_to_map.get_mut(&throw_to) {
                    vals.push(new_item);
                } else {
                    throw_to_map.insert(throw_to, vec![new_item]);
                }
            }

            monkeys.get_mut(&monkey_index).unwrap().starting_items = vec![];
            for (monkey, items) in throw_to_map.into_iter() {
                for item in items {
                    monkeys.get_mut(&monkey).unwrap().starting_items.push(item);
                }
            }
        }
    }

    let mut inspections = monkey_inspections
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<u64>>();
    inspections.sort();
    inspections.reverse();
    let highest = inspections.get(0).unwrap();
    let second_highest = inspections.get(1).unwrap();

    format!("{}", highest * second_highest)
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Monkey>) -> String {
    let mut monkey_inspections: HashMap<u64, u64> = HashMap::new();
    let mut monkeys: HashMap<u64, Monkey> = input.map(|m| (m.number.clone(), m)).collect();

    let test_vals = monkeys
        .values()
        .map(|m| m.test.test_divisible_by)
        .collect::<Vec<u64>>();
    let lcm = lcm(&test_vals);

    for number in monkeys.keys() {
        monkey_inspections.insert(number.clone(), 0);
    }

    for _round in 1..=10_000 {
        for monkey_index in 0..(monkeys.len() as u64) {
            let monkey = monkeys.get(&monkey_index).unwrap();
            let monkey_items = monkey.starting_items.to_owned();

            let mut throw_to_map: HashMap<u64, Vec<u64>> = HashMap::new();

            for item in monkey_items.into_iter() {
                let num = monkey_inspections.get(&monkey.number).unwrap();
                monkey_inspections.insert(monkey.number, num + 1);

                let new_worry_level = match monkey.operation {
                    Operation::Plus(v) => item + v,
                    Operation::PlusSelf => item + item,
                    Operation::Times(v) => item * v,
                    Operation::TimesSelf => item * item,
                };

                let new_item = new_worry_level % lcm;

                let throw_to = if new_item % monkey.test.test_divisible_by == 0 {
                    monkey.test.if_true_throw_to
                } else {
                    monkey.test.if_false_throw_to
                };

                if let Some(vals) = throw_to_map.get_mut(&throw_to) {
                    vals.push(new_item);
                } else {
                    throw_to_map.insert(throw_to, vec![new_item]);
                }
            }

            monkeys.get_mut(&monkey_index).unwrap().starting_items = vec![];
            for (monkey, items) in throw_to_map.into_iter() {
                for item in items {
                    monkeys.get_mut(&monkey).unwrap().starting_items.push(item);
                }
            }
        }
    }

    let mut inspections = monkey_inspections
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<u64>>();
    inspections.sort();
    inspections.reverse();
    let highest = inspections.get(0).unwrap();
    let second_highest = inspections.get(1).unwrap();

    format!("{}", highest * second_highest)
}

fn lcm(numbers: &Vec<u64>) -> u64 {
    let mut lcm = 1;
    for &number in numbers.iter() {
        lcm = lcm * number / gcd(lcm, number);
    }
    lcm
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
