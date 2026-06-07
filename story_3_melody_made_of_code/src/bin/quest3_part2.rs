use std::fs::read_to_string;
use story_3_melody_made_of_code::quest3_solve;

fn main() {
    let file = read_to_string("input/quest3/part2.txt").unwrap();

    let allow_week_bonds = true;

    let result = quest3_solve(file, allow_week_bonds);

    println!("Quest 3 part 1: {}", result);
}