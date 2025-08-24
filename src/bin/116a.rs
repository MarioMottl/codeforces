use std::io;

fn solver(changes: &[(i32, i32)]) -> i32 {
    let mut curr = 0;
    let mut max_cap = 0;

    for &(leave, enter) in changes {
        curr = curr - leave + enter;
        max_cap = max_cap.max(curr)
    }
    max_cap
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let n = line.trim().parse::<i32>().unwrap();

    let mut changes: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        line.clear();
        stdin.read_line(&mut line).expect("Couldnt read from stdin");

        let row = line
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().expect("token is not i32"))
            .collect::<Vec<i32>>();

        if row.len() == 2 {
            changes.push((row[0], row[1]));
        }
    }

    println!("{}", solver(&changes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let changes = vec![(0, 3), (2, 5), (4, 2), (4, 0)];
        assert_eq!(solver(&changes), 6);
    }
}
