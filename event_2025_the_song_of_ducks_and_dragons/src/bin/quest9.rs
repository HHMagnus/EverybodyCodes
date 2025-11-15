use std::collections::HashSet;
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("9", part1, part2, part3);
}

fn part1(file: &str) -> usize {
    let ducks = parse(file);

    let parent1 = &ducks[0].1;
    let parent2 = &ducks[1].1;
    let child = &ducks[2].1;

    similarity(child, parent1, parent2).unwrap()
}

fn parse(file: &str) -> Vec<(usize, Vec<char>)> {
    file.split('\n')
        .map(|line| {
            let mut split = line.split(':');
            let id = split.next().unwrap()
                .parse::<usize>().unwrap();
            let chars = split.next().unwrap()
                .chars().collect::<Vec<char>>();
            (id, chars)
        }).collect::<Vec<_>>()
}

fn similarity(child: &Vec<char>, parent1: &Vec<char>, parent2: &Vec<char>) -> Option<usize> {
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0..child.len() {
        let x = child[i];
        let y1 = parent1[i];
        let y2 = parent2[i];
        if x != y1 && x != y2 {
            return None;
        }

        if x == y1 {
            p1 += 1;
        }
        if x == y2 {
            p2 += 1;
        }
    }
    Some(p1 * p2)
}

fn matches(x: &Vec<char>, y: &Vec<char>, z: &Vec<char>) -> Option<usize> {
    similarity(x, y, z)
        .or_else(|| similarity(y, x, z))
        .or_else(|| similarity(z, y, x))
}

fn part2(file: &str) -> usize {
    let ducks = parse(file)
        .into_iter()
        .map(|x| x.1)
        .collect::<Vec<_>>();

    let mut result = 0;
    for x in 0..ducks.len() {
        for y in x+1..ducks.len() {
            for z in y+1..ducks.len() {
                if let Some(compared) = matches(&ducks[x], &ducks[y], &ducks[z]) {
                    result += compared;
                }
            }
        }
    }
    result
}

fn part3(file: &str) -> usize {
    let ducks = parse(file);

    let mut groups = Vec::new();
    for x in 0..ducks.len() {
        for y in x+1..ducks.len() {
            for z in y+1..ducks.len() {
                if let Some(_) = matches(&ducks[x].1, &ducks[y].1, &ducks[z].1) {
                    let mut set = HashSet::new();
                    set.insert(ducks[x].0);
                    set.insert(ducks[y].0);
                    set.insert(ducks[z].0);
                    groups.push(set);
                }
            }
        }
    }

    let mut group_size = 0;
    while group_size != groups.iter().map(|x| x.len()).max().unwrap() {
        group_size = groups.iter().map(|x| x.len()).max().unwrap();
        groups = groups.iter()
            .fold(groups.clone(), |mut acc: Vec<HashSet<usize>>, group| {
                for g in acc.iter_mut() {
                    if group.iter().any(|i| g.contains(&i)) {
                        for c in group {
                            g.insert(*c);
                        }
                    }
                }
                acc
            });
    }

    groups.into_iter()
        .max_by_key(|i| i.len())
        .unwrap()
        .into_iter()
        .sum()
}