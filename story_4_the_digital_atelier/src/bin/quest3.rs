use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest3/input1.txt").unwrap();
    let (width, height, horizontal, vertical) = parse(file);
    let part1 = isolated_tiles(width, height, horizontal, vertical).len();

    println!("Quest 3 part 1: {}", part1);

    let file = read_to_string("input/quest3/input2.txt").unwrap();
    let (width, height, horizontal, vertical) = parse(file);
    let part2tiles = isolated_tiles(width, height, horizontal.clone(), vertical.clone());
    let part2coloring = coloring(width, height, horizontal, vertical);
    let part2truecolor = part2tiles.iter()
        .filter(|pos| part2coloring[*pos])
        .count();
    let part2falsecolor = part2tiles.iter()
        .filter(|pos| !part2coloring[*pos])
        .count();
    let part2 = part2truecolor.max(part2falsecolor);
    println!("Quest 3 part 2: max({}, {})={}", part2truecolor, part2falsecolor, part2);
}

fn parse(file: String) -> (usize, usize, Vec<usize>, Vec<usize>) {
    let lines = file.lines().collect::<Vec<_>>();
    let width = lines[0].replace("width=", "").parse::<usize>().unwrap();
    let height = lines[1].replace("height=", "").parse::<usize>().unwrap();
    let horizontal = lines[2].replace("horizontal-offsets=", "").chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize).cycle().take(width + height).collect::<Vec<_>>();
    let vertical = lines[3].replace("vertical-offsets=", "").chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize).cycle().take(height + width).collect::<Vec<_>>();
    (width, height, horizontal, vertical)
}

fn isolated_tiles(width: usize, height: usize, horizontal: Vec<usize>, vertical: Vec<usize>) -> Vec<(usize, usize)> {
    let mut isolated_tiles = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let top = x % 2 == horizontal[y];
            let bottom = x % 2 == horizontal[y + 1];
            let left = y % 2 == vertical[x];
            let right = y % 2 == vertical[x + 1];
            if top && bottom && left && right {
                isolated_tiles.push((x, y));
            }
        }
    }
    isolated_tiles
}

fn coloring(width: usize, height: usize, horizontal: Vec<usize>, vertical: Vec<usize>) -> HashMap<(usize, usize), bool> {
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            if visited.contains_key(&(x, y)) {
                continue;
            }

            let color = if y == 0 && x == 0 {
                true
            } else if y > 0 {
                !visited[&(x, y - 1)]
            } else {
                !visited[&(x - 1, y)]
            };

            let mut queue = VecDeque::new();
            queue.push_back((x, y));
            while let Some((x, y)) = queue.pop_front() {
                if (visited.contains_key(&(x, y))) {
                    continue;
                }
                visited.insert((x, y), color);

                let top = x % 2 == horizontal[y];
                let bottom = x % 2 == horizontal[y + 1];
                let left = y % 2 == vertical[x];
                let right = y % 2 == vertical[x + 1];

                if !top && y > 0 {
                    queue.push_back((x, y - 1));
                }
                if !bottom && y < height - 1 {
                    queue.push_back((x, y + 1));
                }
                if !left && x > 0 {
                    queue.push_back((x - 1, y));
                }
                if !right && x < width - 1 {
                    queue.push_back((x + 1, y));
                }
            }
        }
    }
    visited
}