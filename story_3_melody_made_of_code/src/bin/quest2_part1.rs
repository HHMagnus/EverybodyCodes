use std::collections::HashSet;
use std::fs::read_to_string;
use story_3_melody_made_of_code::{quest_2_parse, Direction, State};

fn main() {
    let file = read_to_string("input/quest2/part1.txt").unwrap();

    let matrix = quest_2_parse(file);

    let start = matrix.iter().find(|&&x| x.0 == State::Start).unwrap().1;
    let end = matrix.iter().find(|&&x| x.0 == State::VocalBone).unwrap().1;

    let mut visited = HashSet::new();

    let mut cycle = [Direction::Up, Direction::Right, Direction::Down, Direction::Left].iter().cycle();
    let mut curr = start;

    while curr != end {
        visited.insert(curr);
        let dir = cycle.next().unwrap();
        let (x, y) = curr;
        let next = match dir {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };
        if visited.contains(&next) {
            continue;
        }
        curr = next;
    }

    println!("Quest 2 part 1: {}", visited.len());
}
