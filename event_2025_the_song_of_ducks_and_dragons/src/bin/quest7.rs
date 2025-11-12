use std::any::Any;
use std::collections::{HashMap, HashSet};
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("7", part1, part2, part3);
}

fn part1(file: &str) -> String {
    let (names, rules) = parse(file);

    let name = names.into_iter()
        .filter(|name| matches(&rules, name))
        .next()
        .unwrap();
    name.to_string()
}

fn matches(rules: &HashMap<char, Vec<char>>, name: &str) -> bool {
    let mut chars = name.chars();
    let mut curr = chars.next().unwrap();
    while let Some(next) = chars.next() {
        let rule = rules.get(&curr).unwrap();
        if !rule.contains(&next) {
            return false
        }
        curr = next;
    }
    true
}

fn parse(file: &str) -> (Vec<&str>, HashMap<char, Vec<char>>) {
    let mut split = file.split("\n\n");
    let names = split.next().unwrap().split(',').collect::<Vec<_>>();
    let rules = split.next().unwrap()
        .split("\n")
        .map(|rule| {
            let mut split = rule.split(" > ");
            let char = split.next().unwrap()
                .chars().next().unwrap();
            let chars = split.next().unwrap()
                .split(",").map(|ch| ch.chars().next().unwrap())
                .collect::<Vec<_>>();
            (char, chars)
        }).collect::<HashMap<_, _>>();
    (names, rules)
}

fn part2(file: &str) -> usize {
    let (names, rules) = parse(file);

    names.into_iter()
        .enumerate()
        .filter(|(_, name)| matches(&rules, name))
        .map(|(i, _)| i+1)
        .sum()
}

fn path(rules: &HashMap<char, Vec<char>>, list: &mut HashSet<Vec<char>>, prev: Vec<char>) {
    if prev.len() >= 7 {
        list.insert(prev.clone());
    }

    if prev.len() == 11 {
        return;
    }

    let curr = prev.last().unwrap();
    if let Some(nexts) = rules.get(curr) {
        for &next in nexts {
            let mut vec = prev.clone();
            vec.push(next);
            path(rules, list, vec);
        }
    }
}

fn part3(file: &str) -> usize {
    let (prefixes, rules) = parse(file);

    let mut list = HashSet::new();
    for prefix in prefixes {
        if !matches(&rules, prefix) {
            continue;
        }
        path(&rules, &mut list, prefix.chars().collect());
    }
    list.len()
}