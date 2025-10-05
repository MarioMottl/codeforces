use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let ok = line.chars().any(|c| c == 'H' || c == 'Q' || c == '9');

    println!("{}", if ok { "YES" } else { "NO" })
}
