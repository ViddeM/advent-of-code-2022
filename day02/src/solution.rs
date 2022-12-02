use std::str::Bytes;

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = (u8, u8)> + 'a {
    input.lines().map(|l| {
        let mut a = l.bytes().into_iter();
        (a.next().unwrap(), a.skip(1).next().unwrap())
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = (u8, u8)>) -> String {
    let val: u32 = input
        .map(|bs| match bs {
            (0x41, 0x58) => 1 + 3, // A, X
            (0x42, 0x58) => 1 + 0, // B, X
            (0x43, 0x58) => 1 + 6, // C, X
            (0x41, 0x59) => 2 + 6, // A, Y
            (0x42, 0x59) => 2 + 3, // B, Y
            (0x43, 0x59) => 2 + 0, // C, Y
            (0x41, 0x5A) => 3 + 0, // A, Z
            (0x42, 0x5A) => 3 + 6, // B, Z
            (0x43, 0x5A) => 3 + 3, // C, Z
            (a, b) => panic!("Pattern ({a}, {b}) is not supported!"),
        } as u32)
        .sum();

    format!("{val}")
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = (u8, u8)>) -> String {
    let val: u32 = input
        .map(|(a, b)| match (a, b) {
            (0x41, 0x58) => 3 + 0, // A, X
            (0x42, 0x58) => 1 + 0, // B, X
            (0x43, 0x58) => 2 + 0, // C, X
            (0x41, 0x59) => 1 + 3, // A, Y
            (0x42, 0x59) => 2 + 3, // B, Y
            (0x43, 0x59) => 3 + 3, // C, Y
            (0x41, 0x5A) => 2 + 6, // A, Z
            (0x42, 0x5A) => 3 + 6, // B, Z
            (0x43, 0x5A) => 1 + 6, // C, Z
            (a, b) => panic!("Pattern ({a}, {b}) is not supported!"),
        } as u32)
        .sum();

    format!("{val}")
}
