use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, fs::read_to_string};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest14_p1.txt").unwrap();

	let input = file.split(",").map(|input| {
		let start = input.chars().next().unwrap();
		let length = input[1..].parse::<usize>().unwrap();
		(start, length)
	}).collect::<Vec<_>>();

	let mut res = 0;

	input.into_iter().fold(0, |acc, x| {
		let next = match x.0 {
			'U' => acc + x.1,
			'D' => acc - x.1,
			_ => acc
		};

		res = next.max(res);

		next
	});

	println!("Part 1: {}", res);
}

fn part2() {
	let file = read_to_string("input/quest14_p2.txt").unwrap();

	let inputs = file.lines().map(|line| {
		line.split(",").map(|input| {
			let start = input.chars().next().unwrap();
			let length = input[1..].parse::<usize>().unwrap();
			(start, length)
		}).collect::<Vec<_>>()
	}).collect::<Vec<_>>();
	
	let mut filled = HashSet::new();

	for input in inputs {
		let mut curr = (0, 0, 0);

		for (c, s) in input {
			for _ in 0..s {
				curr = match c {
					'U' => (curr.0, curr.1 + 1, curr.2),
					'D' => (curr.0, curr.1 - 1, curr.2),
					'R' => (curr.0 + 1, curr.1, curr.2),
					'L' => (curr.0 - 1, curr.1, curr.2),
					'F' => (curr.0, curr.1, curr.2 + 1),
					'B' => (curr.0, curr.1, curr.2 - 1),
					_ => panic!("Unknown {}", c),
				};
				filled.insert(curr);
			}
		}
	}

	let res = filled.len();

	println!("Part 2: {}", res);
}

fn part3() {
	let file = read_to_string("input/quest14_p3.txt").unwrap();

	let inputs = file.lines().map(|line| {
		line.split(",").map(|input| {
			let start = input.chars().next().unwrap();
			let length = input[1..].parse::<i32>().unwrap();
			(start, length)
		}).collect::<Vec<_>>()
	}).collect::<Vec<_>>();
	
	let mut filled = HashSet::new();

	let mut leafs = Vec::new();

	for input in &inputs {
		let mut curr = (0, 0, 0);

		for &(c, s) in input {
			for _ in 0..s {
				curr = match c {
					'U' => (curr.0, curr.1 + 1, curr.2),
					'D' => (curr.0, curr.1 - 1, curr.2),
					'R' => (curr.0 + 1, curr.1, curr.2),
					'L' => (curr.0 - 1, curr.1, curr.2),
					'F' => (curr.0, curr.1, curr.2 + 1),
					'B' => (curr.0, curr.1, curr.2 - 1),
					_ => panic!("Unknown {}", c),
				};
				filled.insert(curr);
			}
		}

		leafs.push(curr);
	}

	let max_height = leafs.iter().map(|x| x.1).max().unwrap();

	let res = (0..=max_height).into_iter().filter(|x| filled.contains(&(0, *x, 0))).map(|height| leafs.iter().map(|leaf| dist(height, leaf, &filled)).sum::<i32>()).min().unwrap();

	println!("Part 3: {}", res);
}

fn dist(center_height: i32 , point: &(i32, i32, i32), filled: &HashSet<(i32, i32, i32)>) -> i32 {
	let mut queue = BinaryHeap::new();

	queue.push((Reverse(0), (0, center_height, 0)));
	let mut visited = HashSet::new();

	while let Some((Reverse(length), pos)) = queue.pop() {
		if visited.contains(&pos) {
			continue;
		}
		visited.insert(pos);

		if &pos == point {
			return length;
		}

		let neighbours = [
			(pos.0 - 1, pos.1, pos.2),
			(pos.0 + 1, pos.1, pos.2),
			(pos.0, pos.1 - 1, pos.2),
			(pos.0, pos.1 + 1, pos.2),
			(pos.0, pos.1, pos.2 - 1),
			(pos.0, pos.1, pos.2 + 1),
		].into_iter().filter(|x| filled.contains(x));
		
		for neighbour in neighbours {
			queue.push((Reverse(length + 1), neighbour));
		}

	}
	
	panic!("Cannot happen")
}