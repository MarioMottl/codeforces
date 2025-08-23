use std::io;

fn solver(username: String) -> &'static str {
    use std::collections::HashSet;

    let distinct = username.chars().collect::<HashSet<_>>().len();
    dbg!(distinct);

    if distinct % 2 == 0 {
        "CHAT WITH HER!"
    } else {
        "IGNORE HIM!"
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    println!("{}", solver(line.trim().to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {}
}
