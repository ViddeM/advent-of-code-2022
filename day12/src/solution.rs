use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct QueueEntry {
    x: usize,
    y: usize,
    height: u8,
    cost: usize,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Map {
    start_pos: Position,
    destination: Position,
    width: usize,
    height: usize,
    map: Vec<Vec<u8>>,
}

pub fn parse<'a>(input: &str) -> Map {
    let mut start_pos: Position = Position { x: 0, y: 0 };
    let mut destination: Position = Position { x: 0, y: 0 };

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .map(|(x, &b)| match b {
                    b'S' => {
                        start_pos.x = x;
                        start_pos.y = y;
                        b'a' - 0x61
                    }
                    b'E' => {
                        destination.x = x;
                        destination.y = y;
                        b'z' - 0x61
                    }
                    b => b - 0x61,
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    Map {
        start_pos,
        destination,
        width: map[0].len(),
        height: map.len(),
        map,
    }
}

#[inline(always)]
fn get_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    // Add left
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    // Add up
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    // Add right
    if x < width - 1 {
        neighbours.push((x + 1, y));
    }
    // Add down
    if y < height - 1 {
        neighbours.push((x, y + 1));
    }

    neighbours
}

#[inline(always)]
fn path_find(
    start_pos: Position,
    dest_pos: &Position,
    map: &Vec<Vec<u8>>,
    width: usize,
    height: usize,
) -> usize {
    let mut heap: BinaryHeap<QueueEntry> = BinaryHeap::new();
    let mut dist_map: HashMap<Position, usize> = HashMap::new();
    let mut prev_map: HashMap<Position, Position> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            dist_map.insert(Position { x, y }, usize::MAX);
        }
    }

    dist_map.insert(start_pos.clone(), 0);
    heap.push(QueueEntry {
        x: start_pos.x,
        y: start_pos.y,
        height: map[start_pos.y][start_pos.x],
        cost: 0,
    });

    while let Some(qe) = heap.pop() {
        if qe.x == dest_pos.x && qe.y == dest_pos.y {
            break;
        }

        if qe.cost > dist_map[&Position { x: qe.x, y: qe.y }] {
            // We have found a better alternative for this pos
            continue;
        }

        for (x, y) in get_neighbours(qe.x, qe.y, width, height) {
            let height = map[y][x];
            if height <= qe.height + 1 {
                let next = QueueEntry {
                    x,
                    y,
                    height,
                    cost: qe.cost + 1,
                };

                let next_pos = Position {
                    x: next.x,
                    y: next.y,
                };

                if next.cost < dist_map[&next_pos] {
                    heap.push(next);
                    dist_map.insert(next_pos.clone(), next.cost);
                    prev_map.insert(next_pos, Position { x: qe.x, y: qe.y });
                }
            }
        }
    }

    dist_map[&dest_pos]
}

pub fn solve_part_one<'a>(input: Map) -> String {
    let steps = path_find(
        input.start_pos,
        &input.destination,
        &input.map,
        input.width,
        input.height,
    );

    // Should have reached goal!
    format!("{steps}")
}

pub fn solve_part_two<'a>(input: Map) -> String {
    let mut shortest_path = usize::MAX;

    for (y, row) in input.map.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            if height == 0 {
                let steps = path_find(
                    Position { x, y },
                    &input.destination,
                    &input.map,
                    input.width,
                    input.height,
                );

                if steps < shortest_path {
                    shortest_path = steps;
                }
            }
        }
    }

    format!("{shortest_path}")
}
