use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest3/part1.txt").unwrap();

    println!("Quest 3 part 1: {}", file);
}