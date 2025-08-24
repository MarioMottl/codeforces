use std::io;

fn solver(n: i32) -> &'static str {
    let lucky = [4, 7, 44, 47, 74, 77, 444, 447, 474, 477, 744, 747, 774, 777];
    if lucky.iter().any(|&x| n % x == 0) {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let n = line.trim().parse::<i32>().unwrap();

    println!("{}", solver(n));
}
