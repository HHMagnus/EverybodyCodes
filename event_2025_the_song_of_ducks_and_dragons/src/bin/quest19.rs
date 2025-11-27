use std::collections::{BTreeSet, HashMap, HashSet};
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("19", part1, part2, part3);
}

fn part1(file: &str) -> i64 {
    let walls = parse(file);

    run(walls)
}

fn run(walls: HashMap<i64, Vec<(i64, i64)>>) -> i64 {
    let start = (0, 0);

    let mut positions = BTreeSet::new();
    positions.insert(start);

    let mut x = 1;
    let max_x = *walls.keys().max().unwrap();
    let max_y = 2*walls.iter().map(|(_, vs)| vs.iter().map(|&x| x.0+x.1).max().unwrap()).max().unwrap();

    while x <= max_x + 4 {
        if x % 100000 == 0 {
            println!("{}", x);
        }
        positions = positions.into_iter()
            .flat_map(|(y, flaps)| {
                vec![(y + 1, flaps + 1), (y - 1, flaps)]
            })
            .filter(|&(y, _)| {
                if walls.contains_key(&x) {
                    let wall = walls.get(&x).unwrap();
                    return wall.iter().any(|wall| wall.0 <= y && y < wall.0 + wall.1);
                }
                y > 0 && y < max_y
            })
            .collect();
        x += 1;
    }

    positions.into_iter().map(|(_, flaps)| flaps).min().unwrap()
}

fn parse(file: &str) -> HashMap<i64, Vec<(i64, i64)>> {
    let holes = file.lines()
        .map(|line| {
            let list = line.split(",").map(|i| i.parse::<i64>().unwrap()).collect::<Vec<_>>();
            (list[0], (list[1], list[2]))
        }).collect::<Vec<_>>();
    let mut walls = HashMap::new();
    for hole in holes {
        walls.entry(hole.0).or_insert(Vec::new()).push(hole.1);
    }
    walls
}

fn part2(file: &str) -> i64 {
    let walls = parse(file);

    run(walls)
}

fn part3(file: &str) -> i64 {
    let walls = parse(file);

    run(walls)
}