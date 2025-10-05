use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut input)
        .expect("Couldnt read from stdin");

    let mut it = input.split_whitespace();

    let t: i64 = it.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();

        let r = a % b;
        println!("{}", if r == 0 { 0 } else { b - r })
    }
}
