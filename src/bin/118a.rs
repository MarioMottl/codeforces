use std::io;

fn solver(word: &str) -> String {
    let vowels = ['a', 'o', 'y', 'e', 'u', 'i'];
    let mut out = String::new();

    for ch in word.chars() {
        let c = ch.to_ascii_lowercase();
        if !vowels.contains(&c) {
            out.push('.');
            out.push(c);
        }
    }
    out
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    println!("{}", solver(line.trim()));
}
