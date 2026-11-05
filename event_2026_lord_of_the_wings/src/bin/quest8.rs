use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest8/input1.txt").unwrap();
    let matrix = parse(file);
    let part1 = down_right(&matrix);

    println!("Part 1: {}", part1);

    let file = read_to_string("input/quest8/input2.txt").unwrap();
    let matrix = parse(file);
    let part2 = down_right(&matrix)
        + down_left(&matrix)
        + run_left(&matrix)
        + up_left(&matrix)
        + up_right(&matrix)
        + run_right(&matrix);

    println!("Part 2: {}", part2);
}

fn top(matrix: &Vec<Vec<Option<i32>>>) -> Vec<(i32, i32)> {
    (0..matrix[0].len() as i32).into_iter()
        .map(|x| (x, 0))
        .collect()
}

fn bottom(matrix: &Vec<Vec<Option<i32>>>) -> Vec<(i32, i32)> {
    (0..matrix[0].len() as i32).into_iter()
        .map(|x| (x, matrix.len() as i32 - 1))
        .collect()
}

fn right(matrix: &Vec<Vec<Option<i32>>>) -> Vec<(i32, i32)> {
    (0..matrix.len() as i32).into_iter()
        .map(|y| (matrix[y as usize].len() as i32 - 1, y))
        .collect()
}

fn left(matrix: &Vec<Vec<Option<i32>>>) -> Vec<(i32, i32)> {
    (0..matrix.len() as i32).into_iter()
        .map(|y| (0, y))
        .collect()
}

fn down_right(matrix: &Vec<Vec<Option<i32>>>) -> u32 {
    let direction = (1, 1);
    let mut left = left(matrix);
    let top = top(matrix);
    left.extend(top);
    run_rays(left, direction, matrix)
}

fn down_left(matrix: &Vec<Vec<Option<i32>>>) -> u32 {
    let direction = (-1, 1);
    let mut right = right(matrix);
    let top = top(matrix);
    right.extend(top);
    run_rays(right, direction, matrix)
}

fn run_left(matrix: &Vec<Vec<Option<i32>>>) -> u32 {
    let direction = (-1, 0);
    run_rays(right(matrix), direction, matrix)
}

fn up_left(matrix: &Vec<Vec<Option<i32>>>) -> u32 {
    let direction = (-1, -1);
    let mut right = right(matrix);
    let bottom = bottom(matrix);
    right.extend(bottom);
    run_rays(right, direction, matrix)
}

fn up_right(matrix: &Vec<Vec<Option<i32>>>) -> u32 {
    let direction = (1, -1);
    let mut left = left(matrix);
    let bottom = bottom(matrix);
    left.extend(bottom);
    run_rays(left, direction, matrix)
}

fn run_right(matrix: &Vec<Vec<Option<i32>>>) -> u32 {
    let direction = (1, 0);
    run_rays(left(&matrix), direction, matrix)
}


fn parse(file: String) -> Vec<Vec<Option<i32>>> {
    file.lines()
        .map(|line| line.chars()
            .map(|ch| ch.to_digit(10).map(|x| x as i32))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn run_rays(mut starts: Vec<(i32, i32)>, direction: (i32, i32), matrix: &[Vec<Option<i32>>]) -> u32 {
    starts.sort();
    starts.dedup();
    starts.into_iter()
        .map(|(x, y)| ray_hit((x, y), direction, &matrix, 0))
        .sum()
}

fn ray_hit((x, y): (i32, i32), direction: (i32, i32), matrix: &[Vec<Option<i32>>], shadow: i32) -> u32 {
    if x < 0 || y < 0 || matrix.len() as i32 <= y || matrix[y as usize].len() as i32 <= x {
        return 0;
    }

    let this = matrix[y as usize][x as usize];

    let next = (x + direction.0, y + direction.1);

    if this.is_none() {
        return ray_hit(next, direction, matrix, shadow);
    }

    let this = this.unwrap();

    if this > shadow {
        1 + ray_hit(next, direction, matrix, this - 1)
    } else {
        0 + ray_hit(next, direction, matrix, shadow - 1)
    }
}