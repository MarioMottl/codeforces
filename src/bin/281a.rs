use std::io;

fn solver(word: String) -> String {
    let mut it = word.chars();
    match it.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + &it.collect::<String>(),
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    println!("{}", solver(line.trim().to_string()))
}
