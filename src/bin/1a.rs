use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let mut it = line.split_ascii_whitespace();

    let n: u128 = it.next().unwrap().parse().unwrap();
    let m: u128 = it.next().unwrap().parse().unwrap();
    let a: u128 = it.next().unwrap().parse().unwrap();

    let tiles_n = (n + a - 1) / a;
    let tiles_m = (m + a - 1) / a;

    println!("{}", tiles_m * tiles_n);
}
