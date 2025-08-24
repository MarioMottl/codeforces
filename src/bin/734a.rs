use std::io;

fn solver(input: &str) -> &'static str {
    let anton = input.chars().filter(|&c| c == 'A').count();
    let danik = input.len() - anton;

    if anton > danik {
        "Anton"
    } else if danik > anton {
        "Danik"
    } else {
        "Friendship"
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let _n = line.trim().parse::<i32>().unwrap();

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    let input = line.trim();

    println!("{}", solver(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {}
}
