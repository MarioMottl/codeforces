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

    let mut it = line.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut have = HashSet::new();

    for _ in 0..2 {
        let c: usize = it.next().unwrap().parse().unwrap();
        for _ in 0..c {
            have.insert(it.next().unwrap().parse::<usize>().unwrap());
        }
    }

    println!(
        "{}",
        if have.len() == n {
            "I become the guy."
        } else {
            "Oh, my keyboard!"
        }
    )
}
