use std::{collections::HashSet, io};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let set: HashSet<_> = line.split_ascii_whitespace().collect();
    println!("{}", 4 - set.len())
}
