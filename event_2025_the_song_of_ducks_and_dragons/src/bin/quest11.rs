use std::cmp::min;
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("11", part1, part2, part3);
}

fn part1(file: &str) -> usize {
    let mut nums = file.split('\n')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;

    while total < 10 {
        let mut moved = false;

        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                nums[i - 1] -= 1;
                nums[i] += 1;
                moved = true;
            }
        }

        if !moved {
            break;
        }

        total += 1;
    }

    while total < 10 {

        let mut moved = false;

        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                nums[i - 1] += 1;
                nums[i] -= 1;
                moved = true;
            }
        }

        if !moved {
            break;
        }
        total += 1;
    }

    nums.into_iter().enumerate().map(|(i, n)| (i+1) * n as usize).sum()
}

fn part2(file: &str) -> usize {
    let mut nums = file.split('\n')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;

    loop {
        let mut moved = false;

        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                nums[i - 1] -= 1;
                nums[i] += 1;
                moved = true;
            }
        }

        if !moved {
            break;
        }

        total += 1;
    }

    loop {
        let mut moved = false;

        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                nums[i - 1] += 1;
                nums[i] -= 1;
                moved = true;
            }
        }

        if !moved {
            break;
        }
        total += 1;
    }

    total
}

fn part3(file: &str) -> i64 {
    let mut nums = file.split('\n')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;

    loop {
        let mut moved = false;

        let amount = nums.windows(2)
            .map(|w| w[1] - w[0])
            .filter(|n| *n > 0)
            .min()
            .unwrap()
            .min(1i64);

        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                nums[i - 1] -= amount;
                nums[i] += amount;
                moved = true;
            }
        }

        if !moved {
            break;
        }

        total += amount;
    }

    loop {
        let mut moved = false;

        let amount = nums.windows(2)
            .map(|w| w[1] - w[0])
            .filter(|n| *n > 0)
            .min()
            .unwrap_or(1i64)
            .min(1i64);

        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                nums[i - 1] += amount;
                nums[i] -= amount;
                moved = true;
            }
        }

        if !moved {
            break;
        }
        total += amount;
    }

    total
}
