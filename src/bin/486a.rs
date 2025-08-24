use std::io;

fn solver(n: i64) -> i64 {
    /*
     *  f(4) =  2
     *  f(5) = -3
     * */

    if n % 2 == 0 { n / 2 } else { -(n + 1) / 2 }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let n: i64 = line.trim().parse().unwrap();

    println!("{}", solver(n));
}
