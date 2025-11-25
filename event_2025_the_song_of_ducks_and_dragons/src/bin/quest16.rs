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

    let factors = find_factors(&nums);

    factors.iter().product()
}

fn find_factors(nums: &Vec<usize>) -> Vec<usize> {
    let mut factors = Vec::new();

    for i in 0..nums.len() {
        let num = nums[i];
        let i = i + 1;

        let sum = factors.iter()
            .filter(|&n| i % n == 0)
            .count();

        if sum != num {
            factors.push(i);
        }
    }
    factors
}

fn part3(file: &str) -> usize {
    let nums = file.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let factors = find_factors(&nums);
    let max_blocks = 202520252025000usize;

    let mut min = 0;
    let mut max = max_blocks;
    while min < max {
        let mid = min + (max - min) / 2;
        let mid_length = factors.iter()
            .map(|i| mid / i)
            .sum::<usize>();
        if mid_length < max_blocks {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    min - 1
}