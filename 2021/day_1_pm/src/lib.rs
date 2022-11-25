extern crate proc_macro;

use std::fs;
use quote::quote;

use proc_macro::TokenStream;

#[proc_macro]
pub fn solve_part_1(_arg: TokenStream) -> TokenStream {
    let file_inp = fs::read_to_string("input.txt").expect("FUCK");
    let nums = file_inp.split('\n')
        .map(|l| {
            l.parse::<u32>().expect("Failed to parse line {l}")
        })
        .collect::<Vec<u32>>();

    let mut count = 0;
    let mut prev = u32::MAX;
    for a in nums {
        if a > prev {
            count = count + 1;
        }
        prev = a;
    }

    let answer = format!("Answer is {count}");

    TokenStream::from(quote!{
        const answer: &str = #answer;
    })
}