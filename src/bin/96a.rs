fn football(input: String) -> String {
    let mut a = 0;
    let mut b = 0;

    for char in input.chars() {
        if char == '0' {
            b = 0;
            a += 1;
        } else {
            a = 0;
            b += 1;
        }
        if a == 7 || b == 7 {
            return "YES".to_string();
        }
    }

    return "NO".to_string();
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(football("001001".to_owned()), "NO".to_owned())
    }

    #[test]
    fn example2() {
        assert_eq!(football("100000001".to_owned()), "YES".to_owned())
    }
}
