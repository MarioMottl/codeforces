use std::io::{self, Read};

fn main() {
    let mut line = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut line)
        .expect("Couldnt read from stdin");

    let mut it = line.split_whitespace();

    let _n: i32 = it.next().unwrap().parse().unwrap();

    let mut constellation: Vec<u32> = it.map(|x| x.parse().unwrap()).collect();
    constellation.sort();

    let mut ans = String::new();

    for (i, v) in constellation.iter().enumerate() {
        if i > 0 {
            ans += " ";
        }
        ans += &v.to_string();
    }

    println!("{}", ans);
}
