use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("17", part1, part2, part3);
}

#[derive(PartialEq, Eq, Debug)]
enum Point {
    Vulcano,
    Duck,
    Num(usize),
}

impl Point {
    fn new(c: char) -> Self {
        match c {
            '@' => Self::Vulcano,
            'S' => Self::Duck,
            x => Self::Num(x.to_digit(10).unwrap() as usize),
        }
    }

    fn num(&self) -> usize {
        match self {
            Self::Num(x) => *x,
            _ => 0,
        }
    }
}

fn part1(file: &str) -> usize {
    let matrix = parse(file);

    let center = matrix.iter().find(|&(_, x)| x == &Point::Vulcano).unwrap().0.clone();
    let radius = 10;

    destruction(&matrix, center, radius)
}

fn destruction(matrix: &HashMap<(i32, i32), Point>, center: (i32, i32), radius: i32) -> usize {
    let (xv, yv) = center;

    matrix.into_iter()
        .filter(|&(&xy, _)| in_radius(radius, center, xy))
        .map(|(_, point)| match point {
            Point::Num(n) => *n,
            _ => 0,
        })
        .sum()
}

fn in_radius(radius: i32, vulcano: (i32, i32), point: (i32, i32)) -> bool {
    if vulcano == point {
        return true;
    }

    let (xv, yv) = vulcano;
    let (xc, yc) = point;
    (xv - xc) * (xv - xc) + (yv - yc) * (yv - yc) <= radius * radius
}

fn parse(file: &str) -> HashMap<(i32, i32), Point> {
    file.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), Point::new(c))))
        .collect::<HashMap<(i32, i32), Point>>()
}

fn part2(file: &str) -> usize {
    let matrix = parse(file);

    let center = matrix.iter().find(|&(_, x)| x == &Point::Vulcano).unwrap().0.clone();

    let mut radius = 0;
    let mut score = 0;

    let mut prev_score = 0;
    for i in 1..file.lines().count()*2 {
        let total_score = destruction(&matrix, center, i as i32);

        let curr_score = total_score - prev_score;

        if score < curr_score {
            radius = i as i32;
            score = curr_score;
        }

        prev_score = total_score;
    }

    score * radius as usize
}

fn part3(file: &str) -> i32 {
    let matrix = parse(file);

    let mut radius = 1;

    'o: loop {
        let vulcano = matrix.iter().find(|&(_, x)| x == &Point::Vulcano).unwrap().0.clone();

        let mut furthest = vulcano.1;
        while in_radius(radius, vulcano, (vulcano.0, furthest)) {
            furthest += 1;
        }

        let target = (vulcano.0, furthest);

        let duck = matrix.iter().find(|&(_, x)| x == &Point::Duck).unwrap().0.clone();

        let max_time = (radius+1) * 30;

        let left_path = find_path(&matrix, radius, vulcano, target, duck, true);
        let right_path = find_path(&matrix, radius, vulcano, target, duck, false);

        let total_time = left_path + right_path - matrix.get(&target).unwrap().num() as i32;

        if max_time > total_time {
            return radius * total_time;
        }

        radius += 1;
    }
}

fn find_path(matrix: &HashMap<(i32, i32), Point>, radius: i32, vulcano: (i32, i32), target: (i32, i32), duck: (i32, i32), left: bool) -> i32 {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), duck));
    visited.insert(duck);

    while let Some((Reverse(time), duck)) = queue.pop() {
        let (x, y) = duck;

        if duck == target {
            return time;
        }

        [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
        ].into_iter()
            .filter(|&(x, _)| (y < vulcano.1) || (left && x <= target.0) || (!left && x >= target.0))
            .filter(|&xy| !in_radius(radius, vulcano, xy))
            .filter(|xy| matrix.contains_key(xy))
            .for_each(|xy| {
                if !visited.contains(&xy) {
                    let cost = matrix.get(&xy).unwrap().num();
                    queue.push((Reverse(time + cost as i32), xy));
                    visited.insert(xy);
                }
            })
    }

    panic!("No path found!");
}