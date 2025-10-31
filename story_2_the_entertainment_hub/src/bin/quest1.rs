use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest1/input1.txt").unwrap();

    println!("{}", file);
}