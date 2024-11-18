use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, fs::read_to_string, usize};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest9_p1.txt").unwrap();

	let res = file.lines().map(|line| line.parse::<usize>().unwrap())
		.map(|value| {
			let mut total = value / 10;
			let mut rem = value % 10;

			if rem > 0 {
				total += rem / 5;
				rem %= 5;
			}

			if rem > 0 {
				total += rem / 3;
				rem %= 3;
			}

			if rem > 0 {
				total += rem;
			}

			total
		}).sum::<usize>();

	println!("Part 1: {}", res);
}

fn part2() {
	let file = read_to_string("input/quest9_p2.txt").unwrap();

	let list = vec![1, 3, 5, 10, 15, 16, 20, 24, 25, 30];

	let res = file.lines().map(|line| line.parse::<usize>().unwrap())
		.map(|value| solve2a3(&list, value)).sum::<usize>();

	println!("Part 2: {}", res);
}

fn part3() {
	let file = read_to_string("input/quest9_p3.txt").unwrap();

	let list = vec![1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101];

	let res = file.lines().map(|line| line.parse::<usize>().unwrap())
		.map(|value| {

			let mut first = value / 2;
			let mut second = value - first;

			let mut best = usize::MAX;

			while first.abs_diff(second) <= 100 {
				let b1 = solve2a3(&list, first);
				let b2 = solve2a3(&list, second);
				let b = b1 + b2;
				best = best.min(b);

				first -= 1;
				second += 1;
			}
			
			best
		}).sum::<usize>();

	println!("Part 3: {}", res);
}

fn solve2a3(list: &Vec<usize>, value: usize) -> usize {
	let mut vec = BinaryHeap::new();
	vec.push((Reverse(0), Reverse(0)));
	let mut visited = HashSet::new();
	visited.insert(0);

	while let Some((step, x)) = vec.pop() {
		let x = x.0;
		if x > value {
			continue;
		}

		for y in list {
			let new = x + y;
			if new == value {
				return step.0+1;
			}
			if visited.contains(&new) {
				continue;
			}
			visited.insert(new);
			vec.push((Reverse(step.0+1), Reverse(new)));
		}
	}
	
	panic!("No solution found");
}