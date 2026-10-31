use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let file1 = read_to_string("input/quest5/input1.txt").unwrap();
    let (matrix1, max_x, max_y) = parse(file1);
    let part1land = land(matrix1, max_x, max_y);

    println!("Part 1: {}", part1land.len());

    let file2 = read_to_string("input/quest5/input2.txt").unwrap();
    let (matrix2, max_x, max_y) = parse(file2);
    let part2land = land(matrix2.clone(), max_x, max_y);
    let part2sand = sand(max_x, max_y, part2land);

    println!("Part 2: {}", part2sand.len());

    let file3 = read_to_string("input/quest5/input3.txt").unwrap();
    let (matrix3, max_x, max_y) = parse(file3);
    let part3land = land(matrix3.clone(), max_x, max_y);
    let part3sand = sand(max_x, max_y, part3land.clone());

    let mut trees = HashSet::new();
    for _ in 0..3 {
        let new_trees = part3land.iter()
            .filter(|&&(x, y)| {
                let roots = roots(x, y).into_iter()
                    .filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < max_x as i32 && ny < max_y as i32)
                    .map(|(nx, ny)| (nx as usize, ny as usize))
                    .collect::<Vec<_>>();
                if roots.len() != 20 {
                    return false;
                }

                let all_land = roots.iter()
                    .all(|xy| !part3sand.contains(xy));

                if all_land {
                    return true;
                }

                let four_trees = neighbours(x, y, max_x, max_y).iter()
                    .filter(|&xy| trees.contains(xy))
                    .count() >= 4;

                return four_trees;
            }).cloned()
            .collect::<Vec<(usize, usize)>>();

        trees.extend(new_trees);
    }

    println!("Part 3: {}", trees.len());
}

fn sand(max_x: usize, max_y: usize, part2land: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut sand = HashSet::new();
    loop {
        let new_sand = part2land.iter()
            .filter(|&s| !sand.contains(s))
            .filter(|&&(x, y)| raw_neighbours(x, y)
                .into_iter().any(|(x, y)| x < 0 || y < 0 || x >= max_x as i32 || y >= max_y as i32)
                ||
                !neighbours(x, y, max_x, max_y)
                    .into_iter()
                    .all(|nb| part2land.contains(&nb))
                || neighbours(x, y, max_x, max_y)
                .into_iter()
                .filter(|xy| sand.contains(xy))
                .count() >= 5
            )
            .cloned()
            .collect::<Vec<_>>();

        if new_sand.is_empty() {
            break;
        }
        sand.extend(new_sand);
    }
    sand
}

fn parse(file1: String) -> (Vec<Vec<u32>>, usize, usize) {
    let matrix1 = file1.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let max_x = matrix1[0].len();
    let max_y = matrix1.len();
    (matrix1, max_x, max_y)
}

fn land(matrix1: Vec<Vec<u32>>, max_x: usize, max_y: usize) -> HashSet<(usize, usize)> {
    let central = (max_x / 2, max_y / 2);

    let mut land = HashSet::new();

    land.insert(central);

    loop {
        let new_land = land.iter()
            .flat_map(|&(x, y)| neighbours(x, y, max_x, max_y))
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|neighbour| !land.contains(neighbour))
            .filter(|&(x, y)| neighbours(x, y, max_x, max_y)
                .iter()
                .filter(|neighbour| land.contains(&neighbour) || matrix1[neighbour.1][neighbour.0] == 1)
                .count() >= 5)
            .collect::<Vec<_>>();

        if new_land.is_empty() {
            break;
        }

        new_land.into_iter()
            .for_each(|xy| { land.insert(xy); });
    }
    land
}

fn raw_neighbours(x: usize, y: usize) -> [(i32, i32); 8] {
    let (x, y) = (x as i32, y as i32);
    [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
    ]
}

fn roots(x: usize, y: usize) -> [(i32, i32); 20] {
    let (x, y) = (x as i32, y as i32);
    [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 2, y),
        (x + 2, y),
        (x, y - 2),
        (x, y + 2),
        (x - 1, y - 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 2, y - 1),
        (x + 2, y + 1),
        (x + 2, y - 1),
        (x - 2, y + 1),
        (x - 1, y - 2),
        (x + 1, y + 2),
        (x + 1, y - 2),
        (x - 1, y + 2),
    ]
}

fn neighbours(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    raw_neighbours(x, y).into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .filter(|&(x, y)| x < max_x as i32 && y < max_y as i32)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}