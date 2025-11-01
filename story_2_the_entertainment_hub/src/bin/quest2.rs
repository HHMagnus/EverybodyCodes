use std::cmp::PartialEq;
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
    let mut balloons = parse(file)
        .repeat(100);

    let mut cycle = fluffbolt_cycle();
    let mut part2 = 0;
    while !balloons.is_empty() {
        part2 += 1;
        let fluffbolt = cycle.next().unwrap();
        let balloon1 = balloons.get(0).unwrap();

        if &fluffbolt == balloon1 {
            let another_balloon_in_sight = balloons.len() % 2 == 0;
            if another_balloon_in_sight {
                let second_hit = balloons.len() / 2;
                balloons.remove(second_hit);
            }
        }
        balloons.remove(0);
    }
    println!("Part 2: {}", part2);
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