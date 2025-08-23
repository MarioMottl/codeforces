use std::io;

fn solver(coords: Vec<(i32, i32, i32)>) -> &'static str {
    let (mut sx, mut sy, mut sz) = (0, 0, 0);
    for &(x, y, z) in &coords {
        sx += x;
        sy += y;
        sz += z;
    }
    if sx == 0 && sy == 0 && sz == 0 {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let n = line.trim().parse::<i32>().unwrap();
    let mut coords: Vec<(i32, i32, i32)> = Vec::new();

    for _ in 0..n {
        line.clear();
        stdin.read_line(&mut line).expect("Couldnt read from stdin");
        let nums = line
            .trim()
            .split_ascii_whitespace()
            .map(|tok| tok.parse::<i32>().expect("Couldnt parse"))
            .collect::<Vec<i32>>();

        if nums.len() == 3 {
            coords.push((nums[0], nums[1], nums[2]));
        } else {
            panic!("More than 3 numbers in line");
        }
    }
    println!("{}", solver(coords));
}
