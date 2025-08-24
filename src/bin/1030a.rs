use std::io;

fn solver(a: &[i32]) -> &'static str {
    if a.iter().any(|&x| x == 1) {
        "HARD"
    } else {
        "EASY"
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let _n = line.trim().parse::<i32>().unwrap();

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let row = line
        .split_whitespace()
        .map(|tok| tok.parse::<i32>().expect("token is not i32"))
        .collect::<Vec<i32>>();

    println!("{}", solver(&row));
}
