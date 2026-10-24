use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let name = line.trim_end_matches(|c| c == '\r' || c == '\n').trim();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let age = line.trim_end_matches(|c| c == '\r' || c == '\n').trim();
    let age: i32 = age.trim().parse().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let height = line.trim_end_matches(|c| c == '\r' || c == '\n').trim();
    let height: f32 = height.trim().parse().unwrap();

    println!("[{:10}]", name);
    println!("Age: {:03}", age);
    println!("Height: {:.2} m", height);
}
