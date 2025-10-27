use std::{collections::HashMap, fs::read_to_string, usize};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest11_p1.txt").unwrap();

	let map = file.lines().map(|line| {
		let mut split = line.split(":");
		let x = split.next().unwrap();
		let ys = split.next().unwrap().split(",").collect::<Vec<_>>();
		(x, ys)
	}).collect::<HashMap<_,_>>();

	let mut population = vec!["A"];
	
	for _ in 0..4 {
		population = population.into_iter().flat_map(|x| map[x].clone()).collect();
	}

	println!("Part 1: {}", population.len());
}

fn part2() {
	let file = read_to_string("input/quest11_p2.txt").unwrap();

	let map = file.lines().map(|line| {
		let mut split = line.split(":");
		let x = split.next().unwrap();
		let ys = split.next().unwrap().split(",").collect::<Vec<_>>();
		(x, ys)
	}).collect::<HashMap<_,_>>();

	let mut population = vec!["Z"];
	
	for _ in 0..10 {
		population = population.into_iter().flat_map(|x| map[x].clone()).collect();
	}

	println!("Part 2: {}", population.len());
}

fn part3() {
	let file = read_to_string("input/quest11_p3.txt").unwrap();

	let map = file.lines().map(|line| {
		let mut split = line.split(":");
		let x = split.next().unwrap();
		let ys = split.next().unwrap().split(",").collect::<Vec<_>>();
		(x, ys)
	}).collect::<HashMap<_,_>>();

	let mut min = usize::MAX;
	let mut max = usize::MIN;

	let mut after10 = HashMap::new();

	for &start in map.keys() {
		let mut population = vec![start];
	
		for _ in 0..10 {
			population = population.into_iter().flat_map(|x| map[x].clone()).collect();
		}

		after10.insert(start, population.len());
	}

	for &start in map.keys() {
		let mut population = vec![start];
	
		for _ in 0..10 {
			population = population.into_iter().flat_map(|x| map[x].clone()).collect();
		}
		
		let size = population.into_iter().map(|x|after10[x]).sum::<usize>();

		min = min.min(size);
		max = max.max(size);
	}

	let res = max - min;

	println!("Part 3: {}", res);
}