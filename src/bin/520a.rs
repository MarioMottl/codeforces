use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut line = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut line)
        .expect("Couldnt read from stdin");

    let input = line.lines().nth(1).unwrap().to_lowercase();

    let set: HashSet<char> = input.chars().filter(|c| c.is_ascii_lowercase()).collect();

    println!("{}", if set.len() == 26 { "YES" } else { "NO" })
}
