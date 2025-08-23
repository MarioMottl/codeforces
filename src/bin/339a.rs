use std::io;

fn solver(mut nums: Vec<i32>) -> String {
    nums.sort_unstable();
    let output = nums
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("+");
    println!("{output}");
    output
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");

    let nums = line
        .trim()
        .split("+")
        .map(|tok| tok.parse::<i32>().expect("Couldnt parse"))
        .collect::<Vec<i32>>();

    let _ = solver(nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![3, 2, 1];
        assert_eq!(solver(nums), "1+2+3".to_string())
    }
}
