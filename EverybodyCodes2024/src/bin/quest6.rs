use std::{collections::{HashMap, VecDeque}, fs::read_to_string};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest6_p1.txt").unwrap();

	let pos = solve(&file);

	let lengths = pos.iter().map(|x| x.len()).collect::<Vec<_>>();

	let res = pos.into_iter().filter(|x| lengths.iter().filter(|&&y| y == x.len()).count() == 1).next().unwrap();

	println!("Part 1: {}", res.into_iter().collect::<String>());
}

fn part2() {
	let file = read_to_string("input/quest6_p2.txt").unwrap();

	let pos = solve(&file);

	let lengths = pos.iter().map(|x| x.len()).collect::<Vec<_>>();

	let res = pos.into_iter().filter(|x| lengths.iter().filter(|&&y| y == x.len()).count() == 1).next().unwrap();

	println!("Part 2: {}", res.into_iter().map(|x| x.chars().nth(0).unwrap()).collect::<String>());
}

fn part3() {
	let file = read_to_string("input/quest6_p3.txt").unwrap();

	let pos = solve(&file);

	let lengths = pos.iter().map(|x| x.len()).collect::<Vec<_>>();

	let res = pos.into_iter().filter(|x| lengths.iter().filter(|&&y| y == x.len()).count() == 1).next().unwrap();

	println!("Part 3: {}", res.into_iter().map(|x| x.chars().nth(0).unwrap()).collect::<String>());
}

fn solve(file: &String) -> Vec<Vec<&str>>{
	let map = file.lines().map(|line| {
		let mut s = line.split(":");
		let from = s.next().unwrap();
		let to = s.next().unwrap().split(",").collect::<Vec<_>>();
		(from, to)
	}).collect::<HashMap<_,_>>();

	let mut queue = VecDeque::new();
	queue.push_back(("RR", vec!["RR"]));

	let mut res = Vec::new();

	while let Some(next) = queue.pop_front() {
		if next.0 == "@" {
			res.push(next.1.clone());
			continue;
		}
		
		if !map.contains_key(next.0) {
			continue;
		}
		let node = &map[next.0];
		
		for n in node {
			if next.1.contains(n) {
				continue;
			}
			let mut nc = next.1.clone();
			nc.push(n);
			queue.push_back((n, nc));
		}
	}

	res

}