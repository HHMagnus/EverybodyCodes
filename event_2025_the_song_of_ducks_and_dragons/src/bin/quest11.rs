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

fn part3(file: &str) -> u64 {
    let nums = file.split('\n')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let sum = nums.iter().sum::<u64>();
    let mean = sum / nums.len() as u64;

    nums.iter()
        .filter(|&num| *num < mean)
        .map(|num| num.abs_diff(mean))
        .sum()
}
