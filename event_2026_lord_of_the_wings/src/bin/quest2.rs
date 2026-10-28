use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let file1 = read_to_string("input/quest2/input1.txt").unwrap();
    let part1 = checksum(file1);
    println!("Part 1: {}", part1);

    let file2 = read_to_string("input/quest2/input2.txt").unwrap();
    let part2 = checksum(file2);
    println!("Part 2: {}", part2);

    let file3 = read_to_string("input/quest2/input3.txt").unwrap();
    let part3 = checksum(file3);
    println!("Part 3: {}", part3);
}

fn checksum(file1: String) -> u64 {
    file1.lines()
        .map(|x| {
            x.split(" ").map(|rule| {
                if rule.contains("<") {
                    let mut split = rule.split("<");
                    let x1 = split.next().unwrap().parse::<u32>().unwrap();
                    let x2 = split.next().unwrap().parse::<u32>().unwrap();
                    (x2, x1)
                } else {
                    let mut split = rule.split(">");
                    let x1 = split.next().unwrap().parse::<u32>().unwrap();
                    let x2 = split.next().unwrap().parse::<u32>().unwrap();
                    (x1, x2)
                }
            }).collect::<HashSet<(u32, u32)>>()
        })
        .map(|mut greater_then| {
            let mut nums = greater_then.iter()
                .flat_map(|&(x1, x2)| vec![x1, x2])
                .collect::<HashSet<u32>>()
                .into_iter()
                .collect::<Vec<u32>>();
            let mut missing_rules = nums.iter()
                .flat_map(|&x| nums.iter().map(move |&y| (x, y)))
                .filter(|&(x, y)| x != y && !greater_then.contains(&(x, y)) && !greater_then.contains(&(y, x)))
                .collect::<Vec<(u32, u32)>>();
            while !missing_rules.is_empty() {
                let mut still_missing = Vec::new();
                for rule in missing_rules.into_iter() {
                    let reversed_rule = (rule.1, rule.0);
                    if can_deduct(&greater_then, &rule) {
                        greater_then.insert(rule);
                    } else if can_deduct(&greater_then, &reversed_rule) {
                        greater_then.insert(reversed_rule);
                    } else {
                        still_missing.push(rule);
                    }
                }
                missing_rules = still_missing
            }
            nums.sort_by(|x, y| {
                if greater_then.contains(&(x.clone(), y.clone())) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            nums.into_iter()
                .map(|x| x.to_string())
                .collect::<String>()
                .parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

fn can_deduct(existing_rules: &HashSet<(u32, u32)>, rule: &(u32, u32)) -> bool {
    // rule.0 > x && x > rule.1 == rule.0 > rule.1
    existing_rules.iter()
        .filter(|&(rule0, _)| rule.0 == *rule0)
        .any(|&(_, x1)| existing_rules.iter().any(
            |&(x2, y)| x1 == x2 && y == rule.1
        ))
}