pub fn parse(input: &str) -> Vec<Option<u32>> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(u32::from_str_radix(l, 10).unwrap())
            }
        })
        .collect::<Vec<Option<u32>>>()
}

pub fn solve_part_one(input: Vec<Option<u32>>) -> String {
    let mut curr = 0;
    let mut highest_elf = 0;
    for calories in input.into_iter() {
        match calories {
            None => {
                if curr > highest_elf {
                    highest_elf = curr;
                }
                curr = 0;
            }
            Some(n) => {
                curr += n;
            }
        }
    }
    if curr > highest_elf {
        highest_elf = curr;
    }

    format!("{}", highest_elf)
}

pub fn solve_part_two(input: Vec<Option<u32>>) -> String {
    let mut elf_calories = Vec::new();
    let mut curr = 0;
    for calories in input.into_iter() {
        match calories {
            None => {
                elf_calories.push(curr);
                curr = 0;
            }
            Some(n) => {
                curr += n;
            }
        }
    }
    elf_calories.push(curr);

    elf_calories.sort();
    elf_calories.reverse();

    let total = elf_calories[0] + elf_calories[1] + elf_calories[2];

    format!("{}", total)
}
