use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest1/part2.txt").unwrap();

    let input = file.lines().map(|line| {
        let mut split = line.split(":");
        let num = split.next().unwrap().parse::<i32>().unwrap();
        let split2 = split.next().unwrap().split(" ");
        let nums = split2.map(|color| {
            color
                .replace("r", "0")
                .replace("g", "0")
                .replace("b", "0")
                .replace("s", "0")
                .replace("R", "1")
                .replace("G", "1")
                .replace("B", "1")
                .replace("S", "1")
                .parse::<i32>()
                .unwrap()
        }).collect::<Vec<_>>();

        (num, nums)
    }).collect::<Vec<_>>();

    let result = input.into_iter()
        .max_by(|(_, a), (_, b)| a[3].cmp(&b[3]) .then_with(|| (b[0] + b[1] + b[2]).cmp(&(a[0] + a[1] + a[2]))))
        .unwrap()
        .0;
    println!("Quest 1 part 2: {}", result);
}