use std::{collections::{HashMap, HashSet}, fs::read_to_string};


fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest10_p1.txt").unwrap();
	
	let matrix = file.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let res = find_word(0, 0, &matrix);

	println!("Part 1: {}", res);
}

fn find_word(base_x: usize, base_y: usize, matrix: &Vec<Vec<char>>) -> String {
	let mut res = String::new();
	let base_x = base_x * 9;
	let base_y = base_y * 9;
	for x in 2..6 {
		let x = base_x + x;
		for y in 2..6 {
			let y = base_y + y;
			let l1 = vec![matrix[x][0 + base_y], matrix[x][1 + base_y], matrix[x][6 + base_y], matrix[x][7 + base_y]];
			let l2 = vec![matrix[0 + base_x][y], matrix[1 + base_x][y], matrix[6 + base_x][y], matrix[7 + base_x][y]];
			let c = l1.into_iter().filter(|x| l2.contains(x)).next().unwrap();
			res.push(c);
		}
	}

	res
}

fn part2() {
	let file = read_to_string("input/quest10_p2.txt").unwrap();

	let matrix = file.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut res = 0;
	for base_x in 0..7 {
		for base_y in 0..15 {
			let word = find_word(base_x, base_y, &matrix);
			res += word_power(word);
		}
	}
	
	println!("Part 2: {}", res);
}

fn word_power(word: String) -> usize {
	let map = HashMap::from([
		('A', 1), 
		('B', 2),
		('C', 3), 
		('D', 4),
		('E', 5),
		('F', 6),
		('G', 7),
		('H', 8),
		('I', 9),
		('J', 10),
		('K', 11),
		('L', 12),
		('M', 13),
		('N', 14),
		('O', 15),
		('P', 16),
		('Q', 17),
		('R', 18),
		('S', 19),
		('T', 20),
		('U', 21),
		('V', 22),
		('W', 23),
		('X', 24),
		('Y', 25),
		('Z', 26)
	]);

	word.chars().enumerate().map(|(x, c)| (x+1) * map[&c]).sum()
}

fn part3() {
	let file = read_to_string("input/quest10_p3.txt").unwrap();

	let mut matrix = file.lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut found = HashSet::new();

	let mut res = 0;
	loop {
		let mut hit = false;
		for base_x in 0..10 {
			for base_y in 0..20 {
				if found.contains(&(base_x, base_y)) {
					continue;
				}

				let word = find_word3(base_x, base_y, &mut matrix);
				if let Some(word) = word {
					res += word_power(word);
	
					found.insert((base_x, base_y));
					hit = true;
				}
			}
		}
		if !hit {
			break;
		}
	}
	
	println!("Part 3: {}", res);
}

fn find_word3(base_x: usize, base_y: usize, matrix: &mut Vec<Vec<char>>) -> Option<String> {
	let base_x = base_x * 6;
	let base_y = base_y * 6;

	let mut existing = HashMap::new();

	loop {
		let mut hit = false;

		for x in 2..6 {
			let x = base_x + x;
			for y in 2..6 {
				let y = base_y + y;

				if existing.contains_key(&(x, y)) {
					continue;
				}

				let taken1 = vec![existing.get(&(x, 2 + base_y)), existing.get(&(x, 3 + base_y)), existing.get(&(x, 4 + base_y)), existing.get(&(x, 5 + base_y))].into_iter().filter_map(|x| x).map(|x| *x).collect::<Vec<_>>();
				let taken2 = vec![existing.get(&(2 + base_x, y)), existing.get(&(3 + base_x, y)), existing.get(&(4 + base_x, y)), existing.get(&(5 + base_x, y))].into_iter().filter_map(|x| x).map(|x| *x).collect::<Vec<_>>();

				let mut l1 = vec![matrix[x][0 + base_y], matrix[x][1 + base_y], matrix[x][6 + base_y], matrix[x][7 + base_y]]
					.into_iter().filter(|x| !taken1.contains(x)).collect::<Vec<_>>();
				let mut l2 = vec![matrix[0 + base_x][y], matrix[1 + base_x][y], matrix[6 + base_x][y], matrix[7 + base_x][y]]
					.into_iter().filter(|x| !taken2.contains(x)).collect::<Vec<_>>();

				if l1.len() == 1 && l2.len() == 1 {
					if l1[0] == '?' && l2[0] == '?' {
						continue;
					}

					if l1[0] == '?' {
						l1 = l2.clone();
						for y in [0, 1, 6, 7] {
							if matrix[x][y + base_y] == '?' {
								matrix[x][y + base_y] = l1[0];
							}
						}
						
					}

					if l2[0] == '?' {
						l2 = l1.clone();
						for x in [0, 1, 6, 7] {
							if matrix[x + base_x][y] == '?' {
								matrix[x + base_x][y] = l1[0];
							}
						}
					}
				}

				let c = l1.into_iter().filter(|x| l2.contains(x)).next();

				if let Some(c) = c {
					existing.insert((x, y), c);
					hit = true;
				}
			}
		}

		if !hit {
			break;
		}
	}

	let mut res = String::new();
	
	for x in 2..6 {
		for y in 2..6 {
			if let Some(c) = existing.get(&(x + base_x, y + base_y)) {
				res.push(*c);
			} else {
				return None;
			}
		}
	}

	Some(res)
}