use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let input = line.trim_end_matches(|c| c == '\r' || c == '\n').trim();
    let input: i64 = input.trim().parse().unwrap();

    let mut count: i64 = 0;
    for i in 0..=input {
        count += i;
    }

    println!("{}", count);
}
