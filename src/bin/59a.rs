use std::io;

fn solver(word: &str) -> String {
    let mut upper = 0;
    let mut lower = 0;

    for c in word.chars() {
        if c.is_ascii_uppercase() {
            upper += 1;
        } else {
            lower += 1;
        }
    }
    if upper > lower {
        word.to_ascii_uppercase()
    } else {
        word.to_ascii_lowercase()
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    println!("{}", solver(line.trim()));
}
