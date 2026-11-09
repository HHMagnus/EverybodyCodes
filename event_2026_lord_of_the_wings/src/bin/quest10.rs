use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn90(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn turn180(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
    }
}

fn move_ghost((x, y): (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn main() {
    part1();
    part2();
    part3();
}

fn part1() {
    let file = read_to_string("input/quest10/input1.txt").unwrap();
    let matrix = parse(file);
    let null_ghosts = find_null_ghosts(&matrix);

    let mut null_ghosts = null_ghosts.into_iter()
        .map(|(xy, dir)| (0, xy, dir, 0))
        .collect::<Vec<_>>();

    for _ in 0..10 {
        null_ghosts = move_null_ghosts(&matrix, null_ghosts, usize::MAX);
    }

    let part1 = null_ghosts.iter()
        .map(|(_, _, _, collisions)| collisions)
        .sum::<usize>();

    println!("Part 1: {}", part1);
}

fn part2() {
    let file = read_to_string("input/quest10/input2.txt").unwrap();
    let matrix = parse(file);
    let null_ghosts = find_null_ghosts(&matrix);

    let mut null_ghosts = null_ghosts.into_iter()
        .map(|(xy, dir)| (0, xy, dir, 0))
        .collect::<Vec<_>>();

    let mut part2 = 0;
    let disappear = 10000;

    while !null_ghosts.iter().all(|(_, _,_, collisions)| *collisions == disappear) {
        null_ghosts = move_null_ghosts(&matrix, null_ghosts, disappear);
        part2 += 1;
    }

    println!("Part 2: {}", part2);
}

fn part3() {
    let file = read_to_string("input/quest10/input3.txt").unwrap();
    let matrix = parse(file);
    let null_ghosts = find_null_ghosts(&matrix);

    let mut null_ghosts = null_ghosts.into_iter()
        .enumerate()
        .map(|(id, (xy, dir))| (id, xy, dir, 0usize))
        .collect::<Vec<_>>();

    let mut rounds = 0;
    let disappear = 1000000000000;

    let mut cycles: HashMap<Vec<(usize, (usize, usize), Direction)>, Vec<(usize, Vec<(usize, (usize, usize), Direction, usize)>)>> = HashMap::new();

    let mut len = 0;
    'outer: while !null_ghosts.iter().all(|(_, _,_, collisions)| *collisions == disappear) {
        if len != null_ghosts.len() {
            println!("Null ghost disappeared: {}, {}", rounds, null_ghosts.len());
            len = null_ghosts.len();
        }
        let key = null_ghosts.iter()
            .map(|&(id, xy, dir, _)| (id, xy, dir))
            .collect::<Vec<_>>();
        if cycles.contains_key(&key) {
            let prevs = cycles.get(&key).unwrap();
            for (prev_rounds, prev) in prevs {
                let collision_diff = null_ghosts.iter()
                    .map(|&(id, xy, dir, collisions)| {
                        let prev_collision = prev.iter()
                            .find(|&&(id2, _, _, _)| id == id2)
                            .unwrap();
                        let diff = collisions - prev_collision.3;
                        if diff == 0 {
                            (id, xy, dir, collisions, 0, usize::MAX)
                        } else {
                            let can_be_added = (disappear - collisions - 1) / diff;
                            (id, xy, dir, collisions, diff, can_be_added)
                        }
                    }).collect::<Vec<_>>();
                let move_by = collision_diff.iter()
                    .map(|(_, _, _, _, _, can_be_added)| *can_be_added)
                    .min()
                    .unwrap();

                if move_by > 0 && move_by != usize::MAX {
                    rounds += (rounds - prev_rounds) * move_by;
                    null_ghosts = collision_diff.into_iter()
                        .map(|(id, xy, dir, collisions, diff, _)| (id, xy, dir, collisions+diff*move_by))
                        .filter(|(_, _, _, collisions)| collisions < &disappear)
                        .collect();
                    continue 'outer;
                }
            }
        }

        cycles.entry(key).or_insert(vec![])
            .push((rounds, null_ghosts.clone()));

        null_ghosts = move_null_ghosts(&matrix, null_ghosts, disappear)
            .into_iter()
            .filter(|(_, _, _, collisions)| collisions < &disappear)
            .collect();
        rounds += 1;
    }

    println!("Part 3: {}", rounds);
}

fn move_null_ghosts(matrix: &HashMap<(usize, usize), char>, null_ghosts: Vec<(usize, (usize, usize), Direction, usize)>, disappear: usize) -> Vec<(usize, (usize, usize), Direction, usize)> {
    let moved_null_ghosts = null_ghosts.iter()
        .map(|&(id, xy, dir, collisions)| {
            let next = move_ghost(xy, dir);
            if collisions == disappear {
                (id, xy, dir, collisions)
            } else if matrix[&next] == '#' {
                (id, xy, turn90(dir), collisions + 1)
            } else {
                (id, next, dir, collisions)
            }
        })
        .collect::<Vec<_>>();
    let mut destinations = moved_null_ghosts.iter()
        .filter_map(|(_, xy, _, collisions)| if *collisions >= disappear { None } else { Some(*xy) })
        .collect::<Vec<_>>();
    destinations.sort();
    let destinations = destinations.as_slice()
        .chunk_by(|xy, xy2| xy == xy2)
        .map(|chunk| (chunk[0], chunk.len()))
        .collect::<HashMap<_, _>>();
    let moved_null_ghosts = moved_null_ghosts.into_iter()
        .map(|(id, xy, dir, collisions)| if collisions == disappear {
            (id, xy, dir, collisions)
        } else if destinations[&xy] > 1 {
            (id, xy, turn180(dir), collisions + 1)
        } else {
            (id, xy, dir, collisions)
        }).collect::<Vec<_>>();
    moved_null_ghosts
}

fn find_null_ghosts(matrix: &HashMap<(usize, usize), char>) -> Vec<((usize, usize), Direction)> {
    matrix.iter()
        .filter_map(|(&xy, ch)| match ch {
            '^' => Some((xy, Direction::Up)),
            'v' => Some((xy, Direction::Down)),
            '<' => Some((xy, Direction::Left)),
            '>' => Some((xy, Direction::Right)),
            _ => None,
        }).collect::<Vec<_>>()
}

fn parse(file: String) -> HashMap<(usize, usize), char> {
    file.lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate()
            .map(move |(j, ch)| ((j, i), ch)))
        .collect::<HashMap<_, _>>()
}