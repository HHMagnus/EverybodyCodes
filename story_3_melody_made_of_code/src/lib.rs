use std::collections::{HashSet, VecDeque};
use std::ops::{Add, Sub};

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum State {
    Start,
    VocalBone
}

pub fn quest_2_parse(file: String) -> Vec<(State, (i32, i32))> {
    let matrix = file.lines()
        .enumerate()
        .flat_map(|(y, str)| str.chars().enumerate().map(move |(x, c)| {
            match c {
                '#' => Some((State::VocalBone, (x as i32, y as i32))),
                '@' => Some((State::Start, (x as i32, y as i32))),
                _ => None
            }
        }))
        .filter_map(|x| x)
        .collect::<Vec<_>>();
    matrix
}

pub fn neighbours<T>(start: (T, T)) -> [(T, T); 4]
where T: Add<Output = T> + Sub<Output = T> + From<u8> + Copy {
    [
        Direction::Up.go(start),
        Direction::Right.go(start),
        Direction::Down.go(start),
        Direction::Left.go(start),
    ]
}

impl Direction {
    pub fn go<T>(&self, (x, y): (T, T)) -> (T, T)
    where T: Add<Output = T> + Sub<Output = T> + From<u8> {
        let step = 1.into();
        match self {
            Direction::Up => (x, y - step),
            Direction::Right => (x + step, y),
            Direction::Down => (x, y + step),
            Direction::Left => (x - step, y),
        }
    }
}

pub fn quest2_add_cluster_if_surrounded(start: (i32, i32), visited: &mut HashSet<(i32, i32)>) {
    if visited.contains(&start) {
        return;
    }
    let mut visiting = HashSet::new();
    visiting.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let max_y = *visited.iter().map(|(_, y)| y).max().unwrap();
    let min_y = *visited.iter().map(|(_, y)| y).min().unwrap();
    let max_x = *visited.iter().map(|(x, _)| x).max().unwrap();
    let min_x = *visited.iter().map(|(x, _)| x).min().unwrap();

    while let Some(next) = queue.pop_front() {
        if next.0 > max_x || next.0 < min_x || next.1 > max_y || next.1 < min_y {
            return;
        }

        for pos in neighbours(next) {
            if visited.contains(&pos) {
                continue;
            }
            if visiting.contains(&pos) {
                continue;
            }
            visiting.insert(pos);
            queue.push_back(pos);
        }
    }


    for x in visiting {
        visited.insert(x);
    }
}