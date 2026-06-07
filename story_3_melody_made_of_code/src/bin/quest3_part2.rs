use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest3/part2.txt").unwrap();

    println!("Quest 3 part 2: {}", file);
}