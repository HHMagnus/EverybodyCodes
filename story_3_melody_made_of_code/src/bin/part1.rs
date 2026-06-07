use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/part1.txt").unwrap();
    println!("{}", file);
}