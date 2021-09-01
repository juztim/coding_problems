use std::io;

fn main() {
    println!("{}", reverse(-123456789));
}

pub fn reverse(mut num: i32) -> i32 {
    let negative = num < 0;
    let rev = str::parse(&*num.abs().to_string().chars().rev().collect::<String>()).unwrap_or(0);
    if negative {
        return rev * -1;
    }
    rev
}
