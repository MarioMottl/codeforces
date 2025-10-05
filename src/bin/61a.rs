use std::io;

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    let a = line.trim().as_bytes();

    let mut line = String::new();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    let b = line.trim().as_bytes();

    let out: String = a
        .iter()
        .zip(b)
        .map(|(x, y)| if x == y { "0" } else { "1" })
        .collect();

    println!("{}", out);
}
