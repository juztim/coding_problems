use std::io;

fn main() {
    let mut buffer_one = String::new();
    io::stdin()
        .read_line(&mut buffer_one)
        .expect("Failed to read line");

    let mut buffer_two = String::new();
    io::stdin()
        .read_line(&mut buffer_two)
        .expect("Failed to read line");

    solve(buffer_one.trim_end().parse::<i32>().unwrap(), buffer_two);
}

fn solve(stones: i32, colors: String) {
    let mut remove: i32 = 0;
    let mut last: char = '-';

    for c in colors.chars() {
        if c == last {
            remove += 1;
        }
        last = c;
    }
    println!("{}", remove);
}
