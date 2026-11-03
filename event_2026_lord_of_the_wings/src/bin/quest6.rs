use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

fn main() {
    part1();

    part2();

    part3();
}

fn part3() {
    let file = read_to_string("input/quest6/input3.txt").unwrap();
    let parsed = parse(&file);

    let uses_null_v1 = uses_of(&parsed, &"NULLv1");
    let uses_null_v2 = uses_of(&parsed, &"NULLv2");
    let uses_null_v3 = uses_of(&parsed, &"NULLv3");

    let total = parsed.into_iter()
        .map(|(n, _)| n)
        .filter(|n| {
            let c1 = if uses_null_v1.contains(&n) { 1 } else { 0 };
            let c2 = if uses_null_v2.contains(&n) { 1 } else { 0 };
            let c3 = if uses_null_v3.contains(&n) { 1 } else { 0 };
            c1 + c2 + c3 >= 2
        }).count();

    println!("Part 3: {}", total);
}

fn part2() {
    let file = read_to_string("input/quest6/input2.txt").unwrap();
    let parsed = parse(&file);

    let null = "NULL";

    let total = uses_of(&parsed, &null);

    println!("Part 2: {}", total.len() - 1);
}

fn uses_of<'a>(parsed: &HashMap<&'a str, Vec<&str>>, null: &&'a str) -> Vec<&'a str> {
    let mut cands = vec![null];

    let mut total = Vec::new();

    while let Some(cand) = cands.pop() {
        if total.contains(cand) {
            continue;
        }

        total.push(cand);

        let search = parsed.iter()
            .filter(|(_, i)| i.contains(cand))
            .map(|(i, _)| i);

        cands.extend(search);
    }
    total
}

fn part1() {
    let file = read_to_string("input/quest6/input1.txt").unwrap();
    let parsed = parse(&file);

    let mut total: Vec<&str> = Vec::new();

    let mut queue = VecDeque::new();

    queue.push_front(&"NULL");

    while let Some(x) = queue.pop_front() {
        if total.contains(x) {
            continue;
        }

        total.push(*x);

        if let Some(ingredients) = parsed.get(x) {
            queue.extend(ingredients.iter())
        }
    }

    println!("Part 1: {}", total.len() - 1);
}

fn parse(file: &str) -> HashMap<&str, Vec<&str>> {
    file.lines()
        .map(|l| {
            let mut split = l.split(":");
            (split.next().unwrap(), split.next().unwrap().split(",").collect::<Vec<_>>())
        }).collect::<HashMap<_, _>>()
}