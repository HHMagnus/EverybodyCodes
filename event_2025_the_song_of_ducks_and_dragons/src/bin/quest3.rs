use std::collections::HashSet;
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("3", part1, part2, part3);
}

fn part1(file: &str) -> i32 {
    let nums = parse(file)
        .into_iter()
        .collect::<HashSet<_>>();
    nums.iter().sum()
}

fn parse(file: &str) -> Vec<i32> {
    file.split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn part2(file: &str) -> i32 {
    let mut nums = parse(file);
    nums.sort();
    nums.iter()
        .take(20)
        .sum()
}

fn part3(file: &str) -> usize {
    let mut nums = parse(file);
    nums.sort();
    nums.as_slice()
        .chunk_by(|x, y| x == y)
        .into_iter()
        .map(|v| v.len())
        .max()
        .unwrap()
}