use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let age = line.trim_end_matches(|c| c == '\r' || c == '\n').trim();
    let age: i32 = age.trim().parse().unwrap();

    let mut yes = false;
    if age % 3 == 0 {
        print!("Fizz");
        yes = true;
    }
    if age % 5 == 0 {
        print!("Buzz");
        yes = true;
    }
    if !yes {
        print!("{}", age);
    }
    println!("");
}
