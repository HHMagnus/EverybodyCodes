use std::collections::{HashMap, HashSet, VecDeque};
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("20", part1, part2, part3);
}

fn neighbours(point: (i32, i32)) -> [(i32, i32); 3] {
    let y_even = point.1 % 2 == 0;
    let x_even = point.0 % 2 == 0;

    if y_even == x_even {
        [(point.0, point.1 - 1), (point.0 - 1, point.1), (point.0 + 1, point.1)]
    } else {
        [(point.0, point.1 + 1), (point.0 - 1, point.1), (point.0 + 1, point.1)]
    }
}

#[derive(PartialEq, Eq)]
enum Point {
    Empty,
    Trampoline,
    GoldenTrampoline,
    Start,
}

impl Point {
    fn parse(c: char) -> Point {
        match c {
            'T' => Point::Trampoline,
            'E' => Point::GoldenTrampoline,
            'S' => Point::Start,
            _ => Point::Empty,
        }
    }
}

fn part1(file: &str) -> usize {
    let matrix = parse(file);

    let trampolines = matrix.iter().filter(|&(x, p)| p == &Point::Trampoline)
        .map(|(xy, _)| xy)
        .cloned()
        .collect::<Vec<_>>();

    let points = trampolines.into_iter()
        .flat_map(|point| neighbours(point).into_iter()
            .filter(|neighbour| matrix.contains_key(&neighbour) && matrix[&neighbour] == Point::Trampoline)
            .map(move |neighbour| {
                let mut vec = vec![point, neighbour];
                vec.sort();
                vec
            })
        )
        .collect::<HashSet<_>>();

    points.len()
}

fn parse(file: &str) -> HashMap<(i32, i32), Point> {
    file.lines().enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), Point::parse(c)))
        }).collect::<HashMap<_, _>>()
}

fn part2(file: &str) -> usize {
    let matrix = parse(file);

    let start = find_point(&matrix, Point::Start);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back((start, 0));

    while let Some((pos, steps)) = queue.pop_front() {
        if matrix[&pos] == Point::GoldenTrampoline {
            return steps
        }
        neighbours(pos).into_iter()
            .filter(|neighbour| matrix.contains_key(&neighbour) && (matrix[&neighbour] == Point::Trampoline || matrix[&neighbour] == Point::GoldenTrampoline))
            .for_each(|neighbour| {
                if !visited.contains(&neighbour) {
                    visited.insert(neighbour);
                    queue.push_back((neighbour, steps + 1));
                }
            })
    }

    panic!("No solution found")
}

fn find_point(matrix: &HashMap<(i32, i32), Point>, point: Point) -> (i32, i32) {
    matrix.iter()
        .find(|&(_, p)| p == &point)
        .unwrap()
        .0.clone()
}

fn part3(file: &str) -> i64 {
    0
}