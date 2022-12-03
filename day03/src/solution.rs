pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = &str> + 'a {
    input.lines()
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = &'a str>) -> String {
    let sum: u32 = input
        .map(|l| l.split_at(l.len() / 2))
        .map(|(first, second)| {
            for char_1 in first.as_bytes().into_iter() {
                for char_2 in second.as_bytes().into_iter() {
                    if char_1 == char_2 {
                        return char_1;
                    }
                }
            }
            panic!("Failed to find matching chars for strings {first} and {second}")
        })
        .map(|char| match *char {
            c if c < 97 => c as u32 - 65 + 27,
            c => c as u32 - 97 + 1,
        })
        .sum();

    format!("{sum}")
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = &'a str>) -> String {
    let input: Vec<&str> = input.collect();
    let mut sum = 0;
    for group_base_index in (0..input.len()).step_by(3) {
        let first = input[group_base_index];
        let second = input[group_base_index + 1];
        let third = input[group_base_index + 2];

        let mut group_char = 0;
        for first_char in first.as_bytes().into_iter() {
            for second_char in second.as_bytes().into_iter() {
                for third_char in third.as_bytes().into_iter() {
                    if first_char == second_char && second_char == third_char {
                        group_char = *first_char;
                    }
                }
            }
        }

        sum = sum
            + match group_char {
                c if c < 97 => c as u32 - 65 + 27,
                c => c as u32 - 97 + 1,
            }
    }

    format!("{sum}")
}
