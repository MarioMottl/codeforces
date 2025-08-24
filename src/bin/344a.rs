use std::io;

fn solver(m: &[String]) -> i32 {
    let mut cnt = 1;
    for i in 1..m.len() {
        if m[i] != m[i - 1] {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let n = line.trim().parse::<i32>().unwrap();

    let mut magnets: Vec<String> = Vec::new();

    for _ in 0..n {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("Couldnt read from stdin");
        magnets.push(line.clone());
    }

    println!("{}", solver(&magnets));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let m = vec![
            "10".to_string(),
            "10".to_string(),
            "10".to_string(),
            "01".to_string(),
            "10".to_string(),
            "10".to_string(),
        ];
        assert_eq!(solver(&m), 3);
    }
}
