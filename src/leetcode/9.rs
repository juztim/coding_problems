use std::io;

fn main() {
    println!("{}", is_palindrome(-121));
}

pub fn is_palindrome(num: i32) -> bool {
    let palindrome = reverse(num);
    match palindrome.is_negative() {
        true => false,
        false => palindrome == num,
    }
}

pub fn reverse(mut num: i32) -> i32 {
    let negative = num < 0;
    let rev = str::parse(&*num.abs().to_string().chars().rev().collect::<String>()).unwrap_or(0);
    if negative {
        return rev * -1;
    }
    rev
}
