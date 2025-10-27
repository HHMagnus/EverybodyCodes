use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Formula {
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
    let part1 = Formula::solve(file, eni1);
    println!("Part 1: {}", part1);

    let file = read_to_string("input2.txt").unwrap();
    let part2 = Formula::solve(file, eni2);
    println!("Part 2: {}", part2);

    let file = read_to_string("input3.txt").unwrap();
    let part3 = Formula::solve(file, eni3);
    println!("Part 3: {}", part3);
}

fn parse_input_file(file: String) -> Vec<Formula> {
    file.lines()
        .map(|line| line.split(" ")
            .map(|word| word.split("=").last().unwrap())
            .collect::<Vec<&str>>())
        .map(|vec| Formula {
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

impl Formula {
    fn solve<F>(file: String, f: F) -> u64
    where
        F: Fn(u64, u64, u64) -> u64,
        F: Copy
    {
        let formulas = parse_input_file(file);
        *formulas.into_iter()
            .map(|formula| formula.calc(f))
            .collect::<Vec<_>>()
            .iter()
            .max()
            .unwrap()
    }

    fn calc<F>(&self, f: F) -> u64
    where
        F: Fn(u64, u64, u64) -> u64,
    {
        let p1 = f(self.a, self.x, self.m);
        let p2 = f(self.b, self.y, self.m);
        let p3 = f(self.c, self.z, self.m);
        p1 + p2 + p3
    }
}

fn eni1(n: u64, exp: u64, mod_: u64) -> u64 {
    let mut remainders = Vec::new();
    let mut score = 1;
    for _ in 0..exp {
        score = (score * n) % mod_;
        remainders.push(score);
    }
    remainders.reverse();
    remainders_to_number(remainders)
}

fn remainders_to_number(remainders: Vec<u64>) -> u64 {
    remainders.into_iter()
        .map(|i|i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn eni2(n: u64, exp: u64, mod_: u64) -> u64 {
    let mut score_to_iteration_seen = HashMap::new();
    let mut remainders = Vec::new();
    let mut score = 1;
    let mut remaining_iterations = exp;
    let mut previously_used_cycle = false;
    while remaining_iterations > 0 {
        score = (score * n) % mod_;
        if remaining_iterations <= 5 {
            remainders.push(score);
        }
        if remaining_iterations > 5
            && !previously_used_cycle
            && score_to_iteration_seen.contains_key(&score)
        {
            let cycle_start_iterations = score_to_iteration_seen.get(&score).unwrap();
            let cycle_size = cycle_start_iterations - remaining_iterations;
            remaining_iterations %= cycle_size;
            while remaining_iterations <= 5 {
                remaining_iterations += cycle_size;
            }
            previously_used_cycle = true;
        } else {
            score_to_iteration_seen.insert(score, remaining_iterations);
        }
        remaining_iterations -= 1;
    }
    remainders.reverse();
    remainders_to_number(remainders)
}

fn eni3(n: u64, exp: u64, mod_: u64) -> u64 {
    let mut score_to_iteration_seen = HashMap::new();
    let mut iterations_to_remainder = HashMap::new();
    let mut score = 1;
    let mut remaining_iterations = exp;
    let mut previously_used_cycle = false;
    let mut total_sum = 0;
    while remaining_iterations > 0 {
        score = (score * n) % mod_;
        iterations_to_remainder.insert(remaining_iterations, score);
        if remaining_iterations > 5
            && !previously_used_cycle
            && score_to_iteration_seen.contains_key(&score)
        {
            let cycle_start_iterations = *score_to_iteration_seen.get(&score).unwrap();
            let cycle_size = cycle_start_iterations - remaining_iterations;
            let mut cycle_sum = 0;
            for i in remaining_iterations..cycle_start_iterations {
                cycle_sum += iterations_to_remainder.get(&i).unwrap();
            }
            let mut cycle_usage = remaining_iterations / cycle_size;
            remaining_iterations %= cycle_size;
            while remaining_iterations <= 5 {
                remaining_iterations += cycle_size;
                cycle_usage -= 1;
            }
            total_sum += cycle_sum * cycle_usage;
            previously_used_cycle = true;
        } else {
            score_to_iteration_seen.insert(score, remaining_iterations);
        }
        total_sum += score;
        remaining_iterations -= 1;
    }
    total_sum
}