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
    let file = read_to_string("input.txt").unwrap();

    let matrix = file.lines()
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
        .collect::<Vec<_>>();

    let scores = matrix.into_iter()
        .map(|x| x.calc())
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", scores.iter().max().unwrap());
}

impl Line {
    fn calc(&self) -> u64 {
        let p1 = eni(self.a, self.x, self.m);
        let p2 = eni(self.b, self.y, self.m);
        let p3 = eni(self.c, self.z, self.m);
        p1 + p2 + p3
    }
}

fn eni(n: u64, exp: u64, mod_: u64) -> u64 {
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