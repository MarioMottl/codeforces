use std::io;

fn solver(s1: String, s2: String) -> i32 {
    let a = s1.to_ascii_lowercase();
    let b = s2.to_ascii_lowercase();

    match a.cmp(&b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    let s1 = line.clone();
    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    let s2 = line.clone();

    println!("{}", solver(s1, s2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s1 = "aaaa".to_string();
        let s2 = "aaaA".to_string();
        assert_eq!(solver(s1, s2), 0);
    }

    #[test]
    fn example2() {
        let s1 = "abs".to_string();
        let s2 = "Abz".to_string();
        assert_eq!(solver(s1, s2), -1);
    }

    #[test]
    fn example3() {
        let s1 = "abcdefg".to_string();
        let s2 = "AbCdEfF".to_string();
        assert_eq!(solver(s1, s2), 1);
    }
}
