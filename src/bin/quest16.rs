use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, fs::read_to_string};

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

		let score = score(&index, &faces);

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

fn score(index: &Vec<usize>, faces: &Vec<Vec<&str>>) -> usize {
	index.iter().enumerate()
		.flat_map(|(i, &x)| vec![faces[i][x].chars().nth(0).unwrap(), faces[i][x].chars().nth(2).unwrap()])
		.counts_by(|x| x)
		.into_iter()
		.filter(|x| x.1 > 2)
		.map(|x| x.1 - 2)
		.sum::<usize>()
}

fn part3() {
	let file = read_to_string("input/quest16_p3.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let turns = lines[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let input = lines[2..].into_iter().map(|&x| (0..turns.len()).map(|i| &x[i*4..=i*4+2]).collect::<Vec<_>>()).collect::<Vec<_>>();

	let faces = turns.iter().enumerate().map(|(i, _)| input.iter().map(|x| x[i]).filter(|x| !x.trim().is_empty()).collect::<Vec<_>>()).collect::<Vec<_>>();

	let index = turns.iter().map(|_| 0).collect::<Vec<_>>();

	let mut queue = BinaryHeap::new();

	queue.push((0, 0, index.clone()));

	let mut max_score = (0..=256).into_iter().map(|_| 0).collect::<Vec<_>>();
	let mut seen: Vec<HashMap<Vec<usize>, usize>> = (0..=256).into_iter().map(|_| HashMap::new()).collect::<Vec<_>>();

	while let Some((total_score, turn, index)) = queue.pop() {
		if seen[turn].contains_key(&index) && seen[turn][&index] >= total_score {
			continue;
		}
		seen[turn].insert(index.clone(), total_score);

		if total_score > max_score[turn] {
			max_score[turn] = total_score;
		}

		if turn == 256 {
			continue;
		}

		let pushed = index.clone().into_iter().enumerate().map(|(i, x)| (x + 1) % faces[i].len()).collect::<Vec<_>>();
		let pushed = pushed.into_iter().enumerate().map(|(i, x)| (turns[i] + x) % faces[i].len()).collect::<Vec<_>>();
		queue.push((score(&pushed, &faces) + total_score, turn+1, pushed));

		let pulled = index.clone().into_iter().enumerate().map(|(i, x)| ((x + faces[i].len()) - 1) % faces[i].len()).collect::<Vec<_>>();
		let pulled = pulled.into_iter().enumerate().map(|(i, x)| (turns[i] + x) % faces[i].len()).collect::<Vec<_>>();
		queue.push((score(&pulled, &faces) + total_score, turn+1, pulled));

		let stayed = index.into_iter().enumerate().map(|(i, x)| (turns[i] + x) % faces[i].len()).collect::<Vec<_>>();
		queue.push((score(&stayed, &faces) + total_score, turn+1, stayed));
	}	

	let mut queue = BinaryHeap::new();

	queue.push((Reverse(0), 0, index));

	let mut min_score = (0..=256).into_iter().map(|_| 256*4).collect::<Vec<_>>();
	let mut seen: Vec<HashMap<Vec<usize>, usize>> = (0..=256).into_iter().map(|_| HashMap::new()).collect::<Vec<_>>();

	while let Some((total_score, turn, index)) = queue.pop() {
		if seen[turn].contains_key(&index) && seen[turn][&index] <= total_score.0 {
			continue;
		}
		seen[turn].insert(index.clone(), total_score.0);

		if total_score.0 < min_score[turn] {
			min_score[turn] = total_score.0;
		}

		if turn == 256 {
			continue;
		}

		let pushed = index.clone().into_iter().enumerate().map(|(i, x)| (x + 1) % faces[i].len()).collect::<Vec<_>>();
		let pushed = pushed.into_iter().enumerate().map(|(i, x)| (turns[i] + x) % faces[i].len()).collect::<Vec<_>>();
		queue.push((Reverse(score(&pushed, &faces) + total_score.0), turn+1, pushed));

		let pulled = index.clone().into_iter().enumerate().map(|(i, x)| ((x + faces[i].len()) - 1) % faces[i].len()).collect::<Vec<_>>();
		let pulled = pulled.into_iter().enumerate().map(|(i, x)| (turns[i] + x) % faces[i].len()).collect::<Vec<_>>();
		queue.push((Reverse(score(&pulled, &faces) + total_score.0), turn+1, pulled));

		let stayed = index.into_iter().enumerate().map(|(i, x)| (turns[i] + x) % faces[i].len()).collect::<Vec<_>>();
		queue.push((Reverse(score(&stayed, &faces) + total_score.0), turn+1, stayed));
	}

	println!("Part 3: {} {}", max_score[256], min_score[256]);
}