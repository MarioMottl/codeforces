use std::io;

fn solver(k: i64, n: i64, w: i64) -> i64 {
    let total = k * w * (w + 1) / 2;
    (total - n).max(0)
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let mut it = line.trim().split_ascii_whitespace();
    let k = it.next().unwrap().parse::<i64>().unwrap();
    let n = it.next().unwrap().parse::<i64>().unwrap();
    let w = it.next().unwrap().parse::<i64>().unwrap();

    println!("{}", solver(k, n, w));
}
