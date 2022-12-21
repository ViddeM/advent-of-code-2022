use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Monkey {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Times(String, String),
    Div(String, String),
}

pub fn parse<'a>(input: &str) -> HashMap<String, Monkey> {
    input
        .lines()
        .map(|l| {
            let (name, job) = l.split_once(": ").unwrap();
            let monkey = if let Ok(num) = job.parse::<i64>() {
                Monkey::Number(num)
            } else {
                let (first, rest) = job.split_once(" ").unwrap();
                let (operator, second) = rest.split_once(" ").unwrap();
                match operator {
                    "+" => Monkey::Add(first.to_string(), second.to_string()),
                    "-" => Monkey::Sub(first.to_string(), second.to_string()),
                    "*" => Monkey::Times(first.to_string(), second.to_string()),
                    "/" => Monkey::Div(first.to_string(), second.to_string()),
                    a => panic!("Operator '{a}' not supported"),
                }
            };
            (name.to_string(), monkey)
        })
        .collect()
}

fn solve_rec<'a>(curr: &'a str, map: &HashMap<String, Monkey>) -> i64 {
    let monkey = map.get(curr).unwrap();
    match monkey {
        Monkey::Number(num) => num.clone(),
        Monkey::Add(a, b) => solve_rec(a, map) + solve_rec(b, map),
        Monkey::Sub(a, b) => solve_rec(a, map) - solve_rec(b, map),
        Monkey::Times(a, b) => solve_rec(a, map) * solve_rec(b, map),
        Monkey::Div(a, b) => solve_rec(a, map) / solve_rec(b, map),
    }
}

pub fn solve_part_one<'a>(input: HashMap<String, Monkey>) -> String {
    let val = solve_rec("root", &input);

    format!("{val}")
}

fn solve_rec_2<'a>(curr: &'a str, my_val: i64, map: &HashMap<String, Monkey>) -> i64 {
    if curr == "humn" {
        return my_val;
    }

    let monkey = map.get(curr).unwrap();
    match monkey {
        Monkey::Number(num) => num.clone(),
        Monkey::Add(a, b) => solve_rec_2(a, my_val, map) + solve_rec_2(b, my_val, map),
        Monkey::Sub(a, b) => solve_rec_2(a, my_val, map) - solve_rec_2(b, my_val, map),
        Monkey::Times(a, b) => solve_rec_2(a, my_val, map) * solve_rec_2(b, my_val, map),
        Monkey::Div(a, b) => solve_rec_2(a, my_val, map) / solve_rec_2(b, my_val, map),
    }
}

fn find_matching<'a>(start: &'a str, other_val: i64, map: &HashMap<String, Monkey>) -> i64 {
    let mut print_if = 1000;
    for my_val in 0..i64::MAX {
        if my_val == print_if {
            println!("Status: {my_val}...");
            print_if = print_if << 1;
        }
        let val_a = solve_rec_2(start, my_val, map);
        if val_a == other_val {
            return my_val;
        }
    }

    panic!("Failed to find matching value :(");
}

pub fn solve_part_two<'a>(input: HashMap<String, Monkey>) -> String {
    let val = if let Monkey::Add(a, b) = input.get("root").unwrap() {
        let val_b = solve_rec_2(b, 0, &input);
        find_matching(a, val_b, &input)
    } else {
        panic!("Root was not add?! {:?}", input.get("root").unwrap())
    };

    format!("{val}")
}
