use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, fs::read_to_string};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest13_p1.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter(|(_,c)| c != &'#' && c != &' ').map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();

	let start = *map.iter().find(|x| x.1 == &'S').unwrap().0;

	let res = solvep1a2(&map, start);

	println!("Part 1: {}", res);
}

fn solvep1a2(map: &HashMap<(i32, i32), char>, start: (i32, i32)) -> i32 {
	let end = *map.iter().find(|x| x.1 == &'E').unwrap().0;

	let mut queue = BinaryHeap::new();

	queue.push((Reverse(0), start));
	let mut visited = HashSet::new();

	while let Some((Reverse(length), pos)) = queue.pop() {
		if visited.contains(&pos) {
			continue;
		}
		visited.insert(pos);

		if pos == end {
			return length;
		}
		
		let this = map[&pos];

		let neighbours = [
			(pos.0 - 1, pos.1),
			(pos.0 + 1, pos.1),
			(pos.0, pos.1 - 1),
			(pos.0, pos.1 + 1),
		].into_iter().filter(|x| map.contains_key(x));
		
		for neighbour in neighbours {
			let neigh = map[&neighbour];
			let mov = move_between(this, neigh);
			queue.push((Reverse(length + mov + 1), neighbour));
		}

	}

	panic!("No solution");
}

fn move_between(from: char, to: char) -> i32 {
	let from = from.to_digit(10).unwrap_or(0) as i32;
	let to = to.to_digit(10).unwrap_or(0) as i32;

	let mut diff1 = 0;
	let mut check = from;
	while check != to {
		diff1 += 1;
		check += 1;
		check %= 10;
	}

	let mut diff2 = 0;
	let mut check = from;
	while check != to {
		diff2 += 1;
		check -= 1;
		if check < 0 {
			check = 9;
		}
	}

	diff1.min(diff2)
}

fn part2() {
	let file = read_to_string("input/quest13_p2.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter(|(_,c)| c != &'#' && c != &' ').map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();

	let start = *map.iter().find(|x| x.1 == &'S').unwrap().0;
	let res = solvep1a2(&map, start);

	println!("Part 2: {}", res);
}

fn part3() {
	let file = read_to_string("input/quest13_p3.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter(|(_,c)| c != &'#' && c != &' ').map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();

	let starts = map.iter().filter(|x| x.1 == &'S').map(|x| x.0).cloned().collect::<Vec<_>>();

	// This is quite fast, but it could be faster by pathfinding once from E to any S
	let res = starts.into_iter().map(|start| solvep1a2(&map, start)).min().unwrap();

	println!("Part 3: {}", res);
}