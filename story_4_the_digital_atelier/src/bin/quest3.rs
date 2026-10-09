use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest3/input1.txt").unwrap();
    let (width, height, horizontal, vertical) = parse(file);
    let part1 = isolated_tiles(
        width,
        height,
        horizontal.into_iter().cycle().take(height+width).collect(),
        vertical.into_iter().cycle().take(height+width).collect()
    ).len();

    println!("Quest 3 part 1: {}", part1);

    let file = read_to_string("input/quest3/input2.txt").unwrap();
    let part2 = part2and3(file);
    println!("Quest 3 part 2: {}", part2);

    let file = read_to_string("input/quest3/input3.txt").unwrap();
    let part3 = part2and3(file);
    println!("Quest 3 part 3: {}", part3);
}

fn part2and3(file: String) -> usize {
    let (width, height, horizontal, vertical) = parse(file);
    let v2 = vertical.len()*2;
    let h2 = horizontal.len()*2;
    let tiles = isolated_tiles(v2, h2, horizontal.clone(), vertical.clone());
    let coloring = coloring(v2, h2, horizontal, vertical);
    let wtimes = width / v2;
    let wleft = width % v2;
    let htimes = height / h2;
    let hleft = height % h2;
    let count = |c: bool| -> usize {
        let full = tiles.iter().filter(|p| coloring[*p] == c).count();
        let col = tiles.iter().filter(|p| coloring[*p] == c && p.0 < wleft).count();
        let row = tiles.iter().filter(|p| coloring[*p] == c && p.1 < hleft).count();
        let corner = tiles.iter().filter(|p| coloring[*p] == c && p.0 < wleft && p.1 < hleft).count();
        full * wtimes * htimes + col * htimes + row * wtimes + corner
    };
    let true_count = count(true);
    let false_count = count(false);
    true_count.max(false_count)
}

fn parse(file: String) -> (usize, usize, Vec<usize>, Vec<usize>) {
    let lines = file.lines().collect::<Vec<_>>();
    let width = lines[0].replace("width=", "").parse::<usize>().unwrap();
    let height = lines[1].replace("height=", "").parse::<usize>().unwrap();
    let horizontal = lines[2].replace("horizontal-offsets=", "").chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
    let vertical = lines[3].replace("vertical-offsets=", "").chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
    (width, height, horizontal, vertical)
}

fn isolated_tiles(width: usize, height: usize, horizontal: Vec<usize>, vertical: Vec<usize>) -> Vec<(usize, usize)> {
    let mut isolated_tiles = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let top = x % 2 == horizontal[y % horizontal.len()];
            let bottom = x % 2 == horizontal[(y + 1) % horizontal.len()];
            let left = y % 2 == vertical[x % vertical.len()];
            let right = y % 2 == vertical[(x + 1) % vertical.len()];
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
                if visited.contains_key(&(x, y)) {
                    continue;
                }
                visited.insert((x, y), color);

                let top = x % 2 == horizontal[y % horizontal.len()];
                let bottom = x % 2 == horizontal[(y + 1) % horizontal.len()];
                let left = y % 2 == vertical[x % vertical.len()];
                let right = y % 2 == vertical[(x + 1) % vertical.len()];

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