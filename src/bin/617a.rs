use std::io;

fn solver(x: i32) -> i32 {
    (x + 4) / 5
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let x = line.trim().parse::<i32>().expect("Couldnt parse x");
    println!("{}", solver(x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {}
}
