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
        .map(|x: (i32, i32)| snail_position_formula(x))
        .sum::<i32>();
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
}

fn parse(file: String) -> Vec<(i32, i32)> {
    file.lines()
        .map(|line| {
            let mut split = line.split(" ");
            let x = split.next().unwrap().replace("x=", "").parse::<i32>().unwrap();
            let y = split.next().unwrap().replace("y=", "").parse::<i32>().unwrap();
            (x, y)
        }).collect::<Vec<_>>()
}

fn max((x, y): (i32, i32)) -> i32 {
    x + y - 1
}

fn diagonal((x, y): (i32, i32)) -> (i32, i32) {
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

fn snail_position_formula((x, y): (i32, i32)) -> i32 {
    x + (100 * y)
}