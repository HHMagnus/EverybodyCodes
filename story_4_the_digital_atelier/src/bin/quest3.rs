use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest3/input1.txt").unwrap();
    let lines = file.lines().collect::<Vec<_>>();
    let width = lines[0].replace("width=", "").parse::<usize>().unwrap();
    let height = lines[1].replace("height=", "").parse::<usize>().unwrap();
    let horizontal = lines[2].replace("horizontal-offsets=", "").chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize).cycle().take(width+height).collect::<Vec<_>>();
    let vertical = lines[3].replace("vertical-offsets=", "").chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize).cycle().take(height+width).collect::<Vec<_>>();

    let mut part1 = 0;
    for x in 0..width {
        for y in 0..height {
            let top = x % 2 == horizontal[y];
            let bottom = x % 2 == horizontal[y+1];
            let left = y % 2 == vertical[x];
            let right = y % 2 == vertical[x+1];
            if top && bottom && left && right {
                part1 += 1;
            }
        }
    }

    println!("Quest 3 part 1: {}", part1);
}