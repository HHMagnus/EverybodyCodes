use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest2/input1.txt").unwrap();
    let input = parse(file);

    let mut part1 = (0, 0);
    for _ in 0..3 {
        part1 = cycle(part1, input, 10);
    }

    println!("Part 1: [{},{}]", part1.0, part1.1);

    let file = read_to_string("input/quest2/input2.txt").unwrap();
    let input = parse(file);

    let mut part2 = 0;
    for y in (input.1..=input.1+1000).step_by(10) {
        for x in (input.0..=input.0+1000).step_by(10) {
            let point = (x, y);
            if cycle100(point, 100000) {
                part2 += 1;
            }
        }
    }
    println!("Part 2: {}", part2);

    let file = read_to_string("input/quest2/input3.txt").unwrap();
    let input = parse(file);

    let mut part2 = 0;
    for y in input.1..=input.1+1000 {
        for x in input.0..=input.0+1000 {
            let point = (x, y);
            if cycle100(point, 100000) {
                part2 += 1;
            }
        }
    }
    println!("Part 2: {}", part2);
}

fn parse(file: String) -> (i64, i64) {
    let parsed = file.replace("A=[", "")
        .replace("]", "")
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let input = (parsed[0], parsed[1]);
    input
}

fn cycle(xy: (i64, i64), a: (i64, i64), division: i64) -> (i64, i64) {
    let (mut x, mut y) = xy;
    let (ax, ay) = a;
    // X1 * X2 - Y1 * Y2
    let new_x = x * x - y * y;
    // X1 * Y2 + Y1 * X2
    y = x * y + y * x;
    x = new_x;
    x /= division;
    y /= division;
    x += ax;
    y += ay;
    (x, y)
}

fn cycle100(xy: (i64, i64), division: i64) -> bool {
    let mut curr = xy;
    for _ in 1..100 {
        curr = cycle(curr, xy, division);
        if curr.0 > 1000000 || curr.1 > 1000000 || curr.0 < -1000000 || curr.1 < -1000000 {
            return false;
        }
    }
    true
}
