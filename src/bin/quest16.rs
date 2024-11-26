use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest16_p1.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let turns = lines[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let input = lines[2..].into_iter().map(|&x| vec![&x[0..=2], &x[4..=6], &x[8..=10], &x[12..=14]]).collect::<Vec<_>>();

	let faces = turns.iter().enumerate().map(|(i, _)| input.iter().map(|x| x[i]).filter(|x| !x.trim().is_empty()).collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut index = turns.iter().map(|_| 0).collect::<Vec<_>>();

	for _ in 0..100 {
		index = index.into_iter().enumerate().map(|(i, x)| (turns[i] + x) % faces[i].len()).collect();
	}

	let res = faces.into_iter().enumerate().map(|(i, face)| face[index[i]]).collect::<Vec<_>>();
	let res = res.join(" ");

	println!("Part 1: {}", res);
}

fn part2() {
	let file = read_to_string("input/quest16_p2.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let turns = lines[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let input = lines[2..].into_iter().map(|&x| (0..turns.len()).map(|i| &x[i*4..=i*4+2]).collect::<Vec<_>>()).collect::<Vec<_>>();

	let faces = turns.iter().enumerate().map(|(i, _)| input.iter().map(|x| x[i]).filter(|x| !x.trim().is_empty()).collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut index = turns.iter().map(|_| 0).collect::<Vec<_>>();

	let mut i = 0 as usize;
	
	let mut cycle_detection = HashMap::new();

	cycle_detection.insert(index.clone(), (0, 0));

	let total = 202420242024;

	let mut total_score = 0;

	while i < total {
		index = index.into_iter().enumerate().map(|(i, x)| (turns[i] + x) % faces[i].len()).collect();

		let score = index.iter().enumerate()
			.flat_map(|(i, &x)| vec![faces[i][x].chars().nth(0).unwrap(), faces[i][x].chars().nth(2).unwrap()])
			.counts_by(|x| x)
			.into_iter()
			.filter(|x| x.1 > 2)
			.map(|x| x.1 - 2)
			.sum::<usize>();

		total_score += score;
		i += 1;

		let cycle_key = index.clone();
		if cycle_detection.contains_key(&cycle_key) {
			let (last_i, last_score) = cycle_detection[&cycle_key];
			let cycle_length = i - last_i;
			let moves = (total - i) / cycle_length;
			let score_diff = total_score - last_score;
			total_score += score_diff * moves;
			i += moves * cycle_length;
			cycle_detection.clear();
		} else {
			cycle_detection.insert(cycle_key, (i, total_score));
		}
	}

	println!("Part 2: {}", total_score);
}

fn part3() {
	let file = read_to_string("input/quest16_p3.txt").unwrap();
}