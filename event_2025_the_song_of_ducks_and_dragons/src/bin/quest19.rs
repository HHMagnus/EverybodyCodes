use event_2025_the_song_of_ducks_and_dragons::solve;
use std::collections::{BTreeSet, HashMap};

fn main() {
    solve("19", part1, part2, part3);
}

fn part1(file: &str) -> i64 {
    let walls = parse(file);

    unoptimized_solution(walls)
}

fn unoptimized_solution(walls: HashMap<i64, Vec<(i64, i64)>>) -> i64 {
    let start = (0, 0);

    let mut positions = BTreeSet::new();
    positions.insert(start);

    let mut x = 1;
    let max_x = *walls.keys().max().unwrap();
    let max_y = 2*walls.iter().map(|(_, vs)| vs.iter().map(|&x| x.0+x.1).max().unwrap()).max().unwrap();

    while x <= max_x + 4 {
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

    optimized_solution(walls)
}

fn part3(file: &str) -> i64 {
    let walls = parse(file);

    optimized_solution(walls)
}

fn optimized_solution(walls: HashMap<i64, Vec<(i64, i64)>>) -> i64 {
    let mut walls = walls.into_iter().collect::<Vec<_>>();
    walls.sort_by(|a, b| a.0.cmp(&b.0));

    let mut positions = BTreeSet::new();
    positions.insert((0, 0));

    // new_y = old_y - (new_x - old_x) + 2 * flaps
    // (new_y - old_y + (new_x - old_x)) / 2 = flaps

    let mut old_x = 0;

    for (new_x, ys) in walls {
        let mut new_positions = BTreeSet::new();
        for (old_y, flaps) in positions {
            let diff_x = new_x - old_x;

            for &(new_y_base, new_y_tops) in &ys {
                for new_y in new_y_base..new_y_base + new_y_tops {
                    let required_flaps_2 = new_y - old_y + diff_x;
                    if required_flaps_2 < 0 {
                        continue
                    }
                    if required_flaps_2 % 2 != 0 {
                        continue
                    }
                    let required_flaps = required_flaps_2 / 2;
                    if required_flaps > diff_x {
                        continue
                    }
                    new_positions.insert((new_y, required_flaps + flaps));
                }
            }
        }
        old_x = new_x;
        positions = new_positions;
    }

    positions.into_iter().map(|(_, flaps)| flaps).min().unwrap()
}