use std::{io, ops::Div};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let mut it = line.split_ascii_whitespace();

    let m: u32 = it.next().unwrap().parse().unwrap();
    let n: u32 = it.next().unwrap().parse().unwrap();
    let dominos: u32 = (m * n).div(2);

    println!("{}", dominos);
}
