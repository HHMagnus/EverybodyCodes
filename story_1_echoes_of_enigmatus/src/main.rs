use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

#[derive(Debug)]
struct Line {
    a: u64,
    b: u64,
    c: u64,
    x: u64,
    y: u64,
    z: u64,
    m: u64,
}

fn main() {
    let file = read_to_string("input1.txt").unwrap();
    let matrix = input(file);
    let part1 = matrix.into_iter()
        .map(|x| x.calc1())
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", part1.iter().max().unwrap());

    let file = read_to_string("input2.txt").unwrap();
    let matrix = input(file);
    let part2 = matrix.into_iter()
        .map(|x| x.calc2())
        .collect::<Vec<_>>();

    println!("Part 2: {:?}", part2.iter().max().unwrap());

    let file = read_to_string("input3.txt").unwrap();
    let matrix = input(file);
    let part3 = matrix.into_iter()
        .map(|x| x.calc3())
        .collect::<Vec<_>>();

    println!("Part 3: {:?}", part3.iter().max().unwrap());
}

fn input(file: String) -> Vec<Line> {
    file.lines()
        .map(|line| line.split(" ")
            .map(|word| word.split("=").last().unwrap())
            .collect::<Vec<&str>>())
        .map(|vec| Line {
            a: vec[0].parse().unwrap(),
            b: vec[1].parse().unwrap(),
            c: vec[2].parse().unwrap(),
            x: vec[3].parse().unwrap(),
            y: vec[4].parse().unwrap(),
            z: vec[5].parse().unwrap(),
            m: vec[6].parse().unwrap(),
        })
        .collect::<Vec<_>>()
}

impl Line {
    fn calc1(&self) -> u64 {
        let p1 = eni1(self.a, self.x, self.m);
        let p2 = eni1(self.b, self.y, self.m);
        let p3 = eni1(self.c, self.z, self.m);
        p1 + p2 + p3
    }

    fn calc2(&self) -> u64 {
        let p1 = eni2(self.a, self.x, self.m);
        let p2 = eni2(self.b, self.y, self.m);
        let p3 = eni2(self.c, self.z, self.m);
        p1 + p2 + p3
    }

    fn calc3(&self) -> u64 {
        let p1 = eni3(self.a, self.x, self.m);
        let p2 = eni3(self.b, self.y, self.m);
        let p3 = eni3(self.c, self.z, self.m);
        p1 + p2 + p3
    }
}

fn eni1(n: u64, exp: u64, mod_: u64) -> u64 {
    let mut remainder = Vec::new();
    let mut score = 1;
    for _ in 0..exp {
        score = (score * n) % mod_;
        remainder.push(score);
    }
    remainder.reverse();
    remainder.into_iter()
        .map(|i|i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn eni2(n: u64, exp: u64, mod_: u64) -> u64 {
    let mut prev = HashMap::new();
    let mut remainder = Vec::new();
    let mut score = 1;
    let mut iterations = exp;
    let mut skip = false;
    while iterations > 0 {
        score = (score * n) % mod_;
        if iterations <= 5 {
            remainder.push(score);
        }
        if iterations > 5 && !skip && prev.contains_key(&score) {
            let prev_i = prev.get(&score).unwrap();
            let dist = prev_i - iterations;
            iterations %= dist;
            while iterations <= 5 {
                iterations += dist;
            }
            skip = true;
        } else {
            prev.insert(score, iterations);
        }
        iterations -= 1;
    }
    remainder.reverse();
    remainder.into_iter()
        .map(|i|i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn eni3(n: u64, exp: u64, mod_: u64) -> u64 {
    let mut prev = HashMap::new();
    let mut remainder = HashMap::new();
    let mut score = 1;
    let mut iterations = exp;
    let mut skip = false;
    let mut sum = 0;
    while iterations > 0 {
        score = (score * n) % mod_;
        remainder.insert(iterations, score);
        if iterations > 5 && !skip && prev.contains_key(&score) {
            let prev_i = *prev.get(&score).unwrap();
            let dist = prev_i - iterations;
            let mut dist_sum = 0;
            for i in iterations..prev_i {
                dist_sum += remainder.get(&i).unwrap();
            }
            let mut dist_added = iterations / dist;
            iterations %= dist;
            while iterations <= 5 {
                iterations += dist;
                dist_added -= 1;
            }
            sum += dist_sum * dist_added;
            skip = true;
        } else {
            prev.insert(score, iterations);
        }
        sum += score;
        iterations -= 1;
    }
    sum
}