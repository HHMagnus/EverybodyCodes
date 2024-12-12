use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest18_p1.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();

	let start = (0, 1);

	let mut water = HashSet::new();
	water.insert(start);

	let time = solve(&map, water);

	println!("Part 1: {}", time);
}

fn solve(map: &HashMap<(i32, i32), char>, mut water: HashSet<(i32, i32)>) -> i32 {
	let palms = map.iter().filter(|x| x.1 == &'P').map(|x| *x.0).collect::<Vec<_>>();

	let mut time = 0;
	while !palms.iter().all(|x| water.contains(x)) {
		for xs in water.clone() {
			[
				(xs.0 + 1, xs.1),
				(xs.0 - 1, xs.1),
				(xs.0, xs.1 + 1),
				(xs.0, xs.1 - 1),
			].into_iter().filter(|x| map.contains_key(x) && map[x] != '#').for_each(|x| { water.insert(x); });
		}
		time += 1;
	}

	time
}

fn part2() {
	let file = read_to_string("input/quest18_p2.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();

	let mut water = HashSet::new();
	let min_x = map.iter().map(|x| x.0.0).min().unwrap();
	let max_x = map.iter().map(|x| x.0.0).max().unwrap();
	let min_y = map.iter().map(|x| x.0.1).min().unwrap();
	let max_y = map.iter().map(|x| x.0.1).max().unwrap();
	map.iter().filter(|x| x.1 == &'.' && (x.0.0 == min_x || x.0.0 == max_x || x.0.1 == min_y || x.0.1 == max_y))
		.for_each(|x| { water.insert(x.0.clone()); });

	let time = solve(&map, water);

	println!("Part 2: {}", time);
}

fn part3() {
	let file = read_to_string("input/quest18_p3.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<HashMap<_,_>>();

	let mut reachmap = map.clone().into_iter().filter(|x| x.1 != '#').map(|x| (x.0, Vec::<i32>::new())).collect::<HashMap<_,_>>();
	
	let palms = map.iter().filter(|x| x.1 == &'P').map(|x| *x.0).collect::<Vec<_>>();

	for palm in palms {
		let mut visited = HashSet::new();

		let mut queue = VecDeque::new();

		queue.push_back((0, palm));

		while let Some((time, xs)) = queue.pop_front() {
			if visited.contains(&xs) {
				continue;
			}
			visited.insert(xs);
			let reach = reachmap.get_mut(&xs).unwrap();
			reach.push(time);

			[
				(xs.0 + 1, xs.1),
				(xs.0 - 1, xs.1),
				(xs.0, xs.1 + 1),
				(xs.0, xs.1 - 1),
			].into_iter().filter(|x| map.contains_key(x) && map[x] != '#' && !visited.contains(x)).for_each(|x| { queue.push_back((time + 1, x)); });
		}
	}
	
	let time = reachmap.into_iter().filter(|x| map[&x.0] == '.').map(|x| x.1.into_iter().sum::<i32>()).min().unwrap();

	println!("Part 3: {}", time);
}
