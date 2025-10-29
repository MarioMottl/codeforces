use std::io::{self};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    /*
     * Bills
     *   > 1
     *   > 5
     *   > 10
     *   > 20
     *   > 100
     * */

    let mut money = line.trim().parse::<u32>().unwrap();
    let mut sum = 0;

    for bill in [1, 5, 10, 20, 100].into_iter().rev() {
        let amount = money / bill;
        money -= amount * bill;

        sum += amount;
    }

    println!("{sum}");
}
