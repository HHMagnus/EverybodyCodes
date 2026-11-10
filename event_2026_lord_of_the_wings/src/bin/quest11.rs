use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest11/input1.txt").unwrap();

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