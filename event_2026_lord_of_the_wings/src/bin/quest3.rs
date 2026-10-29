use std::fs::read_to_string;
use std::str::Split;

fn main() {
    let file1 = read_to_string("input/quest3/input1.txt").unwrap();

    let part1 = file1.lines()
        .map(iso_num_from_line)
        .sum::<u64>();

    println!("Part 1: {}", part1);

    let file2 = read_to_string("input/quest3/input2.txt").unwrap();

    let part2 = file2.lines()
        .map(iso_num_from_line)
        .sum::<u64>();

    println!("Part 2: {}", part2);

    let file3 = read_to_string("input/quest3/input3.txt").unwrap();

    let part3 = file3.lines()
        .map(iso2_num)
        .sum::<u64>();

    println!("Part 3: {}", part3);
}

fn iso_num_from_line(line: &str) -> u64 {
    let without_iso = line.replace("ISO-", "");
    let mut split = without_iso.split(" ");
    let nums1 = split_to_digits(&mut split);
    let nums2 = split_to_digits(&mut split);

    iso_num(&nums1, &nums2)
}

fn iso_num(nums1: &Vec<u64>, nums2: &Vec<u64>) -> u64 {
    let mut sum = 0;

    let mut nums1 = nums1.into_iter()
        .rev()
        .cycle();

    for i in 0..nums2.len() {
        let num2 = nums2[nums2.len() - 1 - i];
        let num1 = nums1.next().unwrap();

        let num1 = if i == 0 {
            1
        } else {
            num1.pow(i as u32)
        };

        sum += num1 * num2;
    }

    sum
}

fn iso2_num(line: &str) -> u64 {
    let without_iso = line.replace("ISO2-", "");
    let mut split = without_iso.split(" ");
    let nums1 = split_to_digits(&mut split);
    let nums2 = split_to_digits(&mut split);

    let original = iso_num(&nums1, &nums2);

    let num2 = nums2.iter().map(u64::to_string).collect::<String>()
        .parse::<u64>().unwrap();

    for x in 0..num2 {
        let nums2 = x.to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        let lower_iso = iso_num(&nums1, &nums2);
        if lower_iso == original {
            return 0;
        }
    }

    original
}

fn split_to_digits(split: &mut Split<&str>) -> Vec<u64> {
    split.next().unwrap().chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
}