use std::io::{self, Read};

fn main() {
    let mut line = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut line)
        .expect("Couldnt read from stdin");

    let mut it = line.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let a: Vec<u32> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();

    let max_val = *a.iter().max().unwrap();
    let min_val = *a.iter().min().unwrap();

    let max_i = a.iter().position(|&x| x == max_val).unwrap();
    let min_i = a.iter().rposition(|&x| x == min_val).unwrap();

    let mut moves = max_i + (n - 1 - min_i);
    if max_i > min_i {
        moves -= 1;
    }
    println!("{}", moves);
}
