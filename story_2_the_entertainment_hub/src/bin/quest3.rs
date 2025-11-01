use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Die {
    id: usize,
    faces: Vec<i64>,
    seed: i64,
    pulse: i64,
    roll_number: i64,
    current: i64,
}

impl Die {
    fn parse(file: String) -> Vec<Die> {
        file.lines()
            .map(|line| {
                let mut split = line.split(" ");
                let id = split.next().unwrap().replace(":", "").parse().unwrap();
                let faces = split.next().unwrap()
                    .replace("faces=[", "")
                    .replace("]", "")
                    .split(",")
                    .map(|num| num.parse().unwrap())
                    .collect();
                let seed = split.next().unwrap().replace("seed=", "").parse().unwrap();
                Die {
                    id,
                    faces,
                    seed,
                    pulse: seed,
                    roll_number: 1,
                    current: 0,
                }
            })
            .collect()
    }

    fn roll(&mut self) -> i64 {
        let spin = self.roll_number * self.pulse;
        self.current = (self.current + spin) % self.faces.len() as i64;
        self.pulse += spin;
        self.pulse %= self.seed;
        self.pulse += 1 + self.roll_number + self.seed;
        self.roll_number += 1;
        self.faces[self.current as usize]
    }

    fn sequence(&mut self, sequence: Vec<i64>) -> i64 {
        let mut sequence = sequence;
        sequence.reverse();
        let mut rolls = 0;
        while let Some(roll) = sequence.pop() {
            while self.roll() != roll {
                rolls += 1;
            }
        }
        rolls
    }
}

fn main() {
    let file = read_to_string("input/quest3/input1.txt").unwrap();
    let mut dies = Die::parse(file);

    let mut rolls = 0;
    let mut total = 0;
    while total < 10000 {
        total += dies.iter_mut().map(|die| die.roll()).sum::<i64>();
        rolls += 1;
    }

    println!("Part 1: {}", rolls);

    let file = read_to_string("input/quest3/input2.txt").unwrap();
    let mut split = file.split("\n\n");
    let mut dies = Die::parse(split.next().unwrap().to_string());
    let sequence = split.next().unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let mut scores = dies.iter_mut()
        .map(|die| (die.id, die.sequence(sequence.clone())))
        .collect::<Vec<(usize, i64)>>();
    scores.sort_by_key(|x| x.1);
    let scores = scores.into_iter()
        .map(|x| x.0.to_string())
        .collect::<Vec<_>>();
    let part2 = scores.join(",");
    println!("Part 2: {}", part2);
}