use std::io;

fn solver(h: i32, a: &[i32]) -> i32 {
    a.iter().map(|&x| if x > h { 2 } else { 1 }).sum()
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let mut it = line.trim().split_ascii_whitespace();

    let _n = it.next().unwrap().parse::<i32>().unwrap();
    let h = it.next().unwrap().parse::<i32>().unwrap();

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let row = line
        .split_whitespace()
        .map(|tok| tok.parse::<i32>().expect("token is not i32"))
        .collect::<Vec<i32>>();

    println!("{}", solver(h, &row));
}
