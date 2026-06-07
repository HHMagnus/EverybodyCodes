use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest1/part1.txt").unwrap();

    let input = file.lines().map(|line| {
        let mut split = line.split(":");
        let num = split.next().unwrap().parse::<i32>().unwrap();
        let split2 = split.next().unwrap().split(" ");
        let nums = split2.map(|color| {
            color
                .replace("r", "0")
                .replace("g", "0")
                .replace("b", "0")
                .replace("R", "1")
                .replace("G", "1")
                .replace("B", "1")
                .parse::<i32>()
                .unwrap()
        }).collect::<Vec<_>>();

        (num, nums)
    }).collect::<Vec<_>>();

    let result = input.into_iter()
        .filter(|(_, nums)| nums[0] < nums[1] && nums[1] > nums[2])
        .map(|(num, _)| num)
        .sum::<i32>();
    println!("Quest 1 part 1: {}", result);
}