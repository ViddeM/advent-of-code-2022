use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn parse(coord: &str) -> Self {
        let (x_str, y_str) = coord.split_once(", ").unwrap();
        let x_str = x_str.strip_prefix("x=").unwrap();
        let y_str = y_str.strip_prefix("y=").unwrap();

        Position {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        }
    }

    #[inline(always)]
    fn dist_to(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    sensor_beacon_map: HashMap<Position, Position>,
}

pub fn parse<'a>(input: &str) -> Map {
    let mut map = HashMap::new();

    input
        .lines()
        .map(|l| {
            let without_prefix = l.strip_prefix("Sensor at ").unwrap();
            let (sensor, beacon) = without_prefix
                .split_once(": closest beacon is at ")
                .unwrap();
            (Position::parse(sensor), Position::parse(beacon))
        })
        .for_each(|(sensor, beacon)| {
            map.insert(sensor, beacon);
        });

    Map {
        sensor_beacon_map: map,
    }
}

const WANTED_ROW: i64 = 2_000_000;
const EDGE: i64 = 9_000_000;
pub fn solve_part_one<'a>(input: Map) -> String {
    let sum: u64 = (-EDGE..EDGE)
        .map(|x| {
            let mut count_x = false;
            for (sensor, beacon) in input.sensor_beacon_map.iter() {
                if beacon.x == x && beacon.y == WANTED_ROW {
                    // Can't exist a beacon where there already is one
                    break;
                }

                if (sensor.x - x).abs() + (sensor.y - WANTED_ROW).abs() <= sensor.dist_to(beacon) {
                    count_x = true;
                    break;
                }
            }

            if count_x {
                1
            } else {
                0
            }
        })
        .sum();

    format!("{sum}")
}

const SEARCH_AREA_EDGE: i64 = 4000000;
// const SEARCH_AREA_EDGE: i64 = 20;
fn find_coord(sensor_beacon_map: &HashMap<Position, Position>) -> Position {
    for (sensor, beacon) in sensor_beacon_map.iter() {
        let dist = sensor.dist_to(beacon);
        let from = max(sensor.x - dist - 1, 0);
        let to = min(sensor.x + dist + 1, SEARCH_AREA_EDGE);

        for x in from..=to {
            let above_y = min(sensor.y + dist + 1 - (sensor.x - x).abs(), SEARCH_AREA_EDGE);
            let above_pos = Position { x, y: above_y };

            let mut is_above = true;
            for (inner_sensor, inner_beacon) in sensor_beacon_map.iter() {
                let inner_dist = inner_sensor.dist_to(inner_beacon);
                let sensor_pos_dist = inner_sensor.dist_to(&above_pos);
                if sensor_pos_dist <= inner_dist {
                    is_above = false;
                }
            }
            if is_above {
                return above_pos;
            }

            let below_y = max(sensor.y - (dist + 1 - (sensor.x - x).abs()), 0);
            let below_pos = Position { x, y: below_y };

            let mut is_below = true;
            for (inner_sensor, inner_beacon) in sensor_beacon_map.iter() {
                let inner_dist = inner_sensor.dist_to(inner_beacon);
                let sensor_pos_dist = inner_sensor.dist_to(&below_pos);
                if sensor_pos_dist <= inner_dist {
                    is_below = false;
                }
            }
            if is_below {
                return below_pos;
            }
        }
    }

    panic!("Failed to find position :(");
}

pub fn solve_part_two<'a>(input: Map) -> String {
    let pos = find_coord(&input.sensor_beacon_map);

    let val = pos.x * SEARCH_AREA_EDGE + pos.y;
    format!("{val}")
}
