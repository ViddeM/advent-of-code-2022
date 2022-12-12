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

#[derive(Clone, Debug)]
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

pub fn solve_part_one<'a>(input: Map) -> String {
    let mut heap: BinaryHeap<QueueEntry> = BinaryHeap::new();
    let mut dist_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for (y, row) in input.map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            dist_map.insert((x, y), usize::MAX);
        }
    }

    dist_map.insert((input.start_pos.x, input.start_pos.y), 0);
    heap.push(QueueEntry {
        x: input.start_pos.x,
        y: input.start_pos.y,
        height: input.map[input.start_pos.y][input.start_pos.x],
        cost: 0,
    });

    while let Some(qe) = heap.pop() {
        if qe.x == input.destination.x && qe.y == input.destination.y {
            break;
        }

        if qe.cost > dist_map[&(qe.x, qe.y)] {
            // We have found a better alternative for this pos
            continue;
        }

        for (x, y) in get_neighbours(qe.x, qe.y, input.width, input.height) {
            let height = input.map[y][x];
            if height <= qe.height + 1 {
                let next = QueueEntry {
                    x,
                    y,
                    height,
                    cost: qe.cost + 1,
                };

                if next.cost < dist_map[&(next.x, next.y)] {
                    heap.push(next);
                    dist_map.insert((next.x, next.y), next.cost);
                    prev_map.insert((next.x, next.y), (qe.x, qe.y));
                }
            }
        }
    }

    // Should have reached goal!
    let steps = dist_map[&(input.destination.x, input.destination.y)];

    // let mut curr = (input.destination.x, input.destination.y);
    // while let Some(&(pre_x, pre_y)) = prev_map.get(&curr) {
    //     println!("({pre_x},{pre_y}) -- {}", input.map[pre_y][pre_x]);
    //     curr = (pre_x, pre_y);
    // }

    format!("{steps}")
}

pub fn solve_part_two<'a>(input: Map) -> String {
    todo!("Part two is not yet implemented");
}
