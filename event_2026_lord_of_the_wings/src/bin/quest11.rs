use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

fn main() {
    part1();
    part2();
    part3();
}

fn part1() {
    let file = read_to_string("input/quest11/input1.txt").unwrap();

    let (portals, shortcuts) = parse(file);

    let mut connections = (1..=portals).into_iter().collect::<Vec<_>>()
        .into_iter()
        .as_slice()
        .windows(2)
        .map(|line| (line[0], line[1]))
        .collect::<Vec<_>>();
    connections.push((1, portals));

    let strain_before = bfs(1, &connections) * portals;

    connections.extend(shortcuts);

    let strain_after = (1..=portals).into_iter()
        .map(|x| bfs(x, &connections))
        .sum::<u32>();

    let resonance = strain_before - strain_after;

    println!("Part 1: {} - {} = {}", strain_before, strain_after, resonance);
}

fn part2() {
    let file = read_to_string("input/quest11/input2.txt").unwrap();

    let (portals, shortcuts) = parse(file);

    let mut connections = three_apart_ring(portals);

    let strain_before = bfs(1, &connections) * portals;

    connections.extend(shortcuts);

    let mut strain_after = u32::MAX;

    for i in 1..=portals {
        for j in i+1..=portals {
            if connections.contains(&(i, j)) || connections.contains(&(j, i)) {
                continue;
            }
            let mut con = connections.clone();
            con.push((i, j));

            let strain = (1..=portals).into_iter()
                .map(|x| bfs(x, &con))
                .sum::<u32>();

            strain_after = strain_after.min(strain);
        }
    }

    let resonance = strain_before - strain_after;

    println!("Part 2: {} - {} = {}", strain_before, strain_after, resonance);
}

fn three_apart_ring(portals: u32) -> Vec<(u32, u32)> {
    let mut connections = (1..=portals).into_iter().collect::<Vec<_>>()
        .into_iter()
        .as_slice()
        .windows(4)
        .flat_map(|line| vec![(line[0], line[1]), (line[0], line[2]), (line[0], line[3])])
        .collect::<Vec<_>>();
    connections.push((1, portals));
    connections.push((1, portals - 1));
    connections.push((1, portals - 2));
    connections.push((2, portals));
    connections.push((2, portals - 1));
    connections.push((3, portals));
    connections.push((portals, portals - 2));
    connections.push((portals, portals - 1));
    connections.push((portals - 1, portals - 2));
    connections
}

fn part3() {
    let file = read_to_string("input/quest11/input3.txt").unwrap();

    let (portals, shortcuts) = parse(file);

    let mut connections = three_apart_ring(portals);

    let strain_before = bfs(1, &connections) * portals;

    connections.extend(shortcuts);

    let mut strain_after = u32::MAX;

    let crystal_separation = portals / 8;

    for offset in 1..=8 {
        let crystals = (0..8).into_iter()
            .map(|x| x * crystal_separation + offset)
            .collect::<Vec<_>>();

        for pairing in pairings(&crystals) {
            let mut con = connections.clone();
            con.extend(pairing);

            let strain = (1..=portals).into_iter()
                .map(|x| bfs(x, &con))
                .sum::<u32>();

            strain_after = strain_after.min(strain);
        }
    }

    let resonance = strain_before - strain_after;

    println!("Part 3: {} - {} = {}", strain_before, strain_after, resonance);
}

fn pairings(crystals: &Vec<u32>) -> Vec<Vec<(u32, u32)>> {
    if crystals.len() < 2 {
        return vec![vec![]];
    }

    let first = crystals[0];
    let mut result = Vec::new();

    for i in 1..crystals.len() {
        let mut rest = crystals[1..].to_vec();
        let partner = rest.remove(i - 1);

        for mut tail in pairings(&rest) {
            tail.insert(0, (first, partner));
            result.push(tail);
        }
    }

    result
}

fn parse(file: String) -> (u32, Vec<(u32, u32)>) {
    let mut split = file.split("\n\n");
    let portals = split.next().unwrap()
        .replace("portals: ", "")
        .parse::<u32>()
        .unwrap();
    let shortcuts = split.next().unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            (split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap())
        }).collect::<Vec<_>>();
    (portals, shortcuts)
}

fn bfs(start: u32, connections: &Vec<(u32, u32)>) -> u32 {
    let mut visited = HashMap::new();
    let mut next = VecDeque::new();
    next.push_back((start, 0));

    while let Some((next_portal, steps)) = next.pop_front() {
        if visited.contains_key(&next_portal) {
            continue;
        }
        visited.insert(next_portal, steps);

        connections
            .iter()
            .filter(|&(x, _)| *x == next_portal)
            .map(|&(_, y)| y)
            .filter(|&y| !visited.contains_key(&y))
            .for_each(|y| next.push_back((y, steps + 1)));

        connections
            .iter()
            .filter(|&(_, y)| *y == next_portal)
            .map(|&(x, _)| x)
            .filter(|&x| !visited.contains_key(&x))
            .for_each(|x| next.push_back((x, steps + 1)));
    }

    visited.values().sum()
}