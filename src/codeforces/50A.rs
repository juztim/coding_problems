use std::convert::TryInto;
use std::{io, process};

fn main() {
    // Input 1
    let mut row_one_buffer = String::new();
    io::stdin()
        .read_line(&mut row_one_buffer)
        .expect("error reading line");
    let one_buffer = row_one_buffer.trim();
    // Input 2
    let mut row_two_buffer = String::new();
    io::stdin()
        .read_line(&mut row_two_buffer)
        .expect("error reading line");
    let two_buffer = row_two_buffer.trim();
    let mut in_nums: Vec<&str> = row_one_buffer.split_whitespace().collect();
    let i: usize = in_nums[0].parse().unwrap();
    let k: usize = in_nums[1].parse().unwrap();
    let input = Input { i, k };
    let mut second_vec: Vec<_> = two_buffer.chars().flat_map(|c| c.to_digit(10)).collect();
    solve(input, second_vec);
}

struct Input {
    i: usize,
    k: usize,
}

fn solve(first: Input, mut second: Vec<u32>) -> bool {
    let mut count = 0;
    for i in 0..second.len() {
        if i >= first.k && i >= 1 {
            count += 1;
        }
    }
    println!("{}", count);
    true
}
