use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .expect("Couldnt read from stdin");
    let input_int: u32 = buffer.parse().expect("Couldnt parse input");
    if input_int % 2 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
