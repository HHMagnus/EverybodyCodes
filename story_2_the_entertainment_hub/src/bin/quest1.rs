use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
}

struct Machine {
    matrix: HashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
}

fn parse(file: String) -> (Machine, Vec<Vec<Direction>>) {
    let mut split = file.split("\n\n");
    let matrix_part = split.next().unwrap();
    let input_part = split.next().unwrap();

    let matrix = matrix_part.lines()
        .enumerate()
        .flat_map(|(y, line)|
            line.chars().enumerate()
                .map(move |(x, ch)| if ch == '*' { ((x as i32, y as i32), true) } else { ((x as i32,y as i32), false) })
        ).collect::<HashMap<_, _>>();

    let max_x = matrix
        .keys()
        .map(|&(x, _)| x)
        .max()
        .unwrap();

    let max_y = matrix
        .keys()
        .map(|&(_, y)| y)
        .max()
        .unwrap();

    let input = input_part.lines()
        .map(|line| line.chars()
            .map(|c| if c == 'R' { Direction::Right } else { Direction::Left })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let machine = Machine {
        matrix,
        max_x,
        max_y,
    };

    (machine, input)
}

impl Machine {
    fn all_plays(&self, direction: Vec<Direction>) -> Vec<(i32, i32)> {
        (0..self.max_x/2+1)
            .map(|toss_slot| (toss_slot, self.play(toss_slot, direction.clone())))
            .collect::<Vec<_>>()
    }

    fn best_play(&self, direction: Vec<Direction>) -> i32 {
        (0..self.max_x/2+1)
            .map(|toss_slot| self.play(toss_slot, direction.clone()))
            .max()
            .unwrap()
    }

    fn play(&self, toss_slot: i32, directions: Vec<Direction>) -> i32 {
        let mut toss = (toss_slot * 2, 0);

        let mut directions = directions.into_iter();

        while toss.1 <= self.max_y {
            if self.matrix[&toss] {
                let dir = directions.next().unwrap();
                match dir {
                    Direction::Right => {
                        let mut x = toss.0 + 1;
                        if x > self.max_x {
                            x = toss.0 - 1;
                        }
                        toss = (x, toss.1);
                    },
                    Direction::Left => {
                        let mut x = toss.0 - 1;
                        if x < 0 {
                            x = toss.0 + 1;
                        }
                        toss = (x, toss.1);
                    }
                }
            } else {
                toss = (toss.0, toss.1 + 1);
            }
        }

        let toss_slot = toss_slot + 1;
        let final_slot = toss.0 / 2 + 1;
        let coins_won = ((final_slot * 2) - toss_slot).max(0);

        coins_won
    }
}

fn main() {
    let file = read_to_string("input/quest1/input1.txt").unwrap();
    let (machine, input) = parse(file);

    let part1 = input.into_iter().enumerate()
        .map(|(toss_slot, directions)| machine.play(toss_slot as i32, directions))
        .sum::<i32>();
    println!("Part 1: {}", part1);

    let file = read_to_string("input/quest1/input2.txt").unwrap();
    let (machine, input) = parse(file);

    let part2 = input.into_iter()
        .map(|directions| machine.best_play(directions))
        .sum::<i32>();
    println!("Part 2: {}", part2);

    let file = read_to_string("input/quest1/input3.txt").unwrap();
    let (machine, input) = parse(file);

    let part3_input = input.into_iter()
        .map(|directions| machine.all_plays(directions))
        .collect::<Vec<_>>();
    let part3_options = part3_input.iter()
        .fold(vec![(Vec::new(), 0)], |acc, x| options(acc, x));
    let part3_min = part3_options.iter()
        .map(|x| x.1)
        .min()
        .unwrap();
    let part3_max = part3_options.iter()
        .map(|x| x.1)
        .max()
        .unwrap();
    println!("Part 3: {} {}", part3_min, part3_max);
}

fn options(vec: Vec<(Vec<i32>, i32)>, input: &Vec<(i32, i32)>) -> Vec<(Vec<i32>, i32)> {
    let mut result = Vec::new();
    for (tosses, total) in vec {
        for (toss, score) in input {
            if (!tosses.contains(&toss)) {
                let mut new_tosses = tosses.clone();
                new_tosses.push(*toss);
                result.push((new_tosses, score + total));
            }
        }
    }
    result
}