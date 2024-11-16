use std::{collections::{HashMap, HashSet}, fs::read_to_string, usize};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest4_p1.txt").unwrap();

	let res = solve1a2(file);

	println!("Part 1: {:?}", res);
}

fn part2() {
	let file = read_to_string("input/quest4_p2.txt").unwrap();

	let res = solve1a2(file);

	println!("Part 2: {:?}", res);
}

fn solve1a2(file: String) -> usize {
	let nums = file.lines().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let min = *nums.iter().min().unwrap();

	nums.iter().map(|x| x - min).sum::<usize>()
}

fn part3() {
	let file = read_to_string("input/quest4_p3.txt").unwrap();

	let nums = file.lines().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let sum = nums.iter().sum::<usize>();
	let mut avg = sum / nums.len();
	let mut res = usize::MAX;
	loop {
		let t = nums.iter().map(|x| x.abs_diff(avg)).sum::<usize>();

		if t > res {
			break;
		}
		res = t;
		avg += 1;
	}

	println!("Part 3: {:?}", res);
}
