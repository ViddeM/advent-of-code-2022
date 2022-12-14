#[derive(Clone, Debug)]
pub struct Map {
    map: Vec<Vec<bool>>,
    start_x: u32,
    width: u32,
    height: u32,
}

impl Map {
    #[inline(always)]
    fn get_at(&self, x: u32, y: u32) -> bool {
        self.map[y as usize][(x - self.start_x) as usize]
    }

    #[inline(always)]
    fn set_at(&mut self, x: u32, y: u32, val: bool) {
        self.map[y as usize][(x - self.start_x) as usize] = val;
    }

    fn print_map(&self) {
        for y in 0..self.height {
            let mut row_str = String::new();
            for x in 0..self.width {
                row_str = format!(
                    "{row_str}{}",
                    if self.map[y as usize][x as usize] {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!("{row_str}");
        }
    }
}

pub fn parse<'a>(input: &str) -> Map {
    let mut min_x = u32::MAX;
    let mut max_x = 0;
    let mut min_y = u32::MAX;
    let mut max_y = 0;

    let coords = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|coord| {
                    let (x, y) = coord.split_once(",").unwrap();
                    let x = x.parse::<u32>().unwrap();
                    let y = y.parse::<u32>().unwrap();

                    if x < min_x {
                        min_x = x;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }
                    if y > max_y {
                        max_y = y;
                    }

                    (x, y)
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect::<Vec<Vec<(u32, u32)>>>();

    let width = max_x + 1 - min_x;
    let height = max_y + 1;

    let mut map = Map {
        map: vec![vec![false; width as usize]; height as usize],
        start_x: min_x,
        width,
        height,
    };

    for rock_structure in coords.into_iter() {
        let mut prev_x = 0;
        let mut prev_y = 0;
        for (x, y) in rock_structure.into_iter() {
            if prev_x > 0 && prev_y > 0 {
                // Set all vals from previous pos to true
                if prev_x == x {
                    let start = if y < prev_y { y } else { prev_y };
                    let end = if y > prev_y { y } else { prev_y };
                    for y in start..=end {
                        map.set_at(x, y, true);
                    }
                } else if prev_y == y {
                    let start = if x < prev_x { x } else { prev_x };
                    let end = if x > prev_x { x } else { prev_x };
                    for x in start..=end {
                        map.set_at(x, y, true);
                    }
                } else {
                    panic!("Unable to draw diagonal lines!");
                }
            }

            prev_x = x;
            prev_y = y;
        }
    }

    map
}

fn find_sand_count_with_abyss(map: &mut Map) -> u32 {
    let mut sand_count = 0;
    'outer: loop {
        // println!("Sand no {round}");
        // map.print_map();

        let mut sand_x = 500u32;
        for sand_y in 0..(map.height - 1) {
            let check_y = sand_y + 1;

            // println!(
            //     "Checking ({sand_x}, {check_y}) {}",
            //     map.get_at(sand_x, check_y)
            // );
            if map.get_at(sand_x, check_y) {
                // println!("Sand below! Trying left");

                // Can't move downwards, try down-left
                sand_x = sand_x - 1;
                if sand_x < map.start_x {
                    // Sand will end up outside of map!
                    return sand_count;
                }

                if map.get_at(sand_x, check_y) {
                    // didn't work, try down-right
                    sand_x = sand_x + 2;

                    if sand_x > map.start_x + map.width {
                        // Sand will end up outside of map!
                        return sand_count;
                    }

                    if map.get_at(sand_x, check_y) {
                        // println!("Stopped!");

                        // There are no more places to move, stop
                        map.set_at(sand_x - 1, sand_y, true);
                        sand_count = sand_count + 1;
                        continue 'outer;
                    }
                }
            }
        }

        return sand_count;
    }
}
pub fn solve_part_one<'a>(input: Map) -> String {
    let mut map = input;
    let sand_count = find_sand_count_with_abyss(&mut map);

    format!("{sand_count}")
}

fn find_sand_count_with_floor(map: &mut Map) -> u32 {
    let mut sand_count = 0;
    'outer: while map.get_at(500, 0) == false {
        // map.print_map();

        sand_count = sand_count + 1;

        let mut sand_x = 500u32;
        for sand_y in 0..(map.height - 1) {
            let check_y = sand_y + 1;

            // println!("Getting at ({sand_x},{check_y})");
            if map.get_at(sand_x, check_y) {
                // Can't move downwards, try down-left
                sand_x = sand_x - 1;

                // println!("Getting at ({sand_x},{check_y})");
                if map.get_at(sand_x, check_y) {
                    // didn't work, try down-right
                    sand_x = sand_x + 2;

                    // println!("Getting at ({sand_x},{check_y})");
                    if map.get_at(sand_x, check_y) {
                        // There are no more places to move, stop
                        map.set_at(sand_x - 1, sand_y, true);
                        continue 'outer;
                    }
                }
            }
        }

        return sand_count;
    }

    sand_count
}

pub fn solve_part_two<'a>(input: Map) -> String {
    // let width = input.start_x * 2 + input.width;
    let width = input.start_x * 2 + input.width;
    let height = input.height;

    let mut map = Map {
        map: vec![vec![false; width as usize]; height as usize],
        start_x: 0,
        width: width,
        height: height + 2,
    };

    for y in 0..input.height {
        for x in input.start_x..(input.start_x + input.width) {
            let val = input.get_at(x, y);
            map.set_at(x, y, val);
        }
    }

    // Add floor
    map.map.push(vec![false; width as usize]);
    map.map.push(vec![true; width as usize]);

    let sand_count = find_sand_count_with_floor(&mut map);

    format!("{sand_count}")
}
