use std::io;

fn solver(s: &str) -> &'static str {
    let target = ['h', 'e', 'l', 'l', 'o'];
    let mut it = s.chars();
    for &c in &target {
        if it.find(|&x| x == c).is_none() {
            return "NO";
        }
    }
    "YES"
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    println!("{}", solver(line.trim()));
}
