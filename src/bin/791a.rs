use std::io;

fn solver(mut a: i32, mut b: i32) -> i32 {
    let mut years = 0;
    while a <= b {
        a *= 3;
        b *= 2;
        years += 1;
    }
    years
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let mut it = line.trim().split_ascii_whitespace();
    let a = it.next().unwrap().parse::<i32>().unwrap();
    let b = it.next().unwrap().parse::<i32>().unwrap();
    println!("{}", solver(a, b));
}
