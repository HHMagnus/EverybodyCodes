use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("4", part1, part2, part3);
}

fn part1(file: &str) -> i32 {
    let nums = parse(file);
    let max = *nums.iter().max().unwrap() as f64;
    let min = *nums.iter().min().unwrap() as f64;
    (max / min * 2025f64).floor() as i32
}

fn parse(file: &str) -> Vec<i32> {
    file.split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn part2(file: &str) -> i64 {
    let nums = parse(file);
    let max = *nums.iter().max().unwrap() as f64;
    let min = *nums.iter().min().unwrap() as f64;
    (10000000000000f64 / (max / min)).ceil() as i64
}

fn parse3(file: &str) -> (i32, Vec<(i32, i32)>, i32) {
    let mut nums = file.split("\n");
    let start = nums.next().unwrap()
        .parse::<i32>().unwrap();

    let mut between = Vec::new();

    let mut next = nums.next().unwrap();
    while let Some(next_next) = nums.next() {
        let mut split = next.split("|");
        let left = split.next().unwrap()
            .parse::<i32>().unwrap();
        let right = split.next().unwrap()
            .parse::<i32>().unwrap();
        between.push((left, right));
        next = next_next;
    }
    let end = next.parse::<i32>().unwrap();
    (start, between, end)
}

fn part3(file: &str) -> i64 {
    let (start, nums, end) = parse3(file);

    let mut factor = 1f64;

    let mut curr = start as f64;
    for (b1, b2) in nums {
        let fact = curr / (b1 as f64);
        factor *= fact;
        curr = b2 as f64;
    }
    factor *= (curr / end as f64);
    (100f64 * factor).floor() as i64
}
