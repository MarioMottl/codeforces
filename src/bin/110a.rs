use std::io;

fn solver(s: &str) -> &'static str {
    let cnt = s.chars().filter(|&c| c == '4' || c == '7').count();
    if cnt == 4 || cnt == 7 { "YES" } else { "NO" }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    println!("{}", solver(line.trim()));
}
