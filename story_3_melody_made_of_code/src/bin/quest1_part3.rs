use std::fs::read_to_string;

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
enum Shine {
    Shiny,
    Matte
}

fn main() {
    let file = read_to_string("input/quest1/part3.txt").unwrap();

    let input = parse(file);

    let mut list = input.into_iter()
        .filter_map(|(num, nums)| {
            let color = if nums[0] > nums[1] && nums[0] > nums[2] {
                Color::Red
            } else if nums[1] > nums[0] && nums[1] > nums[2] {
                Color::Green
            } else if nums[2] > nums[0] && nums[2] > nums[1] {
                Color::Blue
            } else {
                return None;
            };

            let shine = if nums[3] <= 30 {
                Shine::Matte
            } else if nums[3] >= 33 {
                Shine::Shiny
            } else {
                return None;
            };

            Some((num, (shine, color)))
        }).collect::<Vec<_>>();
    list.sort_by(|(_, a), (_, b)| a.cmp(&b));

    let result = list.chunk_by(|(_, x), (_, y)| x == y)
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .into_iter()
        .map(|(x, _)| x)
        .sum::<i32>();

    println!("Quest 1 part 3: {:?}", result);
}

fn parse(file: String) -> Vec<(i32, Vec<i32>)> {
    let input = file.lines().map(|line| {
        let mut split = line.split(":");
        let num = split.next().unwrap().parse::<i32>().unwrap();
        let split2 = split.next().unwrap().split(" ");
        let nums = split2.map(|color| {
            let number = color
                .replace("r", "0")
                .replace("g", "0")
                .replace("b", "0")
                .replace("s", "0")
                .replace("R", "1")
                .replace("G", "1")
                .replace("B", "1")
                .replace("S", "1");
            i32::from_str_radix(&number, 2).unwrap()
        }).collect::<Vec<_>>();

        (num, nums)
    }).collect::<Vec<_>>();
    input
}