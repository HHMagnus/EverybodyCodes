use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;

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

    let starts = (0..max_x).filter(|x| x % 2 == 0)
        .map(|x| (x, -1))
        .collect::<Vec<_>>();

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
}