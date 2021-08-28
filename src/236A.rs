use std::collections::HashSet;
use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    println!("{}", solve(buffer.trim_end().parse().unwrap()));
}

fn solve(input: String) -> String {
    if input.chars().collect::<HashSet<char>>().len() % 2 != 0 {
        String::from("IGNORE HIM!")
    } else {
        String::from("CHAT WITH HER!")
    }
}
