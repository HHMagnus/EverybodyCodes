use std::{collections::{HashMap, HashSet}, fs::read_to_string};

use itertools::Itertools;

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest12_p1.txt").unwrap();

	let matrix = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i64, y as i64), c))).collect::<HashMap<_,_>>();

	let a = *matrix.iter().find(|x| x.1 == &'A').unwrap().0;
	let b = *matrix.iter().find(|x| x.1 == &'B').unwrap().0;
	let c = *matrix.iter().find(|x| x.1 == &'C').unwrap().0;

	let t = matrix.iter().filter(|x| x.1 == &'T')
		.map(|x| *x.0)
		.sorted()
		.collect::<Vec<_>>();

	let mut res = 0;

	'outer: for target in t {
		for (rank, &from) in [a, b, c].iter().enumerate() {
			for power in 1..100 {
				if hits(from, target, power) {
					res += (rank+1) as i64 * power;
					continue 'outer;
				}
			}
		}
		panic!("Not found for {:?}", target);
	}

	println!("Part 1: {}", res);
}

fn hits(from: (i64, i64), target: (i64, i64), power: i64) -> bool {
	let mut upper = (from.0 + 2 * power, from.1 - power);

	while upper.0 < target.0 {
		upper = (upper.0 + 1, upper.1 + 1);

		if upper == target {
			return true;
		}
	}
	return false;
}

fn part2() {
	let file = read_to_string("input/quest12_p2.txt").unwrap();

	let matrix = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i64, y as i64), c))).collect::<HashMap<_,_>>();

	let a = *matrix.iter().find(|x| x.1 == &'A').unwrap().0;
	let b = *matrix.iter().find(|x| x.1 == &'B').unwrap().0;
	let c = *matrix.iter().find(|x| x.1 == &'C').unwrap().0;

	let t = matrix.iter().filter(|x| x.1 == &'T' || x.1 == &'H')
		.map(|x| *x.0)
		.sorted()
		.collect::<Vec<_>>();

	let mut res = 0;

	'outer: for target in t {
		for (rank, &from) in [a, b, c].iter().enumerate() {
			for power in 1..100 {
				if hits(from, target, power) {
					let multiplier = if matrix[&target] == 'T' { 1 } else { 2 };
					res += (rank+1) as i64 * power * multiplier;
					continue 'outer;
				}
			}
		}
		panic!("Not found for {:?}", target);
	}

	println!("Part 2: {}", res);
}

// This part is slow. It could be optimized further, but right now it handles the job
fn part3() {
	let file = read_to_string("input/quest12_p3.txt").unwrap();

	let star_paths = file.lines().map(|x| {
		let mut split = x.split(" ");
		let x = split.next().unwrap().parse::<usize>().unwrap();
		let y = split.next().unwrap().parse::<usize>().unwrap();
		(x, y)
	}).map(|(x, y)| {
		let mut time = 0;
		let mut x = x;
		let mut y = y;
		let mut path = HashMap::new();
		loop {
			path.insert((x, y), time);
			if y == 0 || x == 0 {
				break;
			}
			x -= 1;
			y -= 1;
			time += 1;
		}
		path
	}).collect::<Vec<_>>();
	
	let mut scores = star_paths.iter().map(|_| (usize::MAX, 0)).collect::<Vec<_>>();

	let mut power = vec![1, 1, 1];

	let largest_time = *star_paths.iter().map(|x| x.iter().map(|x| x.1).max().unwrap()).max().unwrap();
	let furthest_x = star_paths.iter().map(|x| x.keys().map(|x| x.0).max().unwrap()).max().unwrap();

	while power[2] < largest_time {
		let choice = power.iter().enumerate().filter(|x| *x.1 < largest_time).min_by_key(|(i, p)| **p * (i+1)).unwrap();
		let choice = (choice.0, *choice.1);

		let start = (0, choice.0);
		let mut vec = Vec::new();
		let mut pos = start;
		let mut time = 0;
		for _ in 0..choice.1 {
			time += 1;
			pos = (pos.0 + 1, pos.1 + 1);
			if pos.0 > furthest_x {
				break;
			}
			let check = (time, pos);
			vec.push(check);
		}
		for _ in 0..choice.1 {
			time += 1;
			pos = (pos.0 + 1, pos.1);
			if pos.0 > furthest_x {
				break;
			}
			let check = (time, pos);
			vec.push(check);
		}
		while pos.1 > 0 && pos.0 < furthest_x {
			time += 1;
			pos = (pos.0 + 1, pos.1 - 1);
			let check = (time, pos);
			vec.push(check);
		}

		for (time, pos) in vec {
			for (i, x) in star_paths.iter().enumerate() {

				if x.contains_key(&pos) {
					let expected_time = x[&pos];
					if expected_time >= time {
						let score = (choice.0+1) * choice.1;
						let height = pos.1;
						if scores[i].1 < height || (scores[i].1 == height && scores[i].0 > score) {
							scores[i] = (score, height);
						}
					}
				}
			}
		}

		power[choice.0] += 1;
	}

	let res = scores.into_iter().map(|x| x.0).sum::<usize>();

	println!("Part 3: {}", res);
}