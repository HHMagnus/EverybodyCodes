use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("8", part1, part2, part3);
}

fn part1(file: &str) -> usize {
    let nums = file.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
    let mut result = 0;
    let size = 32;
    for w in nums.windows(2) {
        let total = w[0].abs_diff(w[1]);
        if size / 2 == total {
            result += 1;
        }
    }
    result
}

fn crosses((x1, x2): (usize, usize), (y1, y2): (usize, usize)) -> bool {
    if y1 == x1 || y1 == x2 || y2 == x1 || y2 == x2 {
        return false;
    }

    let min = x1.min(x2);
    let max = x1.max(x2);
    let cross1 = min < y1 && y1 < max;
    let cross2 = min < y2 && y2 < max;

    (cross1 && !cross2) || (!cross1 && cross2)
}

fn part2(file: &str) -> usize {
    let nums = file.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
    let mut cross = 0;
    let mut strings = Vec::new();
    for w in nums.windows(2) {
        let x1 = w[0];
        let x2 = w[1];

        for &(y1, y2) in strings.iter() {
            if crosses((x1, x2), (y1, y2)) {
                cross += 1;
            }
        }

        strings.push((x1, x2));
    }
    cross
}

fn part3(file: &str) -> usize {
    let nums = file.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
    let strings = nums.windows(2)
        .map(|w| (w[0], w[1]))
        .collect::<Vec<_>>();
    let mut res = 0;
    let nails = 256;
    for x1 in 1..=nails {
        for x2 in 1..=nails {
            let cross = strings.iter()
                .filter(|&(y1, y2)|
                    crosses((x1, x2), (*y1, *y2))
                    || (x1 == *y1 && x2 == *y2) || (x1 == *y2 && x2 == *y1)
                )
                .count();
            res = res.max(cross);
        }
    }
    res
}