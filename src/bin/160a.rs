use std::io;

fn solver(mut a: Vec<i32>) -> i32 {
    a.sort_unstable_by(|x, y| y.cmp(x));
    let total: i32 = a.iter().sum();
    let mut sum = 0;

    for (index, &x) in a.iter().enumerate() {
        sum += x;
        if sum > total - sum {
            return (index + 1) as i32;
        }
    }
    a.len() as i32
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let _n = line.trim().parse::<i32>().unwrap();

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let coin_values = line
        .split_whitespace()
        .map(|tok| tok.parse::<i32>().expect("token is not i32"))
        .collect::<Vec<i32>>();

    println!("{}", solver(coin_values));
}
