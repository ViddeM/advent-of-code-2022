pub fn parse<'a>(input: &str) -> (Vec<Vec<u8>>, usize) {
    let mut width = 0;
    (
        input
            .lines()
            .map(|l| {
                width = l.len();
                l.as_bytes().into_iter().map(|i| i - 48).collect()
            })
            .collect(),
        width,
    )
}

pub fn solve_part_one<'a>(input: (Vec<Vec<u8>>, usize)) -> String {
    let (input, width) = input;
    let mut visible_map: Vec<Vec<bool>> = vec![vec![false; width]; input.len()];

    // Left to right
    for (y, row) in input.iter().enumerate() {
        let mut highest = -1;
        for (x, tree) in row.iter().enumerate() {
            if *tree as i32 > highest {
                visible_map[y][x] = true;
                highest = *tree as i32;
            }
        }
    }

    // Right to left
    for (y, row) in input.iter().enumerate() {
        let mut highest = -1;
        for (x, tree) in row.iter().enumerate().rev() {
            if *tree as i32 > highest {
                visible_map[y][x] = true;
                highest = *tree as i32;
            }
        }
    }

    let mut highest_row = vec![-1; width];
    // Up to down
    for (y, row) in input.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let highest = highest_row[x];
            if *tree as i32 > highest {
                visible_map[y][x] = true;
                highest_row[x] = *tree as i32;
            }
        }
    }

    let mut highest_row = vec![-1; width];
    // Down to up
    for (y, row) in input.iter().enumerate().rev() {
        for (x, tree) in row.iter().enumerate() {
            let highest = highest_row[x];
            if *tree as i32 > highest {
                visible_map[y][x] = true;
                highest_row[x] = *tree as i32;
            }
        }
    }

    // println!("{visible_map:#?}");

    let count: u32 = visible_map
        .into_iter()
        .map(|row| row.into_iter().map(|b| if b { 1 } else { 0 }).sum::<u32>())
        .sum();

    format!("{count}")
}

fn find_visible_up(map: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let tree = map[y][x];
    let mut sum = 0;
    for val in (0..y).rev() {
        sum = sum + 1;
        if map[val][x] >= tree {
            return sum;
        }
    }

    return sum;
}

fn find_visible_down(map: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let tree = map[y][x];
    let mut sum = 0;
    for val in (y..map.len()).skip(1) {
        sum = sum + 1;
        if map[val][x] >= tree {
            return sum;
        }
    }

    return sum;
}

fn find_visible_right(map: &Vec<Vec<u8>>, x: usize, y: usize, width: usize) -> u32 {
    let tree = map[y][x];
    let mut sum = 0;
    for val in (x..width).skip(1) {
        sum = sum + 1;
        if map[y][val] >= tree {
            return sum;
        }
    }

    return sum;
}

fn find_visible_left(map: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let tree = map[y][x];
    let mut sum = 0;
    for val in (0..x).rev() {
        sum = sum + 1;
        if map[y][val] >= tree {
            return sum;
        }
    }

    return sum;
}

pub fn solve_part_two<'a>(input: (Vec<Vec<u8>>, usize)) -> String {
    let (input, width) = input;

    let mut highest_scenic = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let visible_up = find_visible_up(&input, x, y);
            let visible_down = find_visible_down(&input, x, y);
            let visible_right = find_visible_right(&input, x, y, width);
            let visible_left = find_visible_left(&input, x, y);
            // println!("({x},{y}) :: {visible_up} {visible_down} {visible_right} {visible_left}");

            let scenic = visible_up * visible_down * visible_right * visible_left;
            if scenic > highest_scenic {
                highest_scenic = scenic;
            }
        }
    }

    format!("{highest_scenic}")
}
