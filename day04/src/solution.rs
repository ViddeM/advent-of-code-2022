pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = ((u32, u32), (u32, u32))> + 'a {
    input.lines().map(|l| {
        let (elf_one, elf_two) = l.split_once(",").unwrap();
        let (elf_one_low, elf_one_high) = elf_one.split_once("-").unwrap();
        let (elf_two_low, elf_two_high) = elf_two.split_once("-").unwrap();

        (
            (
                u32::from_str_radix(elf_one_low, 10).unwrap(),
                u32::from_str_radix(elf_one_high, 10).unwrap(),
            ),
            (
                u32::from_str_radix(elf_two_low, 10).unwrap(),
                u32::from_str_radix(elf_two_high, 10).unwrap(),
            ),
        )
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = ((u32, u32), (u32, u32))>) -> String {
    let sum: u32 = input
        .map(|((l1, h1), (l2, h2))| {
            if (l1 <= l2 && h1 >= h2) || (l2 <= l1 && h2 >= h1) {
                1
            } else {
                0
            }
        })
        .sum();

    format!("{sum}")
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = ((u32, u32), (u32, u32))>) -> String {
    let sum: u32 = input
        .map(|((l1, h1), (l2, h2))| {
            if (l1 <= l2 && h1 >= l2) || (l2 <= l1 && h2 >= l1) {
                1
            } else {
                0
            }
        })
        .sum();

    format!("{sum}")
}
