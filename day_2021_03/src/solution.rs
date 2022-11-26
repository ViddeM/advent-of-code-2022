const BIT_LEN: usize = 12;
const BIT_MASK: usize = 0b111111111111;

pub fn solve_part_one(input: &str) -> String {
    let parsed: Vec<i32> = input
        .lines()
        .into_iter()
        .map(|l| i32::from_str_radix(l.trim(), 2).expect("Failed to parse number"))
        .collect();

    let mut bits = [0i32; BIT_LEN];

    for num in parsed.into_iter() {
        for bit in 0..BIT_LEN {
            let the_bit = 1 << (bit as i32);
            let bit_val = if num & the_bit > 0 { 1 } else { -1 };
            bits[bit] = bits[bit] + bit_val;
        }
    }

    let mut gamma_rate = 0;
    for index in 0..BIT_LEN {
        let bit = bits[index];
        let gamma_bit_val = if bit > 0 { 1 << index } else { 0 };
        gamma_rate = gamma_rate + gamma_bit_val;
    }

    let epsilon_rate = gamma_rate ^ BIT_MASK;

    (gamma_rate * epsilon_rate).to_string()
}

pub fn solve_part_two(input: &str) -> String {
    todo!("Part two is not yet implemented");
}
