use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut input = line.trim_end_matches(|c| c == '\r' || c == '\n').trim().to_string();

    println!("{}", vowel_count(&input));
    make_loud(&mut input);
    println!("{}", input);
    println!("{}", vowel_count(&input));
}
fn vowel_count(s: &str) -> usize {
    let vowels: String = String::from("aeiou"); 
    let mut n = 0;
    for c in s.to_lowercase().chars() {
        for vowel in vowels.chars() {
            if c == vowel { n += 1; }
        }
    }
    n
}

fn make_loud(s: &mut String) {
    *s = s.to_uppercase();
    if s.chars().last() != Some('!') {
        s.push('!');
    }
}
