use std::io;

fn solver(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut cnt = 0;
    for index in 1..bytes.len() {
        if bytes[index] == bytes[index - 1] {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let _n = line
        .trim()
        .parse::<i32>()
        .expect("Couldnt convert n to i32");

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    println!("{}", solver(line.trim()));
}
