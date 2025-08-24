use std::io;

fn solver(p: &[i32]) -> Vec<usize> {
    let mut res = vec![0; p.len()];

    for (i, &x) in p.iter().enumerate() {
        res[x as usize - 1] = i + 1;
    }
    res
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let _n: i32 = line.trim().parse().unwrap();

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let presents = line
        .split_whitespace()
        .map(|tok| tok.parse::<i32>().expect("token is not i32"))
        .collect::<Vec<i32>>();

    let mut output = String::new();
    for elem in solver(&presents) {
        output.push_str(&elem.to_string());
        output.push_str(" ");
    }
    println!("{}", output.trim());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {}
}
