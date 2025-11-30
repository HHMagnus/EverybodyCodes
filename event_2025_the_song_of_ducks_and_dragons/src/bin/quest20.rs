use event_2025_the_song_of_ducks_and_dragons::solve;
use std::collections::{HashMap, HashSet, VecDeque};

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

#[derive(PartialEq, Eq, Copy, Clone)]
enum Point {
    Empty,
    Trampoline,
    GoldenTrampoline,
    Hole,
    Start,
}

impl Point {
    fn is_jumpable(&self) -> bool {
        match self {
            Point::Start => true,
            Point::Trampoline => true,
            Point::GoldenTrampoline => true,
            _ => false,
        }
    }
}

impl Point {
    fn parse(c: char) -> Point {
        match c {
            'T' => Point::Trampoline,
            'E' => Point::GoldenTrampoline,
            'S' => Point::Start,
            '#' => Point::Hole,
            _ => Point::Empty,
        }
    }
}

fn part1(file: &str) -> usize {
    let matrix = parse(file);

    let trampolines = matrix.iter().filter(|&(_, p)| p == &Point::Trampoline)
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
    let matrix = parse(file);
    let matrix2 = rotate(&matrix);
    let matrix3 = rotate(&matrix2);

    let start = find_point(&matrix, Point::Start);

    let matrices = vec![matrix, matrix2, matrix3];

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert((start, 0));
    queue.push_back((start, 0, 0));

    while let Some((pos, rotation, steps)) = queue.pop_front() {
        if matrices[rotation][&pos] == Point::GoldenTrampoline {
            return steps
        }

        let rotation = (rotation + 1) % 3;
        let chosen_matrix = &matrices[rotation];

        let mut neighbours = neighbours(pos).into_iter().collect::<Vec<_>>();
        neighbours.push(pos);
        neighbours.into_iter()
            .filter(|neighbour| chosen_matrix.contains_key(&neighbour) && chosen_matrix[&neighbour].is_jumpable())
            .for_each(|neighbour| {
                let visited_key = (neighbour, rotation);
                if !visited.contains(&visited_key) {
                    visited.insert(visited_key);
                    queue.push_back((neighbour, rotation, steps + 1));
                }
            })
    }

    panic!("No solution found");
}

// https://github.com/wilkotom/EverybodyCodes/blob/main/rust/2025/day20/src/main.rs
fn rotate(matrix: &HashMap<(i32, i32), Point>) -> HashMap<(i32, i32), Point> {
    let mut new_grid = HashMap::new();
    let start_point = matrix.keys()
        .filter(|pos| matrix.contains_key(&pos) && matrix[&pos] != Point::Empty)
        .fold((0, 0), |c, &k| if c.1 > k.1 {c} else {k});
    let mut start_x = start_point.0;
    let mut start_y = start_point.1;
    let mut level = 0;
    while start_y >= 0 {
        let mut original = (start_x, start_y);

        let mut new_coord = (start_x - start_point.0, level);
        if let Some(entry) = matrix.get(&original) {
            new_grid.insert(new_coord, *entry);
        }
        while original.1 > 0 {
            original.1 -= 1;
            new_coord.0 +=1;
            if let Some(entry) = matrix.get(&original) {
                new_grid.insert(new_coord, *entry);
            }
            original.0 -= 1;
            new_coord.0 +=1;
            if let Some(entry) = matrix.get(&original) {
                new_grid.insert(new_coord, *entry);
            }
        }
        level +=1;
        start_x += 1;
        start_y -=1;
    }

    new_grid
}