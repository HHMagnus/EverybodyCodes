use std::fs::read_to_string;

fn main() {
    let file1 = read_to_string("input/quest1/input1.txt").unwrap();
    let part1 = file1.split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .map(square_of_pines)
        .sum::<u32>();
    println!("Part 1: {}", part1);

    let file2 = read_to_string("input/quest1/input2.txt").unwrap();
    let part2 = file2.split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .map(frame_of_pines)
        .sum::<u32>();
    println!("Part 2: {}", part2);

    let file3 = read_to_string("input/quest1/input3.txt").unwrap();
    let part3 = file3.split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .map(square_of_pines_without_center)
        .sum::<u32>();
    println!("Part 3: {:?}", part3);
}

fn square_of_pines(x: u32) -> u32 {
    let mut i = 1;
    while i*i < x {
        i += 1;
    }
    i*i - x
}

fn frame_of_pines(x: u32) -> u32 {
    let mut i = 3;
    while i*4-4 < x {
        i += 1;
    }
    i*4-4 - x
}

fn square_of_pines_without_center(x: u32) -> u32 {
    let mut i = 3;
    while i*i-1 < x {
        i += 2;
    }
    i*i-1 - x
}