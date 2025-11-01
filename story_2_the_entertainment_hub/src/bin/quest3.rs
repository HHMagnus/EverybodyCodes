use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Die {
    id: usize,
    faces: Vec<i32>,
    seed: i32,
    pulse: i32,
    roll_number: i32,
    current: i32,
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

    fn roll(&mut self) -> i32 {
        let spin = self.roll_number * self.pulse;
        self.current = (self.current + spin) % self.faces.len() as i32;
        self.pulse += spin;
        self.pulse %= self.seed;
        self.pulse += 1 + self.roll_number + self.seed;
        self.roll_number += 1;
        self.faces[self.current as usize]
    }
}

fn main() {
    let file = read_to_string("input/quest3/input1.txt").unwrap();
    let mut dies = Die::parse(file);

    let mut rolls = 0;
    let mut total = 0;
    while total < 10000 {
        total += dies.iter_mut().map(|die| die.roll()).sum::<i32>();
        rolls += 1;
    }

    println!("Part 1: {}", rolls);
}