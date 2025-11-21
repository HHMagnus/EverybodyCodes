use event_2025_the_song_of_ducks_and_dragons::solve;
use std::collections::{BTreeSet, HashMap};

fn main() {
    solve("14", part1, part2, part3);
}

fn part1(file: &str) -> usize {
    let (max_y, max_x, active) = parse(file);

    part1and2(max_y, max_x, active, 10)
}

fn part1and2(max_y: i32, max_x: i32, active: BTreeSet<(i32, i32)>, rounds: usize) -> usize {
    let mut active = active;

    let mut result = 0;

    for _ in 0..rounds {
        active = round(max_y, max_x, active);
        result += active.len();
    }

    result
}

fn round(max_y: i32, max_x: i32, active: BTreeSet<(i32, i32)>) -> BTreeSet<(i32, i32)> {
    let mut new_active = BTreeSet::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let actives = [
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
            ].into_iter()
                .filter(|xy| active.contains(xy))
                .count();
            let xy = (x, y);
            let is_active = active.contains(&xy);
            if is_active && actives % 2 == 1 {
                new_active.insert(xy);
            }
            if !is_active && actives % 2 == 0 {
                new_active.insert(xy);
            }
        }
    }
    new_active
}

fn parse(file: &str) -> (i32, i32, BTreeSet<(i32, i32)>) {
    let data = file.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, chr)| ((x as i32, y as i32), chr)))
        .collect::<Vec<_>>();
    let max_y = data.iter().map(|&((_, y), _)| y).max().unwrap();
    let max_x = data.iter().map(|&((x, _), _)| x).max().unwrap();
    let mut active = data.into_iter()
        .filter(|(_, ch)| ch == &'#')
        .map(|(x, _)| x)
        .collect::<BTreeSet<_>>();
    (max_y, max_x, active)
}

fn part2(file: &str) -> usize {
    let (max_y, max_x, active) = parse(file);

    part1and2(max_y, max_x, active, 2025)
}

fn part3(file: &str) -> usize {
    let (sample_y, sample_x, sample) = parse(file);

    let max_y = 34;
    let max_x = 34;

    let mut seen = HashMap::new();

    let mut active = BTreeSet::new();
    let mut result = 0;

    let mut i = 0;
    let rounds = 1000000000;
    let mut skipped = false;
    while i < rounds {
        i += 1;

        active = round(max_y - 1, max_x - 1, active);
        if active.contains_sample(max_y, max_x, sample_y, sample_x, &sample) {
            result += active.len();
        }

        if seen.contains_key(&active) && !skipped {
            let (seen_i, seen_result) = seen.get(&active).unwrap();
            let diff_i = i - seen_i;
            let diff_result = result - seen_result;
            while i+diff_i < rounds {
                i += diff_i;
                result += diff_result;
            }
            skipped = true;
        }
        seen.insert(active.clone(), (i, result));
    }

    result
}

trait ContainsAll {
    fn contains_sample(&self, max_y: i32, max_x: i32, sample_y: i32, sample_x: i32, sample: &BTreeSet<(i32, i32)>) -> bool;
}

impl ContainsAll for BTreeSet<(i32, i32)> {
    fn contains_sample(&self, max_y: i32, max_x: i32, sample_y: i32, sample_x: i32, sample: &BTreeSet<(i32, i32)>) -> bool {
        let offset_x = (max_x - sample_x) / 2;
        let offset_y = (max_y - sample_y) / 2;

        for x in 0..sample_x {
            for y in 0..sample_y {
                let xy = (x, y);
                let sample_xy = (x + offset_x, y + offset_y);
                if self.contains(&sample_xy) != sample.contains(&xy) {
                    return false;
                }
            }
        }
        true
    }
}