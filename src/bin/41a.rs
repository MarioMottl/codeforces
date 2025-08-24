use std::io;

fn solver(s: &str, t: &str) -> &'static str {
    if s.chars().rev().collect::<String>() == t {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let s = line.clone();
    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    let t = line.trim();

    println!("{}", solver(s.trim(), t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {}
}
