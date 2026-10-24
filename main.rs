use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let input = line.trim_end_matches(|c| c == '\r' || c == '\n').trim();

    println!("{}", input.to_uppercase());
    println!("{}",input.chars().count());
    println!("{}",input.len());
}
