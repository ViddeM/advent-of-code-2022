pub fn parse<'a>(input: &'a str) -> &[u8] {
    input.as_bytes()
}

#[inline(always)]
fn find_packets(input: &[u8], window_size: usize) -> u32 {
    'outer: for (index, arr) in input.windows(window_size).enumerate() {
        for (i1, a) in arr.iter().enumerate() {
            for (i2, b) in arr.iter().enumerate() {
                if i1 != i2 && a == b {
                    continue 'outer;
                }
            }
        }
        return (index + window_size) as u32;
    }
    panic!("Failed to find answer");
}

pub fn solve_part_one<'a>(input: &[u8]) -> String {
    format!("{}", find_packets(input, 4))
}

pub fn solve_part_two<'a>(input: &[u8]) -> String {
    format!("{}", find_packets(input, 14))
}
