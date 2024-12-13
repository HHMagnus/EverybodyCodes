use std::{collections::BTreeMap, fs::read_to_string};

fn main() {
	part1();
	part2();
	part3();
}

enum Rot {
	Right,
	Left
}

fn part1() {
	let file = read_to_string("input/quest19_p1.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let mut command = lines[0].chars().map(|x| if x == 'R' { Rot::Right } else if x == 'L' { Rot::Left } else { panic!("wtf is {}", x)}).cycle();

	let mut map = lines[2..].into_iter().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<BTreeMap<_,_>>();

	rotate_all(&mut map, &mut command);
	
	let max_x = map.iter().map(|x| x.0.0).max().unwrap();
	let max_y = map.iter().map(|x| x.0.1).max().unwrap();
	println!("Part 1: ");
	for y in 0..=max_y {
		for x in 0..=max_x {
			print!("{}", map[&(x, y)]);
		}
		println!("");
	}
	println!("");
}

fn part2() {
	let file = read_to_string("input/quest19_p2.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let mut map = lines[2..].into_iter().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<BTreeMap<_,_>>();

	for _ in 0..100 {
		let mut command = lines[0].chars().map(|x| if x == 'R' { Rot::Right } else if x == 'L' { Rot::Left } else { panic!("wtf is {}", x)}).cycle();
		rotate_all(&mut map, &mut command);
	}

	let max_x = map.iter().map(|x| x.0.0).max().unwrap();
	let max_y = map.iter().map(|x| x.0.1).max().unwrap();
	println!("Part 2: ");
	for y in 0..=max_y {
		for x in 0..=max_x {
			print!("{}", map[&(x, y)]);
		}
		println!("");
	}
	println!("");
}

fn rotate_all(map: &mut BTreeMap<(i32, i32), char>, command: &mut impl Iterator<Item = Rot>) {
	let max_x = map.iter().map(|x| x.0.0).max().unwrap();
	let max_y = map.iter().map(|x| x.0.1).max().unwrap();
	for y in 0..=max_y {
		for x in 0..=max_x {
			let neighbour_points = [
				(x - 1, y - 1),
				(x + 0, y - 1),
				(x + 1, y - 1),
				(x + 1, y + 0),
				(x + 1, y + 1),
				(x + 0, y + 1),
				(x - 1, y + 1),
				(x - 1, y + 0),
			];
			let is_rotation_point = neighbour_points.iter().all(|x| map.contains_key(x));

			if !is_rotation_point {
				continue;
			}

			match command.next().unwrap() {
				Rot::Right => {
					let original = map[&neighbour_points[neighbour_points.len()-1]];
					for i in (1..neighbour_points.len()).rev() {
						*map.get_mut(&neighbour_points[i]).unwrap() = map[&neighbour_points[i-1]];
					}
					*map.get_mut(&neighbour_points[0]).unwrap() = original;
				},
				Rot::Left => {
					let original = map[&neighbour_points[0]];
					for i in 0..neighbour_points.len()-1 {
						*map.get_mut(&neighbour_points[i]).unwrap() = map[&neighbour_points[i+1]];
					}
					*map.get_mut(&neighbour_points[neighbour_points.len()-1]).unwrap() = original;
				},
			}
		}
	}
}

fn part3() {
	let file = read_to_string("input/quest19_p3.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let mut map = lines[2..].into_iter().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))).collect::<BTreeMap<_,_>>();

	let mut i = 0;
	while i < 1048576000 {
		if i % 1000 == 0 {
			println!("{}", i);
		}
		let mut command = lines[0].chars().map(|x| if x == 'R' { Rot::Right } else if x == 'L' { Rot::Left } else { panic!("wtf is {}", x)}).cycle();
		rotate_all(&mut map, &mut command);
		i += 1;
	}

	let max_x = map.iter().map(|x| x.0.0).max().unwrap();
	let max_y = map.iter().map(|x| x.0.1).max().unwrap();
	println!("Part 3: ");
	for y in 0..=max_y {
		for x in 0..=max_x {
			print!("{}", map[&(x, y)]);
		}
		println!("");
	}
	println!("");
}
