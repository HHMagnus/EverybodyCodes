use std::{collections::HashSet, fs::read_to_string};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest2_p1.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let line1 = lines[0].replace("WORDS:", "");
	let words = line1.split(",").collect::<Vec<_>>();

	let ins = lines[2];

	let part1 = words.into_iter().map(|x| ins.match_indices(x).count()).sum::<usize>();

	println!("Part 1: {}", part1);	
}

fn part2() {
	let file = read_to_string("input/quest2_p2.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let line1 = lines[0].replace("WORDS:", "");
	let words = line1.split(",").collect::<Vec<_>>();
	let wordsr = words.iter().cloned().map(|x| x.chars().rev().collect::<String>()).collect::<Vec<_>>();

	let ins = lines[2..].to_vec();

	let part2 = ins.into_iter().map(|i| {
		let mut matches = find_matches(&words, i);
		let mut matchesr = find_matches(&wordsr.iter().map(|x| x.as_str()).collect(), i);
		matches.append(&mut matchesr);
		matches.into_iter().collect::<HashSet<_>>().len()
	}).sum::<usize>();

	println!("Part 2: {:?}", part2);
}

fn find_matches(words: &Vec<&str>, ins: &str) -> Vec<usize>{
	let mut indexes = Vec::new();
	let chars = ins.chars().collect::<Vec<_>>();
	for &word in words {
		let word_chars = word.chars().collect::<Vec<_>>();
		'check: for i in 0..chars.len() {
			for j in 0..word.len() {
				if i+j >= chars.len() || chars[i+j] != word_chars[j] {
					continue 'check;
				}
			}
			for j in 0..word.len() {
				indexes.push(i+j);
			}
		}
	}
	indexes
}

fn find_matches_around(words: &Vec<&str>, ins: &str) -> Vec<usize>{
	let mut indexes = Vec::new();
	let chars = ins.chars().collect::<Vec<_>>();
	for &word in words {
		let word_chars = word.chars().collect::<Vec<_>>();
		'check: for i in 0..chars.len() {
			for j in 0..word.len() {
				let mut ij = i+j;
				if ij >= chars.len() {
					ij = ij - chars.len();
				}
				if chars[ij] != word_chars[j] {
					continue 'check;
				}
			}
			for j in 0..word.len() {
				let mut ij = i+j;
				if ij >= chars.len() {
					ij = ij - chars.len();
				}
				indexes.push(ij);
			}
		}
	}
	indexes
}

fn part3() {
	let file = read_to_string("input/quest2_p3.txt").unwrap();

	let lines = file.lines().collect::<Vec<_>>();

	let line1 = lines[0].replace("WORDS:", "");
	let words = line1.split(",").collect::<Vec<_>>();
	let wordsr = words.iter().cloned().map(|x| x.chars().rev().collect::<String>()).collect::<Vec<_>>();

	let ins_front = lines[2..].to_vec();
	let ins_down = (0 .. lines[2].len()).map(|x| (0..ins_front.len()).map(|y| lines[y+2].chars().nth(x).unwrap()).collect::<String>()).collect::<Vec<_>>();

	let mut part3front = ins_front.into_iter().enumerate().flat_map(|(x, i)| {
		let mut matches = find_matches_around(&words, &i);
		let mut matchesr = find_matches_around(&wordsr.iter().map(|x| x.as_str()).collect(), &i);
		matches.append(&mut matchesr);
		matches.into_iter().map(|y| (x, y)).collect::<Vec<_>>()
	}).collect::<Vec<_>>();
	
	let mut part3down = ins_down.into_iter().enumerate().flat_map(|(y, i)| {
		let mut matches = find_matches(&words, i.as_str());
		let mut matchesr = find_matches(&wordsr.iter().map(|x| x.as_str()).collect(), i.as_str());
		matches.append(&mut matchesr);
		matches.into_iter().map(|x| (x, y)).collect::<Vec<_>>()
	}).collect::<Vec<_>>();

	part3front.append(&mut part3down);
	let part3 = part3front.into_iter().collect::<HashSet<_>>().len();

	println!("Part 3: {:?}", part3);
}