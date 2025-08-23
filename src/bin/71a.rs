use std::io;

fn solver(inputs: Vec<String>) {
    dbg!(&inputs);
    for word in inputs {
        if word.len() > 10 {
            let chars: Vec<char> = word.chars().collect();
            let first = chars.first().expect("Couldnt get first");
            let last = chars.last().expect("Couldnt get second");
            println!("{}{}{}", first, word.len() - 2, last);
        } else {
            println!("{word}");
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .expect("Couldnt read from stdin");

    let n: i32 = buffer.trim().parse().expect("Input not an integer");
    let mut inputs: Vec<String> = Vec::new();

    for _ in 0..n {
        let mut buffer = String::new();
        buffer.clear();
        stdin
            .read_line(&mut buffer)
            .expect("Couldnt read from stdin");

        inputs.push(buffer.trim().to_string());
    }

    solver(inputs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let inputs: Vec<String> = vec![
            "word".to_string(),
            "localization".to_string(),
            "internationalization".to_string(),
            "pneumonoultramicroscopicsilicovolcanoconiosis".to_string(),
        ];
        solver(inputs);
    }
}
