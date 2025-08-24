use std::io;

fn solver(s: &str, t: i32) -> String {
    let mut c = s.chars().collect::<Vec<char>>();

    for _ in 0..t {
        let mut i = 0;
        while i + 1 < c.len() {
            if c[i] == 'B' && c[i + 1] == 'G' {
                c.swap(i, i + 1);
                i += 2;
            } else {
                i += 1;
            }
        }
    }
    c.into_iter().collect()
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let mut it = line.trim().split_ascii_whitespace();

    let _n = it.next().unwrap().parse::<i32>().unwrap();
    let t = it.next().unwrap().parse::<i32>().unwrap();

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let input = line.trim();

    println!("{}", solver(input, t))
}
