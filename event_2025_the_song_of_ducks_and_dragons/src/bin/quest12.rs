use std::collections::{HashMap, HashSet, VecDeque};
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("12", part1, part2, part3);
}

fn part1(file: &str) -> usize {
    let matrix = parse(file);

    barrels(matrix, vec![(0, 0)])
}

fn barrels(
    matrix: HashMap<(i32, i32), u32>,
    initial: Vec<(i32, i32)>
) -> usize {
    let mut hit = HashSet::new();
    let mut queue = VecDeque::new();

    for xy in initial {
        queue.push_front(xy);
    }

    hits(&matrix, &mut hit, queue);

    hit.len()
}

fn hits(matrix: &HashMap<(i32, i32), u32>, hit: &mut HashSet<(i32, i32)>, mut queue: VecDeque<(i32, i32)>) {
    while let Some(xy) = queue.pop_front() {
        if hit.contains(&xy) {
            continue;
        }
        hit.insert(xy);
        let size = matrix[&xy];
        let (x, y) = xy;
        [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1)
        ].into_iter()
            .filter(|xy| matrix.contains_key(xy))
            .filter(|xy| matrix[xy] <= size)
            .for_each(|xy| queue.push_back(xy));
    }
}

fn parse(file: &str) -> HashMap<(i32, i32), u32> {
    file.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, chr)| ((x as i32, y as i32), chr.to_digit(10).unwrap())))
        .collect::<HashMap<_, _>>()
}

fn part2(file: &str) -> usize {
    let matrix = parse(file);

    let max = matrix.keys()
        .max()
        .unwrap()
        .clone();

    barrels(matrix, vec![(0, 0), max])
}

fn part3(file: &str) -> usize {
    let matrix = parse(file);

    let mut hit = HashSet::new();

    for _ in 0..3 {

        let starts = matrix.keys()
            .filter(|&xy| !hit.contains(xy))
            .cloned()
            .collect::<Vec<_>>();

        let mut best_hit = hit.clone();

        for start in starts {
            let mut cloned_hit = hit.clone();
            let mut queue = VecDeque::new();
            queue.push_back(start);
            hits(&matrix, &mut cloned_hit, queue);
            if best_hit.len() < cloned_hit.len() {
                best_hit = cloned_hit;
            }
        }

        hit = best_hit;

    }

    hit.len()
}