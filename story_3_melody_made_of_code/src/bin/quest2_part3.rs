use std::collections::HashSet;
use std::fs::read_to_string;
use story_3_melody_made_of_code::{neighbours, quest2_add_cluster_if_surrounded, quest_2_parse, Direction, State};

fn main() {
    let file = read_to_string("input/quest2/part3.txt").unwrap();

    let matrix = quest_2_parse(file);

    let start = matrix.iter().find(|&&x| x.0 == State::Start).unwrap().1;
    let vocal_bones = matrix.iter().filter(|&&x| x.0 == State::VocalBone).map(|x| x.1).collect::<Vec<_>>();

    let surrounding = vocal_bones.iter()
        .flat_map(|&x| neighbours(x))
        .collect::<HashSet<_>>();

    let mut visited = HashSet::new();
    for &vocal_bone in &vocal_bones {
        visited.insert(vocal_bone);
    }
    for &vocal_bone in &vocal_bones {
        for vocal_bone_neighbour in neighbours(vocal_bone) {
            quest2_add_cluster_if_surrounded(vocal_bone_neighbour, &mut visited);
        }
    }
    visited.insert(start);

    let mut cycle = [
        Direction::Up,
        Direction::Up,
        Direction::Up,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Left,
        Direction::Left,
    ].iter().cycle();
    let mut curr = start;

    let mut step = 0;

    while !surrounding.iter().all(|x| visited.contains(x)) {
        //debug_map(&vocal_bones, &mut visited, curr);

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

    println!("Quest 2 part 3: {}", step);
}

fn debug_map(vocal_bones: &Vec<(i32, i32)>, visited: &mut HashSet<(i32, i32)>, curr: (i32, i32)) {
    let max_y = *visited.iter().map(|(_, y)| y).max().unwrap();
    let min_y = *visited.iter().map(|(_, y)| y).min().unwrap();
    let max_x = *visited.iter().map(|(x, _)| x).max().unwrap();
    let min_x = *visited.iter().map(|(x, _)| x).min().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if vocal_bones.contains(&(x, y)) {
                print!("#");
            } else if curr == (x, y) {
                print!("@");
            } else if visited.contains(&(x, y)) {
                print!("+");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!();
    println!();
}