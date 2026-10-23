use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let first = line.trim_end_matches(|c| c == '\r' || c == '\n');
    let first: i64 = first.trim().parse().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let second = line.trim_end_matches(|c| c == '\r' || c == '\n');
    let second: i64 = second.trim().parse().unwrap();

    let mut result = first;
    println!("{}", result);
    result += second;
    println!("{}", result);
}
