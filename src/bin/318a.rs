use std::io;

/*
 * 10 3
 * 1, 3, 5, 7, 9, 2, 4, 6, 8, 10
 * Output: 5
 * */

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    let mut iterator = line.split_ascii_whitespace();

    let n: i128 = iterator.next().unwrap().parse().unwrap();
    let k: i128 = iterator.next().unwrap().parse().unwrap();

    let odds_end = (n + 1) / 2;

    let ans = if k <= odds_end {
        2 * k - 1
    } else {
        2 * (k - odds_end)
    };

    println!("{}", ans);
}
