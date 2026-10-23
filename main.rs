use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let width = line.trim_end_matches(|c| c == '\r' || c == '\n');
    let width: i64 = width.trim().parse().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let height = line.trim_end_matches(|c| c == '\r' || c == '\n');
    let height: i64 = height.trim().parse().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let num_people = line.trim_end_matches(|c| c == '\r' || c == '\n');
    let num_people: i64 = num_people.trim().parse().unwrap();

    let area = width * height;
    let square_people = area / num_people;
    let rest = area % num_people;
    println!("{}", area);
    println!("{}", square_people);
    println!("{}", rest);
}
