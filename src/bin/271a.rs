use std::io;

fn solver(mut year: i32) -> i32 {
    use std::collections::HashSet;
    loop {
        year += 1;
        let s = year.to_string();
        let set: HashSet<_> = s.chars().collect();

        if set.len() == s.len() {
            return year;
        }
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let year = line.trim().parse::<i32>().unwrap();

    println!("{}", solver(year))
}
