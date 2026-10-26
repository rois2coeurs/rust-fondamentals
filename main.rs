use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let a = line.trim_end_matches(|c| c == '\r' || c == '\n').trim();
    let a: i64 = a.trim().parse().unwrap();


    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let b = line.trim_end_matches(|c| c == '\r' || c == '\n').trim();
    let b: i64 = b.trim().parse().unwrap();

    println!("{}", square(a));
    println!("{}", square(b));
    println!("{}", sum_of_square(a, b));
}

fn square(n: i64) -> i64 {
    n*n
}

fn sum_of_square(a: i64, b: i64) -> i64 {
    square(a) + square(b)
}
