use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest1/input1.txt").unwrap();
    let part1 = file.lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>())
        .map(|nums| {
            let mut visited = HashSet::new();
            visited.insert(0);
            let mut curr = 0;
            for num in nums {
                let min = curr - num;
                if min >= 0 && !visited.contains(&min) {
                    curr = min;
                } else {
                    curr = curr + num;
                }
                visited.insert(curr);
            }
            curr
        })
        .sum::<i32>();

    println!("Quest 1 part 1: {}", part1);

    let file = read_to_string("input/quest1/input2.txt").unwrap();
    let part2 = file.lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>())
        .map(|nums| {
            let mut visited = HashSet::new();
            visited.insert(0);
            let mut curr = 0;
            for num in nums {
                let min = curr - num;
                if min >= 0 && !visited.contains(&min) {
                    curr = min;
                } else {
                    curr = curr + num;
                    while visited.contains(&curr) {
                        curr += 1;
                    }
                }
                visited.insert(curr);
            }
            curr
        })
        .sum::<i32>();

    println!("Quest 1 part 2: {}", part2);

    let file = read_to_string("input/quest1/input3.txt").unwrap();
    let part3 = file.lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>())
        .map(|nums| {
            let mut visited = HashSet::new();
            let mut top_arcs = HashSet::new();
            let mut bottom_arcs = HashSet::new();
            visited.insert(0);
            let mut curr = 0;
            let mut top_arc = false;
            for num in nums {
                let min = curr - num;
                let arc = if top_arc { &mut top_arcs } else { &mut bottom_arcs };
                if min > 0 && !visited.contains(&min) && !arc.iter()
                    .any(|b: &(i32, i32)| (b.0 < min && b.1 > min && b.1 < curr) || (b.0 > min && b.0 < curr && b.1 > curr)) {
                    arc.insert((min, curr));
                    visited.insert(min);
                    curr = min;
                    top_arc = !top_arc;
                    continue;
                }

                let mut max = curr + num;

                loop {
                    if visited.contains(&max) {
                        max += 1;
                        continue;
                    }

                    let leaving = arc.iter()
                        .any(|l: &(i32, i32)| l.0 < curr && l.1 > curr && l.1 < max);
                    if leaving {
                        break;
                    }

                    if let Some(into) = arc.iter()
                        .find(|l: &&(i32, i32)| l.0 > curr && l.0 < max && l.1 > max) {
                        max = into.1 + 1;
                        continue;
                    }

                    arc.insert((curr, max));
                    visited.insert(max);
                    curr = max;
                    top_arc = !top_arc;
                    break;
                }
            }
            curr
        })
        .sum::<i32>();

    println!("Quest 1 part 3: {}", part3);
}