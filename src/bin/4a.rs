use std::io;

fn solver(input: String) -> String {
    let x: i32 = input.trim().parse().expect("Input not an integer");
    if x >= 4 && x % 2 == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .expect("Couldnt read from stdin");
    println!("{}", solver(buffer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "8".to_string();
        assert_eq!(solver(input), "YES".to_string())
    }
}
