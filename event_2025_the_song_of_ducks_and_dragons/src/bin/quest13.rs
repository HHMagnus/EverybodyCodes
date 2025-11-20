use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("13", part1, part2, part3);
}

fn part1(file: &str) -> usize {
    let nums = file.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut clock = vec![1];

    let mut behind = Vec::new();
    for (i, &num) in nums.iter().enumerate() {
        if i % 2 == 0 {
            clock.push(num);
        } else {
            behind.push(num);
        }
    }
    behind.reverse();
    clock.append(&mut behind);
    drop(behind);

    clock[2025 % clock.len()]
}

fn part2(file: &str) -> usize {
    let clock = parse_range_clock(file);

    clock[20252025 % clock.len()]
}

fn parse_range_clock(file: &str) -> Vec<usize> {
    let nums = file.lines()
        .map(|line| {
            let mut split = line.split("-");
            let first = split.next().unwrap()
                .parse::<usize>().unwrap();
            let second = split.next().unwrap()
                .parse::<usize>().unwrap();
            (first, second)
        })
        .collect::<Vec<_>>();

    let mut clock = vec![1];

    let mut behind = Vec::new();
    for (i, &(first, second)) in nums.iter().enumerate() {
        if i % 2 == 0 {
            for num in first..=second {
                clock.push(num);
            }
        } else {
            for num in first..=second {
                behind.push(num);
            }
        }
    }
    behind.reverse();
    clock.append(&mut behind);
    drop(behind);
    clock
}

fn part3(file: &str) -> usize {
    let clock = parse_range_clock(file);

    clock[202520252025 % clock.len()]
}