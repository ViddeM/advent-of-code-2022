use std::collections::HashMap;

pub fn parse<'a>(
    input: &'a str,
) -> (
    HashMap<u8, Vec<char>>,
    impl Iterator<Item = (u32, u8, u8)> + 'a,
) {
    let (graph, instructions) = input.split_once("\n\n").unwrap();

    let mut map: HashMap<u8, Vec<char>> = HashMap::new();
    for l in graph.lines().rev() {
        for (index, box_char_index) in (1..l.len()).step_by(4).enumerate() {
            let index = (index + 1) as u8;
            let char = l.chars().nth(box_char_index).unwrap();
            if char.is_digit(10) {
                // Skip numbered row
                break;
            }

            if char == ' ' {
                // Skip empty
                continue;
            }

            if map.contains_key(&index) {
                let stack = map.get_mut(&index).unwrap();
                stack.push(char);
            } else {
                map.insert(index, vec![char]);
            };
        }
    }

    let ins_iter = instructions.lines().map(|l| {
        let (count, rest) = l
            .strip_prefix("move ")
            .unwrap()
            .split_once(" from ")
            .unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        (
            u32::from_str_radix(count, 10).unwrap(),
            u8::from_str_radix(from, 10).unwrap(),
            u8::from_str_radix(to, 10).unwrap(),
        )
    });

    (map, ins_iter)
}

pub fn solve_part_one<'a>(
    input: (
        HashMap<u8, Vec<char>>,
        impl Iterator<Item = (u32, u8, u8)> + 'a,
    ),
) -> String {
    let (mut map, instructions) = input;
    for (count, from, to) in instructions {
        let mut tmp_stack = Vec::new();
        {
            let from_stack = map.get_mut(&from).unwrap();
            for _ in 0..count {
                tmp_stack.push(from_stack.pop().unwrap());
            }
        }
        tmp_stack.reverse();
        {
            let to_stack = map.get_mut(&to).unwrap();
            for _ in 0..count {
                to_stack.push(tmp_stack.pop().unwrap());
            }
        }
    }

    let mut solution = String::new();
    for stack_index in 1..(map.len() + 1) {
        let stack = map.get_mut(&(stack_index as u8)).unwrap();
        solution = format!("{solution}{}", stack.pop().unwrap());
    }

    format!("{solution}")
}

pub fn solve_part_two<'a>(
    input: (
        HashMap<u8, Vec<char>>,
        impl Iterator<Item = (u32, u8, u8)> + 'a,
    ),
) -> String {
    let (mut map, instructions) = input;
    for (count, from, to) in instructions {
        let mut tmp_stack = Vec::new();
        {
            let from_stack = map.get_mut(&from).unwrap();
            for _ in 0..count {
                tmp_stack.push(from_stack.pop().unwrap());
            }
        }
        {
            let to_stack = map.get_mut(&to).unwrap();
            for _ in 0..count {
                to_stack.push(tmp_stack.pop().unwrap());
            }
        }
    }

    let mut solution = String::new();
    for stack_index in 1..(map.len() + 1) {
        let stack = map.get_mut(&(stack_index as u8)).unwrap();
        solution = format!("{solution}{}", stack.pop().unwrap());
    }

    format!("{solution}")
}

fn print_stacks(map: &HashMap<u8, Vec<char>>) {
    for (key, stack) in map.iter() {
        println!(
            "{key}: {}",
            stack.iter().map(|c| format!("[{c}]")).collect::<String>()
        );
    }
}
