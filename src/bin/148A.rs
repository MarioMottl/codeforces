use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut input)
        .expect("Couldnt read from stdin");

    let mut it = input.split("\n");

    let frying_pan = it.next().unwrap().trim().parse::<u32>().unwrap();
    let tail_shut = it.next().unwrap().trim().parse::<u32>().unwrap();
    let paws_trampled = it.next().unwrap().trim().parse::<u32>().unwrap();
    let call_mom = it.next().unwrap().trim().parse::<u32>().unwrap();
    let dragons = it.next().unwrap().trim().parse::<u32>().unwrap();

    let mut unique: HashSet<u32> = HashSet::with_capacity(dragons as usize);

    for sequence in [frying_pan, tail_shut, paws_trampled, call_mom] {
        if sequence <= dragons {
            for item in (sequence..=dragons).step_by(sequence as usize) {
                unique.insert(item);
            }
        }
    }

    println!("{}", unique.len())
}
