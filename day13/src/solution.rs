use std::str::Chars;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Int(u32),
    List(Vec<Value>),
}

impl Value {
    fn to_str(&self) -> String {
        match self {
            Value::Int(a) => a.to_string(),
            Value::List(vals) => format!(
                "[{}]",
                vals.iter()
                    .map(|v| v.to_str())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    index: u32,
    first: Value,
    second: Value,
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Pair> + 'a {
    input
        .strip_suffix("\n")
        .unwrap()
        .split("\n\n")
        .enumerate()
        .map(|(index, pair)| {
            let (first, second) = pair.split_once("\n").unwrap();

            fn to_num(chars: String) -> Value {
                Value::Int(chars.parse::<u32>().unwrap())
            }

            fn parse_list(line: &mut Chars) -> Value {
                let mut list: Vec<Value> = vec![];
                let mut number: String = String::new();

                while let Some(c) = line.next() {
                    match c {
                        '[' => {
                            if number.len() > 0 {
                                list.push(to_num(number));
                                number = String::new();
                            }

                            // Sublist
                            let val = parse_list(line);
                            list.push(val);
                        }
                        ']' => {
                            if number.len() > 0 {
                                list.push(to_num(number));
                                number = String::new();
                            }

                            return Value::List(list);
                        }
                        ',' => {
                            if number.len() > 0 {
                                list.push(to_num(number));
                                number = String::new();
                            }
                        } // Ignore
                        digit => number = format!("{number}{digit}"),
                    }
                }

                if number.len() > 0 {
                    list.push(to_num(number));
                }

                Value::List(list)
            }

            let first = first.strip_prefix("[").unwrap();
            let first = first.strip_suffix("]").unwrap();
            let mut first_chars = first.chars();
            let first_list = parse_list(&mut first_chars);

            let second = second.strip_prefix("[").unwrap();
            let second = second.strip_suffix("]").unwrap();
            let mut second_chars = second.chars();
            let second_list = parse_list(&mut second_chars);

            Pair {
                index: index as u32 + 1,
                first: first_list,
                second: second_list,
            }
        })
}

#[inline(always)]
fn compare_values(left: &Value, right: &Value, log: bool) -> Option<bool> {
    if log {
        println!("Comparing {} vs {}", left.to_str(), right.to_str());
    }

    match (left, right) {
        (Value::Int(l), Value::Int(r)) => {
            if l < r {
                if log {
                    println!("\t Right int was higher!");
                }
                return Some(true);
            }

            if l > r {
                if log {
                    println!("\t Left int was higher ;(");
                }
                return Some(false);
            }
        }
        (Value::List(_), Value::Int(_)) => {
            return compare_values(left, &Value::List(vec![right.clone()]), log)
        }
        (Value::Int(_), Value::List(_)) => {
            return compare_values(&Value::List(vec![left.clone()]), right, log)
        }
        (Value::List(l), Value::List(r)) => {
            let mut l = l.into_iter();
            let mut r = r.into_iter();
            while let Some(left_val) = l.next() {
                if let Some(right_val) = r.next() {
                    if let Some(v) = compare_values(left_val, right_val, log) {
                        return Some(v);
                    }
                } else {
                    // Left list was longer
                    return Some(false);
                }
            }

            if let Some(_) = r.next() {
                // Right list was longer
                return Some(true);
            }
        }
    }

    None
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Pair>) -> String {
    let sum = input
        .map(|pair| {
            let in_order = compare_values(&pair.first, &pair.second, false);

            if in_order.expect("Failed to calc list") {
                pair.index as i32
            } else {
                -1
            }
        })
        .filter(|&index| index > 0)
        .sum::<i32>();

    format!("{sum}")
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Pair>) -> String {
    let key_a = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let key_b = Value::List(vec![Value::List(vec![Value::Int(6)])]);

    let mut lines: Vec<Value> = vec![key_a.clone(), key_b.clone()];

    for pair in input {
        lines.push(pair.first);
        lines.push(pair.second);
    }

    lines.sort_by(|a, b| {
        let res = compare_values(a, b, false);
        match res {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            None => std::cmp::Ordering::Equal,
        }
    });

    let mut key_a_index = 0;
    let mut key_b_index = 0;
    for (index, l) in lines.into_iter().enumerate() {
        if l == key_a {
            key_a_index = index + 1;
        }

        if l == key_b {
            key_b_index = index + 1;
        }
    }

    format!("{}", key_a_index * key_b_index)
}
