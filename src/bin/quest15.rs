use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap, HashMap, HashSet}, fs::read_to_string};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest15_p1.txt").unwrap();
	
	let res = solve(file);

	println!("Part 1: {}", res);
}

fn part2() {
	let file = read_to_string("input/quest15_p2.txt").unwrap();
	
	let res = solve(file);

	println!("Part 2: {}", res);
}

fn part3() {
	let file = read_to_string("input/quest15_p3.txt").unwrap();
	
	// This is technically fast enough for part 3, but it could be optimized further.
	// An example would be to construct a graph of shortest path such that instead of searching in the entire area, it only searched between actually important points
	// By doing this the search space would be drastically minimized.
	let res = solve(file);

	println!("Part 3: {}", res);
}

fn solve(file: String) -> i32 {
	let map = file.lines().enumerate().flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();

	let start = *map.iter().find(|((_, y), c)| y == &0 && c == &&'.').unwrap().0;

	let goals = map.iter().map(|x| x.1).filter(|&&x| x != '#' && x != '~' && x != '.').cloned().collect::<BTreeSet<_>>();

	dist(start, goals, &map)
}

fn dist(start: (i32, i32) , goals: BTreeSet<char>, map: &HashMap<(i32, i32), char>) -> i32 {
	let mut queue = BinaryHeap::new();

	queue.push((Reverse(0), start, BTreeSet::new(), false));

	let mut visited = HashSet::new();

	while let Some((Reverse(length), pos, mut reached, backwards)) = queue.pop() {
		let curr = map[&pos];

		if goals.contains(&curr) && !reached.contains(&curr) {
			reached.insert(curr);
		}

		let key = (pos, reached.clone());
		if visited.contains(&key) {
			continue;
		}
		visited.insert(key);

		if reached == goals && pos == start {
			return length;
		}

		let neighbours = [
			(pos.0 - 1, pos.1),
			(pos.0 + 1, pos.1),
			(pos.0, pos.1 - 1),
			(pos.0, pos.1 + 1),
		].into_iter().filter(|x| map.contains_key(x) && map[x] != '#' && map[x] != '~');
		
		for neighbour in neighbours {
			queue.push((Reverse(length + 1), neighbour, reached.clone(), backwards));
		}
	}
	
	panic!("Cannot happen")
}
