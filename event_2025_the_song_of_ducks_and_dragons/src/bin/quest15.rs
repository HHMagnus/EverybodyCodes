use event_2025_the_song_of_ducks_and_dragons::solve;
use std::collections::{HashSet, VecDeque};

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

#[derive(Copy, Clone)]
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

    fn moves(&self, pos: (i32, i32), num: usize) -> (i32, i32) {
        let num = num as i32;
        match self {
            Direction::North => (pos.0, pos.1 - num),
            Direction::East => (pos.0 + num, pos.1),
            Direction::South => (pos.0, pos.1 + num),
            Direction::West => (pos.0 - num, pos.1),
        }
    }
}

struct Wall {
    start: (i32, i32),
    direction: Direction,
    length: usize,
}

impl Wall {
    fn new(start: (i32, i32), direction: Direction, length: usize) -> Self {
        Wall {
            start,
            direction,
            length,
        }
    }

    fn contains(&self, pos: &(i32, i32)) -> bool {
        match self.direction {
            Direction::North =>
                pos.0 == self.start.0
                    && self.start.1 >= pos.1
                    && pos.1 >= self.start.1 - self.length as i32,
            Direction::South =>
                pos.0 == self.start.0
                    && self.start.1 <= pos.1
                    && pos.1 <= self.start.1 + self.length as i32,
            Direction::East =>
                pos.1 == self.start.1
                    && self.start.0 <= pos.0
                    && pos.0 <= self.start.0 + self.length as i32,
            Direction::West =>
                pos.1 == self.start.1
                    && self.start.0 >= pos.0
                    && pos.0 >= self.start.0 - self.length as i32,
        }
    }
}

fn part1(file: &str) -> usize {
    solve_quest15(file)
}

fn shortest_path(end: &(i32, i32), walls: Vec<Wall>, start: (i32, i32)) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashSet::new();

    while let Some((pos, steps)) = queue.pop_front() {
        let (x, y) = pos;

        for xy in [
            (x, y - 1),
            (x, y + 1),
            (x - 1, y),
            (x + 1, y),
        ] {
            if &xy == end {
                return Some(steps + 1);
            }

            if walls.iter().any(|wall| wall.contains(&xy)) {
                continue
            }

            if !visited.contains(&xy) {
                visited.insert(xy);
                queue.push_back((xy, steps + 1))
            }
        }
    }

    None
}

fn walls(instructions: Vec<Instruction>) -> ((i32, i32), Vec<Wall>) {
    let mut point = (0, 0);
    let mut dir = Direction::North;

    let mut walls = Vec::new();

    for instruction in instructions {
        let num;
        (num, dir) = match instruction {
            Instruction::Right(num) => (num, dir.right()),
            Instruction::Left(num) => (num, dir.left()),
        };

        let wall = Wall::new(point, dir, num);
        point = dir.moves(point, num);
        walls.push(wall);
    }
    (point, walls)
}

fn part2(file: &str) -> usize {
    solve_quest15(file)
}

fn solve_quest15(file: &str) -> usize {
    let instructions = file.split(",")
        .map(|line| Instruction::new(line))
        .collect::<Vec<_>>();

    let start = (0, 0);
    let (end, walls) = walls(instructions);

    let shortest_path = shortest_path(&end, walls, start);

    shortest_path.expect("No solution found")
}

fn part3(file: &str) -> usize {
    solve_quest15(file)
}