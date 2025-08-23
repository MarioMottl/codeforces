use std::io;

fn solver(mut n: i64, mut k: i64) -> i64 {
    while k > 0 {
        if n % 10 == 0 {
            n /= 10;
        } else {
            n -= 1;
        }
        k -= 1;
    }
    n
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let mut it = line.trim().split_ascii_whitespace();
    let n = it.next().unwrap().parse::<i64>().unwrap();
    let k = it.next().unwrap().parse::<i64>().unwrap();

    println!("{}", solver(n, k));
}
