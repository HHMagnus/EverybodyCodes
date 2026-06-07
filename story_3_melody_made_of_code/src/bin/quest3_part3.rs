use std::fs::read_to_string;
use story_3_melody_made_of_code::quest3_solve;

fn main() {
    let file = read_to_string("input/quest3/part3.txt").unwrap();

    let allow_weak_bonds = true;
    let replace_weak_bonds = true;

    let result = quest3_solve(file, allow_weak_bonds, replace_weak_bonds);

    println!("Quest 3 part 3: {}", result);
}