use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let mut it = line.split_ascii_whitespace();

    let _n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let mut participants: Vec<u32> = Vec::new();

    line.clear();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    for it in line.split_ascii_whitespace() {
        participants.push(it.parse().expect("Couldnt parse &str into u32"));
    }
    let threshold = participants[k - 1];

    let advancers = participants
        .into_iter()
        .filter(|&x| x > 0 && x >= threshold)
        .count();
    println!("{}", advancers);
}

#[cfg(test)]
mod tests {
    use super::*;
}
