#[derive(Clone, Debug)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    input.lines().map(|l| {
        if l == "noop" {
            Instruction::Noop
        } else {
            let (ins, num) = l.split_once(" ").unwrap();
            if ins != "addx" {
                panic!("Expected ins to be addx, got '{ins}'");
            }

            Instruction::AddX(num.parse().unwrap())
        }
    })
}

const INTERESTING_CYCLES: [u32; 6] = [20, 60, 100, 140, 180, 220];
pub fn solve_part_one<'a>(input: impl Iterator<Item = Instruction>) -> String {
    let mut input = input;

    let mut x = 1;
    let mut next_cycle_val = None;

    let mut sum = 0;

    for cycle in 1..=220 {
        if INTERESTING_CYCLES.contains(&(cycle as u32)) {
            sum += x * cycle;
        }

        match next_cycle_val {
            None => {
                let ins = input.next().unwrap();
                match ins {
                    Instruction::Noop => {
                        next_cycle_val = None;
                    }
                    Instruction::AddX(val) => {
                        next_cycle_val = Some(val);
                    }
                }
            }
            Some(val) => {
                x += val;
                next_cycle_val = None;
            }
        }
    }

    format!("{sum}")
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Instruction>) -> String {
    let mut input = input;
    let mut pos = 1;
    let mut next_cycle_val = None;
    let mut drawn_line = String::new();

    for cycle in 1..=240 {
        let draw_pos = (cycle - 1) % 40;
        let char_to_draw = if draw_pos >= pos - 1 && draw_pos <= pos + 1 {
            '#'
        } else {
            '.'
        };
        drawn_line = format!("{drawn_line}{char_to_draw}");

        if cycle % 40 == 0 {
            println!("{drawn_line}");
            drawn_line = String::new();
        }

        match next_cycle_val {
            None => {
                let ins = input.next().unwrap();
                match ins {
                    Instruction::Noop => {
                        next_cycle_val = None;
                    }
                    Instruction::AddX(val) => {
                        next_cycle_val = Some(val);
                    }
                }
            }
            Some(val) => {
                pos += val;
                next_cycle_val = None;
            }
        }
    }

    format!("{drawn_line}\nREAD ABOVE")
}
