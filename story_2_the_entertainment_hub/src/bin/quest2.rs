use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::iter::Cycle;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Balloon {
    R,
    G,
    B
}

fn main() {
    let file = read_to_string("input/quest2/input1.txt").unwrap();
    let mut balloons = parse(file);
    balloons.reverse();

    let mut cycle = fluffbolt_cycle();
    let mut part1 = 0;
    while !balloons.is_empty() {
        part1 += 1;
        let fluffbolt = cycle.next().unwrap();
        while let Some(b) = balloons.pop()
            && b == fluffbolt {}
    }

    println!("Part 1: {}", part1);

    let file = read_to_string("input/quest2/input2.txt").unwrap();
    let balloons = parse(file);
    let part2 = solve_circle(balloons, 100);
    println!("Part 2: {}", part2);

    let file = read_to_string("input/quest2/input3.txt").unwrap();
    let balloons = parse(file);
    let part3 = solve_circle(balloons, 100000);
    println!("Part 3: {}", part3);
}

fn solve_circle(balloons: Vec<Balloon>, size: usize) -> i32 {
    let mut side1: VecDeque<Balloon> = balloons.repeat(size/2).into();
    let mut side2: VecDeque<Balloon> = balloons.repeat(size/2).into();

    let mut cycle = fluffbolt_cycle();
    let mut result = 0;
    while !side1.is_empty() {
        result += 1;
        let fluffbolt = cycle.next().unwrap();
        let balloon1 = side1.get(0).unwrap();

        if &fluffbolt == balloon1 {
            let size = side1.len() + side2.len();
            let another_balloon_in_sight = size % 2 == 0;
            if another_balloon_in_sight {
                side2.pop_front().unwrap();
            }
        }
        side1.pop_front().unwrap();

        if side2.len() - side1.len() > 1 {
            side1.push_back(side2.pop_front().unwrap());
        }
    }
    result + side2.len() as i32
}

fn fluffbolt_cycle() -> Cycle<IntoIter<Balloon>> {
    vec![Balloon::R, Balloon::G, Balloon::B].into_iter().cycle()
}

fn parse(file: String) -> Vec<Balloon> {
    file.chars()
        .map(|c| match c {
            'R' => Balloon::R,
            'G' => Balloon::G,
            'B' => Balloon::B,
            _ => panic!("No balloon"),
        })
        .collect::<Vec<Balloon>>()
}