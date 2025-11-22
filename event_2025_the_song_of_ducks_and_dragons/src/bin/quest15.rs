use std::collections::{HashSet, VecDeque};
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("15", part1, part2, part3);
}

enum Instruction {
    Right(usize),
    Left(usize),
}

impl Instruction {
    fn new(line: &str) -> Self {
        let char = line.chars().next().unwrap();
        let num = line[1..].to_string()
            .parse::<usize>().unwrap();
        match char {
            'R' => Instruction::Right(num),
            'L' => Instruction::Left(num),
            _ => panic!("Unknown instruction {}", char)
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn moves(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::East => (pos.0 + 1, pos.1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::West => (pos.0 - 1, pos.1),
        }
    }
}

fn part1(file: &str) -> usize {
    let instructions = file.split(",")
        .map(|line| Instruction::new(line))
        .collect::<Vec<_>>();

    let mut point = (0, 0);
    let mut dir = Direction::North;

    let mut walls = HashSet::new();
    walls.insert(point);

    for instruction in instructions {
        let num;
        (num, dir) = match instruction {
            Instruction::Right(num) => (num, dir.right()),
            Instruction::Left(num) => (num, dir.left()),
        };

        for _ in 0..num {
            point = dir.moves(point);
            walls.insert(point);
        }
    }

    let start = (0, 0);
    let end = point;

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashSet::new();

    while let Some((pos, steps)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        if pos == end {
            return steps;
        }
        let (x, y) = pos;

        [
            (x, y - 1),
            (x, y + 1),
            (x - 1, y),
            (x + 1, y),
        ].into_iter()
            .filter(|xy| !visited.contains(xy))
            .filter(|xy| xy == &end || !walls.contains(xy))
            .for_each(|xy| queue.push_back((xy, steps + 1)));
    }

    panic!("No solution found!");
}

fn part2(file: &str) -> usize {
    0
}

fn part3(file: &str) -> usize {
    0
}