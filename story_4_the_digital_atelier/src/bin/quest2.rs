use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

enum Destination {
    A,
    B,
    C
}

struct Quest2 {
    start: (i32, i32),
    a: (i32, i32),
    b: (i32, i32),
    c: (i32, i32),
    moves: Vec<Destination>
}

fn parse_coord(line: &str, prefix: &str) -> (i32, i32) {
    let mut start = line.split(",");
    let start0 = start.next().unwrap().replace(&(prefix.to_owned() + "=["), "").parse::<i32>().unwrap();
    let start1 = start.next().unwrap().replace("]", "").parse::<i32>().unwrap();
    (start0, start1)
}

impl Quest2 {
    fn parse(file: String) -> Self {
        let lines = file.lines().collect::<Vec<&str>>();
        let start = parse_coord(&lines[0], "START");
        let a = parse_coord(&lines[1], "A");
        let b = parse_coord(&lines[2], "B");
        let c = parse_coord(&lines[3], "C");
        let mut moves = Vec::new();
        if lines.len() == 5 {
            moves = lines[4].replace("MOVES=", "").chars()
                .map(|ch| match ch {
                    'A' => Destination::A,
                    'B' => Destination::B,
                    'C' => Destination::C,
                    _ => unreachable!("Unknown char {}", ch)
                })
                .collect();
        }
        Quest2 {
            start,
            a,
            b,
            c,
            moves
        }
    }
}

fn half(curr: i32, end: i32) -> i32 {
    curr + ((end as f32 - curr as f32) / 2f32).floor() as i32
}
fn main() {
    let file = read_to_string("input/quest2/input1.txt").unwrap();
    let part1 = part1and2(Quest2::parse(file)).len();
    println!("Quest 2 part 1: {}", part1);

    let file = read_to_string("input/quest2/input2.txt").unwrap();
    let part2 = fire_flies(part1and2(Quest2::parse(file)));
    println!("Quest 2 part 2: {}", part2);

    let file = read_to_string("input/quest2/input3.txt").unwrap();
    let part3 = fire_flies(part3_bettles(Quest2::parse(file)));
    println!("Quest 2 part 3: {}", part3);
}

fn fire_flies(bettles: HashSet<(i32, i32)>) -> usize {
    bettles.iter()
        .flat_map(|&(x, y)| vec![
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1)
        ])
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|fire_flies| !bettles.contains(fire_flies))
        .count()
}

fn part1and2(skyline: Quest2) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    let mut curr = skyline.start;
    visited.insert(curr);
    for mov in skyline.moves {
        let end = match mov {
            Destination::A => skyline.a,
            Destination::B => skyline.b,
            Destination::C => skyline.c,
        };
        curr = (half(curr.0, end.0), half(curr.1, end.1));
        visited.insert(curr);
    }
    visited
}


fn part3_bettles(skyline: Quest2) -> HashSet<(i32, i32)> {
    let moves = vec![skyline.a, skyline.b, skyline.c];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(skyline.start);
    visited.insert(skyline.start);
    while let Some(curr) = queue.pop_front() {
        for end in &moves {
            let new = (half(curr.0, end.0), half(curr.1, end.1));
            if visited.contains(&new) {
                continue;
            }
            visited.insert(new);
            queue.push_back(new);
        }
    }
    visited
}