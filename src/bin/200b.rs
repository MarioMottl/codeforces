use std::io;

fn solver(p: &[f64]) -> f64 {
    p.iter().sum::<f64>() / p.len() as f64
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let _n: i32 = line.trim().parse().unwrap();

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let percentage = line
        .split_whitespace()
        .map(|tok| tok.parse::<f64>().expect("token is not i32"))
        .collect::<Vec<f64>>();

    println!("{}", solver(&percentage));
}
