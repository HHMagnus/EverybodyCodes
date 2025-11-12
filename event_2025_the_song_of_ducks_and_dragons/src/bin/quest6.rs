use std::ascii::AsciiExt;
use std::collections::{HashMap, VecDeque};
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("6", part1, part2, part3);
}

fn part1(file: &str) -> i32 {
    let mut count = 0;
    let mut score = 0;
    for char in file.chars() {
        if char == 'A' {
            count += 1;
        }
        if char == 'a' {
            score += count;
        }
    }
    score
}

fn part2(file: &str) -> i32 {
    let mut count = HashMap::new();
    let mut score = 0;
    for char in file.chars() {
        if char.is_uppercase() {
            count.entry(char).and_modify(|e| *e += 1).or_insert(1);
        }
        if char.is_lowercase() {
            let count = count.get(&char.to_ascii_uppercase()).unwrap_or(&0);
            score += count;
        }
    }
    score
}

fn part3(file: &str) -> usize {
    let mut count = HashMap::new();
    let mut score = 0;
    for (i, char) in file.chars().cycle().take(file.len() * 1000).enumerate() {
        if char.is_lowercase() {
            continue;
        }
        let entry = count.entry(char).or_insert(VecDeque::new());
        entry.push_back(i);
    }

    for (i, char) in file.chars().cycle().take(file.len() * 1000).enumerate() {
        if char.is_uppercase() {
            continue;
        }
        let entry = count.entry(char.to_ascii_uppercase()).or_insert(VecDeque::new());
        while let Some(f) = entry.pop_front() {
            if f.abs_diff(i) <= 1000 {
                entry.push_front(f);
                break;
            }
        }
        for x in entry.iter() {
            if x.abs_diff(i) > 1000 {
                break;
            }
            score += 1;
        }
    }
    score
}