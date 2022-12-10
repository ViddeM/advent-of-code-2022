use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone)]
pub enum Dir {
    R,
    L,
    U,
    D,
}

impl Dir {
    fn get_delta(&self) -> (i32, i32) {
        match self {
            Dir::R => (1, 0),
            Dir::L => (-1, 0),
            Dir::U => (0, 1),
            Dir::D => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    dir: Dir,
    count: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn dist(&self, other: &Position) -> f32 {
        let dist_x = (self.x - other.x).abs();
        let dist_y = (self.y - other.y).abs();

        return ((dist_x.pow(2) + dist_y.pow(2)) as f32).sqrt();
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
enum HeadPos {
    Same,
    Above,
    AboveRight,
    Right,
    BelowRight,
    Below,
    BelowLeft,
    Left,
    AboveLeft,
}

impl Display for HeadPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HeadPos::Same => "S",
                HeadPos::Above => "A",
                HeadPos::AboveRight => "AR",
                HeadPos::Right => "R",
                HeadPos::BelowRight => "BR",
                HeadPos::Below => "B",
                HeadPos::BelowLeft => "BL",
                HeadPos::Left => "L",
                HeadPos::AboveLeft => "AL",
            }
        )
    }
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    input.lines().map(|l| {
        let (dir, count) = l.split_once(" ").unwrap();
        let num = count.parse::<u8>().unwrap();
        Instruction {
            dir: match dir {
                "R" => Dir::R,
                "L" => Dir::L,
                "U" => Dir::U,
                "D" => Dir::D,
                a => panic!("Invalid direction {a}"),
            },
            count: num,
        }
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Instruction>) -> String {
    let mut head_pos = HeadPos::Same;
    let mut tail_pos = Position { x: 0, y: 0 };
    let mut visited: HashSet<Position> = HashSet::new();

    for ins in input {
        for _ in 0..ins.count {
            let (new_head_pos, new_tail_pos) = match ins.dir {
                Dir::R => move_right(&head_pos, &tail_pos),
                Dir::L => move_left(&head_pos, &tail_pos),
                Dir::U => move_up(&head_pos, &tail_pos),
                Dir::D => move_down(&head_pos, &tail_pos),
            };

            head_pos = new_head_pos;
            visited.insert(tail_pos);
            tail_pos = new_tail_pos;
        }
    }

    visited.insert(tail_pos);

    // TODO: Off-by-one?
    format!("{}", visited.len())
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = Instruction>) -> String {
    let mut rope = vec![Position { x: 0, y: 0 }; 9];
    let mut visited: HashSet<Position> = HashSet::new();

    let mut head_pos = Position { x: 0, y: 0 };
    for ins in input {
        let (dx, dy) = ins.dir.get_delta();

        for _ in 0..ins.count {
            head_pos = Position {
                x: head_pos.x + dx,
                y: head_pos.y + dy,
            };

            let mut prev_pos = head_pos.clone();
            for knot_index in 0..rope.len() {
                prev_pos = move_knot(&prev_pos, rope.get(knot_index).unwrap());
                rope[knot_index] = prev_pos.clone();
            }

            visited.insert(rope.last().unwrap().clone());
        }
    }

    format!("{}", visited.len())
}

fn print_rope(rope: &Vec<Position>) {
    let val = rope
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("[{val}]");
}

fn move_knot(head_pos: &Position, tail_pos: &Position) -> Position {
    if (tail_pos.y - head_pos.y).abs() > 1 && tail_pos.x == head_pos.x {
        Position {
            x: tail_pos.x,
            y: tail_pos.y + if head_pos.y > tail_pos.y { 1 } else { -1 },
        }
    } else if (tail_pos.x - head_pos.x).abs() > 1 && tail_pos.y == head_pos.y {
        Position {
            x: tail_pos.x + if head_pos.x > tail_pos.x { 1 } else { -1 },
            y: tail_pos.y,
        }
    } else if head_pos.dist(tail_pos) > 2f32 {
        Position {
            x: tail_pos.x + if head_pos.x > tail_pos.x { 1 } else { -1 },
            y: tail_pos.y + if head_pos.y > tail_pos.y { 1 } else { -1 },
        }
    } else {
        tail_pos.clone()
    }
}

fn move_right(head_pos: &HeadPos, tail_pos: &Position) -> (HeadPos, Position) {
    match head_pos {
        HeadPos::Same => (HeadPos::Right, tail_pos.clone()),
        HeadPos::Above => (HeadPos::AboveRight, tail_pos.clone()),
        HeadPos::AboveRight => (
            HeadPos::Right,
            Position {
                x: tail_pos.x + 1,
                y: tail_pos.y + 1,
            },
        ),
        HeadPos::Right => (
            HeadPos::Right,
            Position {
                x: tail_pos.x + 1,
                y: tail_pos.y,
            },
        ),
        HeadPos::BelowRight => (
            HeadPos::Right,
            Position {
                x: tail_pos.x + 1,
                y: tail_pos.y - 1,
            },
        ),
        HeadPos::Below => (HeadPos::BelowRight, tail_pos.clone()),
        HeadPos::BelowLeft => (HeadPos::Below, tail_pos.clone()),
        HeadPos::Left => (HeadPos::Same, tail_pos.clone()),
        HeadPos::AboveLeft => (HeadPos::Above, tail_pos.clone()),
    }
}

fn move_left(head_pos: &HeadPos, tail_pos: &Position) -> (HeadPos, Position) {
    match head_pos {
        HeadPos::Same => (HeadPos::Left, tail_pos.clone()),
        HeadPos::Above => (HeadPos::AboveLeft, tail_pos.clone()),
        HeadPos::AboveRight => (HeadPos::Above, tail_pos.clone()),
        HeadPos::Right => (HeadPos::Same, tail_pos.clone()),
        HeadPos::BelowRight => (HeadPos::Below, tail_pos.clone()),
        HeadPos::Below => (HeadPos::BelowLeft, tail_pos.clone()),
        HeadPos::BelowLeft => (
            HeadPos::Left,
            Position {
                x: tail_pos.x - 1,
                y: tail_pos.y - 1,
            },
        ),
        HeadPos::Left => (
            HeadPos::Left,
            Position {
                x: tail_pos.x - 1,
                y: tail_pos.y,
            },
        ),
        HeadPos::AboveLeft => (
            HeadPos::Left,
            Position {
                x: tail_pos.x - 1,
                y: tail_pos.y + 1,
            },
        ),
    }
}

fn move_up(head_pos: &HeadPos, tail_pos: &Position) -> (HeadPos, Position) {
    match head_pos {
        HeadPos::Same => (HeadPos::Above, tail_pos.clone()),
        HeadPos::Above => (
            HeadPos::Above,
            Position {
                x: tail_pos.x,
                y: tail_pos.y + 1,
            },
        ),
        HeadPos::AboveRight => (
            HeadPos::Above,
            Position {
                x: tail_pos.x + 1,
                y: tail_pos.y + 1,
            },
        ),
        HeadPos::Right => (HeadPos::AboveRight, tail_pos.clone()),
        HeadPos::BelowRight => (HeadPos::Right, tail_pos.clone()),
        HeadPos::Below => (HeadPos::Same, tail_pos.clone()),
        HeadPos::BelowLeft => (HeadPos::Left, tail_pos.clone()),
        HeadPos::Left => (HeadPos::AboveLeft, tail_pos.clone()),
        HeadPos::AboveLeft => (
            HeadPos::Above,
            Position {
                x: tail_pos.x - 1,
                y: tail_pos.y + 1,
            },
        ),
    }
}

fn move_down(head_pos: &HeadPos, tail_pos: &Position) -> (HeadPos, Position) {
    match head_pos {
        HeadPos::Same => (HeadPos::Below, tail_pos.clone()),
        HeadPos::Above => (HeadPos::Same, tail_pos.clone()),
        HeadPos::AboveRight => (HeadPos::Right, tail_pos.clone()),
        HeadPos::Right => (HeadPos::BelowRight, tail_pos.clone()),
        HeadPos::BelowRight => (
            HeadPos::Below,
            Position {
                x: tail_pos.x + 1,
                y: tail_pos.y - 1,
            },
        ),
        HeadPos::Below => (
            HeadPos::Below,
            Position {
                x: tail_pos.x,
                y: tail_pos.y - 1,
            },
        ),
        HeadPos::BelowLeft => (
            HeadPos::Below,
            Position {
                x: tail_pos.x - 1,
                y: tail_pos.y - 1,
            },
        ),
        HeadPos::Left => (HeadPos::BelowLeft, tail_pos.clone()),
        HeadPos::AboveLeft => (HeadPos::Left, tail_pos.clone()),
    }
}
