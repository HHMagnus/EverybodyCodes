use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, fs::read_to_string};

fn main() {
	part1();
	//part2();
	part3();
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Dir {
	Up,
	Down,
	Right,
	Left,
}

fn part1() {
	let file = read_to_string("input/quest20_p1.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| {
		cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let start = map.iter().find(|x| x.1 == &'S').unwrap().0.clone();
	let dir = Dir::Down;

	let mut queue = BinaryHeap::new();
	queue.push((1000, Reverse(0), start, dir));

	let mut visited = HashMap::<((i32, i32), Dir), i32>::new();

	let mut max = 0;

	while let Some((altitude, Reverse(time), pos, dir)) = queue.pop() {
		let key = (pos, dir);
		if visited.contains_key(&key) {
			let alt = visited[&key];
			if alt > altitude {
				continue;
			}
		}
		visited.insert(key, altitude);
		if time == 100 {
			max = max.max(altitude);
			break;
		}

		[
			dir,
			turn_left(dir),
			turn_right(dir),
		].into_iter().for_each(|dir| {
			let newpos = match dir {
				Dir::Up => (pos.0, pos.1 - 1),
				Dir::Down => (pos.0, pos.1 + 1),
				Dir::Right => (pos.0 + 1, pos.1),
				Dir::Left => (pos.0 - 1, pos.1),
			};

			if !map.contains_key(&newpos) {
				return;
			}

			let stream = map[&newpos];
			
			let altitude = match stream {
				'.' => altitude-1,
				'-' => altitude-2,
				'+' => altitude+1,
				_ => return,
			};

			queue.push((altitude, Reverse(time +1 ), newpos, dir));
		});
	}

	println!("Part 1: {}", max);
}

fn turn_left(dir: Dir) -> Dir {
	match dir {
		Dir::Up => Dir::Left,
		Dir::Down => Dir::Right,
		Dir::Right => Dir::Up,
		Dir::Left => Dir::Down,
	}
}

fn turn_right(dir: Dir) -> Dir {
	match dir {
		Dir::Up => Dir::Right,
		Dir::Down => Dir::Left,
		Dir::Right => Dir::Down,
		Dir::Left => Dir::Up,
	}
}

fn part2() {
	let file = read_to_string("input/quest20_p2.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| {
		cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let start = map.iter().find(|x| x.1 == &'S').unwrap().0.clone();
	let dir = Dir::Down;

	let cliff = 10000;

	let mut queue = BinaryHeap::new();
	queue.push((Reverse(0), cliff, start, dir, false, false, false));

	let mut visited = HashMap::new();

	let mut goal = Option::None;

	while let Some((Reverse(time), altitude, pos, dir, a, b, c)) = queue.pop() {
		let key = (pos, dir, a, b, c);
		if visited.contains_key(&key) {
			let (tim, alt) = visited[&key];
			if tim <= time && alt >= altitude {
				continue;
			}
		}
		visited.insert(key, (time, altitude));
		
		if goal.is_some() {
			break;
		}

		[
			dir,
			turn_left(dir),
			turn_right(dir),
		].into_iter().for_each(|dir| {
			let mut a = a;
			let mut b = b;
			let mut c = c;

			let newpos = match dir {
				Dir::Up => (pos.0, pos.1 - 1),
				Dir::Down => (pos.0, pos.1 + 1),
				Dir::Right => (pos.0 + 1, pos.1),
				Dir::Left => (pos.0 - 1, pos.1),
			};

			if !map.contains_key(&newpos) {
				return;
			}

			let stream = map[&newpos];
			
			let altitude = match stream {
				'.' => altitude-1,
				'-' => altitude-2,
				'+' => altitude+1,
				'S' => {
					if a && b && c && altitude > cliff {
						goal = Some(time+1);
					}
					return
				},
				'A' => {
					if a { return }
					a = true;
					altitude-1
				},
				'B' => {
					if !a || b {
						return
					}
					b = true;
					altitude-1
				},
				'C' => {
					if !a || !b || c {
						return
					}
					c = true;
					altitude-1
				}
				_ => return,
			};

			queue.push((Reverse(time + 1), altitude, newpos, dir, a, b, c));
		});
	}

	println!("Part 2: {}", goal.unwrap());
}

fn part3() {
	let file = read_to_string("input/quest20_p3.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| {
		cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let start = map.iter().find(|x| x.1 == &'S').unwrap().0.clone();
	let dir = Dir::Down;

	let starting_altitude = 384400;

	let mut queue = BinaryHeap::new();
	queue.push((starting_altitude, start, dir));

	let mut visited = HashMap::new();

	let maxy = map.iter().map(|x| x.0.1).max().unwrap()+1;

	let mut furthest = 0;

	while let Some((altitude, pos, dir)) = queue.pop() {
		if furthest < pos.1 {
			furthest = furthest.max(pos.1);
		}
		
		if altitude <= 0 {
			continue;
		}

		let key = (pos, dir);
		if visited.contains_key(&key) {
			let alt = visited[&key];
			if alt >= altitude {
				continue;
			}
		}
		visited.insert(key,  altitude);

		[
			dir,
			turn_left(dir),
			turn_right(dir),
		].into_iter().for_each(|dir| {
			let newpos = match dir {
				Dir::Up => (pos.0, pos.1 - 1),
				Dir::Down => (pos.0, pos.1 + 1),
				Dir::Right => (pos.0 + 1, pos.1),
				Dir::Left => (pos.0 - 1, pos.1),
			};

			let mappos = (newpos.0, newpos.1 % maxy);

			if !map.contains_key(&mappos) {
				return;
			}

			let stream = map[&mappos];
			
			let altitude = match stream {
				'.' => altitude-1,
				'-' => altitude-2,
				'+' => altitude+1,
				_ => return,
			};

			queue.push((altitude, newpos, dir));
		});
	}

	println!("Part 3: {}", furthest);
}
