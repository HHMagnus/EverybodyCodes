use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;

enum Direction {
    Right,
    Left,
}

struct Machine {
    matrix: HashMap<(i32, i32), Direction>,
}

fn main() {
    let file = read_to_string("input/quest1/input1.txt").unwrap();
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

    let mut coins = 0;

    for (toss_slot, directions) in input.into_iter().enumerate() {
        let toss_slot = toss_slot as i32;
        let mut toss = (toss_slot * 2, 0);

        let mut directions = directions.into_iter();

        while toss.1 <= max_y {
            if matrix[&toss] {
                let dir = directions.next().unwrap();
                match dir {
                    Direction::Right => {
                        let mut x = toss.0 + 1;
                        if x > max_x {
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

        println!("{} {} {}", toss_slot, final_slot, coins_won);
        coins += coins_won;
    }

    println!("Part 1: {}", coins);
}