use std::collections::HashSet;
use std::fs::read_to_string;
use story_3_melody_made_of_code::{neighbours, quest2_add_cluster_if_surrounded, quest_2_parse, Direction, State};

fn main() {
    let file = read_to_string("input/quest2/part2.txt").unwrap();

    let matrix = quest_2_parse(file);

    let start = matrix.iter().find(|&&x| x.0 == State::Start).unwrap().1;
    let end = matrix.iter().find(|&&x| x.0 == State::VocalBone).unwrap().1;

    let surrounding = neighbours(end);

    let mut visited = HashSet::new();
    visited.insert(end);
    visited.insert(start);

    let mut cycle = [Direction::Up, Direction::Right, Direction::Down, Direction::Left].iter().cycle();
    let mut curr = start;

    let mut step = 0;

    while !surrounding.iter().all(|x| visited.contains(x)) {
        let dir = cycle.next().unwrap();
        let next = dir.go(curr);
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);

        for neighbour in neighbours(next) {
            quest2_add_cluster_if_surrounded(neighbour, &mut visited);
        }
        step += 1;
        curr = next;
    }

    println!("Quest 2 part 2: {}", step);
}
