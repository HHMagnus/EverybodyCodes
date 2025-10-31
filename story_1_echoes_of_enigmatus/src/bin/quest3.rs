use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest3/input1.txt").unwrap();
    let coords = file.lines()
        .map(|line| {
            let mut split = line.split(" ");
            let x = split.next().unwrap().replace("x=", "").parse::<u32>().unwrap();
            let y = split.next().unwrap().replace("y=", "").parse::<u32>().unwrap();
            (x, y)
        }).collect::<Vec<(u32, u32)>>();
    println!("{:?}", coords);
}