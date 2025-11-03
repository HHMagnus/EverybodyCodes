use std::{collections::HashSet, fs::read_to_string};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest3_p1.txt").unwrap();

	let part1 = solve1a2(file);
	println!("Part 1: {:?}", part1);
}

fn part2() {
	let file = read_to_string("input/quest3_p2.txt").unwrap();

	let part2 = solve1a2(file);
	println!("Part 1: {:?}", part2);
}

fn solve1a2(file: String) -> usize {
	let mut set = file.lines().enumerate().flat_map(|(y, line)| {
		let chars = line.chars().enumerate().filter_map(|(x, c)| {
			if c == '#' { Some((x,y)) } else { None }
		});
		chars.collect::<Vec<_>>()
	}).collect::<HashSet<_>>();

	let mut res = 0;
	while !set.is_empty() {
		res += set.len();

		set = set.iter().cloned().filter(|(x, y)| set.contains(&(x - 1, *y)) && set.contains(&(x + 1, *y)) && set.contains(&(*x, y - 1)) && set.contains(&(*x, y + 1))).collect();
	}

	res
}

fn part3() {
	let file = read_to_string("input/quest3_p3.txt").unwrap();

	let mut set = file.lines().enumerate().flat_map(|(y, line)| {
		let chars = line.chars().enumerate().filter_map(|(x, c)| {
			if c == '#' { Some((x+1,y+1)) } else { None }
		});
		chars.collect::<Vec<_>>()
	}).collect::<HashSet<_>>();

	let mut part3 = 0;
	while !set.is_empty() {
		part3 += set.len();

		set = set.iter().cloned().filter(|(x, y)| set.contains(&(x - 1, *y)) && set.contains(&(x + 1, *y)) && set.contains(&(*x, y - 1)) && set.contains(&(*x, y + 1))
			&& set.contains(&(x - 1, *y - 1)) && set.contains(&(x + 1, *y - 1)) && set.contains(&(*x - 1, y + 1)) && set.contains(&(*x + 1, y + 1))).collect();
	}
	
	println!("Part 1: {:?}", part3);
}