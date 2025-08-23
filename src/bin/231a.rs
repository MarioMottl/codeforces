use std::io;

fn solver(inputs: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for line in inputs {
        if line.iter().sum::<i32>() >= 2 {
            count += 1;
        }
    }
    println!("{count}");
    count
}

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();
    stdin.read_line(&mut line).expect("failed to read n");
    let n: usize = line.trim().parse().expect("n is not an integer");

    let mut inputs: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        line.clear();
        stdin.read_line(&mut line).expect("failed to read line");
        if line.trim().is_empty() {
            inputs.push(Vec::new());
            continue;
        }
        let row = line
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().expect("token is not i32"))
            .collect::<Vec<i32>>();
        inputs.push(row);
    }
    let _ = solver(inputs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let inputs = vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 0, 0]];
        assert_eq!(solver(inputs), 2);
    }

    #[test]
    fn example2() {
        let inputs = vec![vec![1, 0, 0], vec![0, 1, 1]];
        assert_eq!(solver(inputs), 1);
    }
}
