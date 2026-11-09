use std::collections::HashMap;
use std::fs::read_to_string;
use std::ptr::null;

#[derive(Clone, Copy, Debug)]
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
    let file = read_to_string("input/quest10/input1.txt").unwrap();

    let matrix = parse(file);

    let mut null_ghosts = matrix.iter()
        .filter_map(|(&xy, ch)| match ch {
            '^' => Some((xy, Direction::Up)),
            'v' => Some((xy, Direction::Down)),
            '<' => Some((xy, Direction::Left)),
            '>' => Some((xy, Direction::Right)),
            _ => None,
        }).collect::<Vec<_>>();

    let mut part1 = 0;

    for _ in 0..10 {
        null_ghosts = null_ghosts.into_iter()
            .map(|(xy, dir)| {
                let next = move_ghost(xy, dir);
                if matrix[&next] == '#' {
                    part1 += 1;
                    (xy, turn90(dir))
                } else {
                    (next, dir)
                }
            })
            .collect();
        let mut destinations = null_ghosts.iter()
            .map(|(xy, _)| *xy)
            .collect::<Vec<_>>();
        destinations.sort();
        let destinations = destinations.as_slice()
            .chunk_by(|xy, xy2| xy == xy2)
            .map(|chunk| (chunk[0], chunk.len()))
            .collect::<HashMap<_, _>>();
        null_ghosts = null_ghosts.into_iter()
            .map(|(xy, dir)| if destinations[&xy] > 1 {
                part1 += 1;
                (xy, turn180(dir))
            } else {
                (xy, dir)
            }).collect();
    }

    println!("Part 1: {}", part1);
}

fn parse(file: String) -> HashMap<(usize, usize), char> {
    file.lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate()
            .map(move |(j, ch)| ((j, i), ch)))
        .collect::<HashMap<_, _>>()
}