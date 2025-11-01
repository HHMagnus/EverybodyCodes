use std::cmp::PartialEq;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Balloon {
    R,
    G,
    B
}

fn main() {
    let file = read_to_string("input/quest2/input1.txt").unwrap();

    let mut balloons = file.chars()
        .map(|c| match c {
            'R' => Balloon::R,
            'G' => Balloon::G,
            'B' => Balloon::B,
            _ => panic!("No balloon"),
        })
        .collect::<Vec<Balloon>>();
    balloons.reverse();

    let mut cycle = vec![Balloon::R, Balloon::G, Balloon::B].into_iter().cycle();

    let mut part1 = 0;

    while !balloons.is_empty() {
        part1 += 1;

        let fluffbolt = cycle.next().unwrap();

        while let (Some(b)) = balloons.pop() && b == fluffbolt {

        }
    }

    println!("Part 1: {}", part1);
}