use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest2/input1.txt").unwrap();
    let parsed = file.replace("A=[", "")
        .replace("]", "")
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let input = (parsed[0], parsed[1]);

    let mut value = (0, 0);
    for _ in 0..3 {
        value = cycle(value, input);
        println!("{:?}, {:?}", value, value);
    }
    
    println!("Part 1: [{},{}]", value.0, value.1);
}

fn cycle(xy: (i64, i64), a: (i64, i64)) -> (i64, i64) {
    let (mut x, mut y) = xy;
    let (ax, ay) = a;
    // X1 * X2 - Y1 * Y2
    let new_x = x * x - y * y;
    // X1 * Y2 + Y1 * X2
    y = x * y + y * x;
    x = new_x;
    x /= 10;
    y /= 10;
    x += ax;
    y += ay;
    (x, y)
}