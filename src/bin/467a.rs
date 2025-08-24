use std::io;

fn solver(rooms: &[(i32, i32)]) -> i32 {
    rooms.iter().filter(|&&(p, q)| q - p >= 2).count() as i32
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let n = line.trim().parse::<i32>().unwrap();

    let mut rooms: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        line.clear();
        stdin.read_line(&mut line).expect("Couldnt read from stdin");
        let nums = line
            .trim()
            .split_ascii_whitespace()
            .map(|tok| tok.parse::<i32>().expect("Couldnt parse"))
            .collect::<Vec<i32>>();

        if nums.len() == 2 {
            rooms.push((nums[0], nums[1]));
        } else {
            panic!("More than 3 numbers in line");
        }
    }

    println!("{}", solver(&rooms));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let rooms = vec![(1, 1), (2, 2), (3, 3)];

        assert_eq!(solver(&rooms), 0);
    }

    #[test]
    fn example2() {
        let rooms = vec![(1, 10), (0, 10), (10, 10)];

        assert_eq!(solver(&rooms), 2);
    }
}
