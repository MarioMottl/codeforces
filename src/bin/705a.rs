use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Couldnt read from stdin");
    /*
     * For example if
     * n = 1,"I hate it"
     * n = 2 "I hate that I love it"
     * n = 3 "I hate that I love that I hate it"
     * 0...n -> i i> 0 :: "that"
     * i % 2 == 0 {"I hate"} else {"I love"}
     * */

    let n: i32 = line.trim().parse().unwrap();
    let mut ans = String::new();

    for i in 0..n {
        if i > 0 {
            ans += " that ";
        }
        if i % 2 == 0 {
            ans += "I hate";
        } else {
            ans += "I love";
        }
    }
    ans += " it";

    println!("{}", ans);
}
