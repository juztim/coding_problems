use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    println!("{}", capitalize(&*buffer));
}

fn capitalize(word: &str) -> String {
    let mut up = word.to_string();
    let uppercase_char = word
        .chars()
        .nth(0)
        .unwrap()
        .to_uppercase()
        .collect::<String>();
    up.replace_range(0..1, &*uppercase_char);
    up
}
