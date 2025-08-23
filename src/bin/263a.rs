use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();

    let mut inputs: Vec<Vec<i32>> = Vec::with_capacity(5);
    for _ in 0..5 {
        line.clear();
        stdin.read_line(&mut line).expect("failed to read line");

        let row = line
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().expect("token is not i32"))
            .collect::<Vec<i32>>();
        inputs.push(row);
    }

    for i in 0..5 {
        for j in 0..5 {
            if inputs[i][j] == 1 {
                let moves = (i as i32 - 2).abs() + (j as i32 - 2).abs();
                println!("{}", moves);
                return;
            }
        }
    }
}
