use std::collections::{HashMap, HashSet};
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("10", part1, part2, part3);
}

fn mov((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (x - 2, y + 1),
        (x - 2, y - 1),
        (x + 2, y + 1),
        (x + 2, y - 1),
        (x + 1, y - 2),
        (x - 1, y - 2),
        (x + 1, y + 2),
        (x - 1, y + 2)
    ]
}

#[derive(PartialEq, Eq, Debug)]
enum Type {
    Dragon,
    Sheep,
    HideOut,
    Empty,
}


impl Type {
    fn map(c: char) -> Self {
        match c {
            '.' => Type::Empty,
            'D' => Type::Dragon,
            'S' => Type::Sheep,
            '#' => Type::HideOut,
            x => panic!("Unknown char {}", x)
        }
    }
}

fn part1(file: &str) -> usize {
    let board = parse(file);

    let dragon = board.iter()
        .find(|&(_, y)| *y == Type::Dragon)
        .unwrap()
        .0.clone();

    let mut spaces = HashSet::new();
    spaces.insert(dragon);
    let mut curr = spaces.clone();
    for _ in 0..4 {
        let mut next = HashSet::new();
        for x in curr {
            for y in mov(x) {
                next.insert(y);
                spaces.insert(y);
            }
        }
        curr = next;
    }

    spaces.into_iter()
        .filter(|x| board.contains_key(x))
        .filter(|x| board[x] == Type::Sheep)
        .count()
}

fn parse(file: &str) -> HashMap<(i32, i32), Type> {
    file.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, ch)| {
            ((x as i32, y as i32), Type::map(ch))
        })
    }).collect::<HashMap<_, _>>()
}

fn part2(file: &str) -> usize {
    let board = parse(file);

    let dragon_tile = board.iter()
        .find(|&(_, y)| *y == Type::Dragon)
        .unwrap()
        .0.clone();
    let mut dragon = HashSet::new();
    dragon.insert(dragon_tile);

    let mut sheeps = board.iter()
        .filter(|&(_, y)| *y == Type::Sheep)
        .map(|(x, _)| *x)
        .collect::<Vec<_>>();

    let max_y = board.iter()
        .map(|x| x.0.1)
        .max()
        .unwrap();

    let mut total = 0;

    for _ in 0..20 {
        // Dragon turn
        dragon = dragon.into_iter().flat_map(|x| mov(x)).collect();

        let eaten = check_eaten(&board, &dragon, &sheeps);
        sheeps = sheeps.into_iter().filter(|x| !eaten.contains(x)).collect();
        total += eaten.len();

        // Sheep turn
        sheeps = sheeps.into_iter()
            .map(|x| (x.0, x.1+1))
            .filter(|&x| x.1 <= max_y)
            .collect();

        let eaten = check_eaten(&board, &dragon, &sheeps);
        sheeps = sheeps.into_iter().filter(|x| !eaten.contains(x)).collect();
        total += eaten.len();
    }

    total
}

fn check_eaten(board: &HashMap<(i32, i32), Type>, dragon: &HashSet<(i32, i32)>, sheeps: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    sheeps.iter()
        .filter(|x| board.contains_key(x))
        .filter(|x| board[x] != Type::HideOut)
        .filter(|x| dragon.contains(x))
        .cloned()
        .collect::<Vec<_>>()
}

fn part3(file: &str) -> usize {
    0
}