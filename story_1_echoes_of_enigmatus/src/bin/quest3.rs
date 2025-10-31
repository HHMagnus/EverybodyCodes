use std::cmp::min;
use std::fs::read_to_string;
use std::mem::swap;

fn main() {
    let file = read_to_string("input/quest3/input1.txt").unwrap();
    let coords = parse(file);

    let part1 = coords.into_iter()
        .map(|coord| {
            let mut c = coord;
            for _ in 0..100 {
                c = diagonal(c);
            }
            c
        })
        .map(|x: (u64, u64)| snail_position_formula(x))
        .sum::<u64>();
    println!("Part 1: {:?}", part1);

    let file = read_to_string("input/quest3/input2.txt").unwrap();
    let mut coords = parse(file);
    let mut part2 = 0;
    while !coords.iter().all(|coord| coord.1 == 1) {
        coords = coords.into_iter()
            .map(diagonal)
            .collect();
        part2 += 1;
    }
    println!("Part 2: {:?}", part2);

    let file = read_to_string("input/quest3/input3.txt").unwrap();

    let part3 = part3(file);
    println!("Part 3: {:?}", part3);
}

fn part3(file: String) -> u64 {
    let day_cycles = parse(file).into_iter()
        .map(|coord| {
            let mut c = coord;
            let mut day = 0;
            while c.1 != 1 {
                c = diagonal(c);
                day += 1;
            }
            let cycle = max(coord);
            (day, cycle)
        })
        .collect::<Vec<_>>();

    day_cycles.into_iter().reduce(least).unwrap().0
}

fn least(day_cycle1: (u64, u64), day_cycle2: (u64, u64)) -> (u64, u64) {
    let (mut day1, cycle1) = day_cycle1;
    let (mut day2, cycle2) = day_cycle2;
    while day1 != day2 {
        if day1 < day2 {
            day1 += cycle1;
        } else {
            day2 += cycle2;
        }
    }

    let lcm = (cycle1 * cycle2) / gcd(cycle1, cycle2);

    (day1, lcm)
}

// https://en.wikipedia.org/wiki/Binary_GCD_algorithm
pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both 2ⁱ u and 2ʲ v
    let i = u.trailing_zeros();  u >>= i;
    let j = v.trailing_zeros();  v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        // Swap if necessary so u ≤ v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Identity 4: gcd(u, v) = gcd(u, v-u) as u ≤ v and u, v are both odd
        v -= u;
        // v is now even

        if v == 0 {
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) as u is odd
        v >>= v.trailing_zeros();
    }
}

fn parse(file: String) -> Vec<(u64, u64)> {
    file.lines()
        .map(|line| {
            let mut split = line.split(" ");
            let x = split.next().unwrap().replace("x=", "").parse::<u64>().unwrap();
            let y = split.next().unwrap().replace("y=", "").parse::<u64>().unwrap();
            (x, y)
        }).collect::<Vec<_>>()
}

fn max((x, y): (u64, u64)) -> u64 {
    x + y - 1
}

fn diagonal((x, y): (u64, u64)) -> (u64, u64) {
    let max = max((x, y));
    let mut a = x + 1;
    if a > max {
        a = 1;
    }
    let mut b = y - 1;
    if b <= 0 {
        b = max;
    }
    (a, b)
}

fn snail_position_formula((x, y): (u64, u64)) -> u64 {
    x + (100 * y)
}