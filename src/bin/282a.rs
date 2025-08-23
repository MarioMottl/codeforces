use std::io;

fn solver(statements: Vec<String>) {
    let mut x = 0;

    for stmt in statements {
        if stmt.contains("++") {
            x += 1;
        } else if stmt.contains("--") {
            x -= 1;
        }
    }

    println!("{}", x);
}

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();
    stdin.read_line(&mut line).expect("failed to read n");
    let n: usize = line.trim().parse().expect("n is not an integer");

    let mut statements = Vec::with_capacity(n);

    for _ in 0..n {
        line.clear();
        stdin.read_line(&mut line).expect("failed to read line");
        statements.push(line.trim().to_string());
    }

    solver(statements);
}
