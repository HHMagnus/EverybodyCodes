use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("16", part1, part2, part3);
}

fn part1(file: &str) -> usize {
    let nums = file.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    nums.into_iter()
        .map(|num| 90 / num)
        .sum()
}

fn part2(file: &str) -> usize {
    let nums = file.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    0
}

fn part3(file: &str) -> usize {
    0
}