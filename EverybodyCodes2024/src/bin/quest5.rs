use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string, usize};

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest5_p1.txt").unwrap();

	let matrix = file.lines().map(|line| line.chars().map(|x| x).collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut rows = (0..4).map(|y| (0..matrix.len()).map(|x| matrix[x][y*2].to_digit(10).unwrap()).collect::<VecDeque<_>>()).collect::<Vec<_>>();

	let mut row = 0;

	let mut round = 0;

	while round < 10 {
		let first = rows[row].pop_front().unwrap();

		let next_pos = first - 1;
		let mut next_row = row + 1;
		next_row %= 4;

		rows[next_row].insert(next_pos as usize, first);

		round += 1;

		row += 1;
		row %= 4;
	}

	let res = rows.iter().map(|x| x[0].to_string()).collect::<String>();

	println!("Part 1: {}", res);
}

fn part2() {
	let file = read_to_string("input/quest5_p2.txt").unwrap();

	let matrix = file.lines().map(|line| line.split(" ").collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut rows = (0..4).map(|y| (0..matrix.len()).map(|x| matrix[x][y].parse::<u32>().unwrap()).collect::<VecDeque<_>>()).collect::<Vec<_>>();

	let mut row = 0;

	let mut round = 0;

	let mut count = HashMap::new();

	loop {
		let first = rows[row].pop_front().unwrap();

		let next_pos = first - 1;
		let mut next_row = row + 1;
		next_row %= 4;

		rows[next_row].insert(next_pos as usize, first);

		round += 1;

		row += 1;
		row %= 4;

		let res = rows.iter().map(|x| x[0].to_string()).collect::<String>();

		*count.entry(res.clone()).or_insert(0) += 1;

		if count.get(&res).unwrap() == &2024 {

			let num = res.parse::<usize>().unwrap();
			let res = round * num;
			println!("Part 2: {}", res);
			break;
		}
	}
}

fn part3() {
	let file = read_to_string("input/quest5_p3.txt").unwrap();

	let matrix = file.lines().map(|line| line.split(" ").collect::<Vec<_>>()).collect::<Vec<_>>();

	let mut rows = (0..4).map(|y| (0..matrix.len()).map(|x| matrix[x][y].parse::<u32>().unwrap()).collect::<VecDeque<_>>()).collect::<Vec<_>>();

	let mut row = 0;

	let mut res = usize::MIN;

	let mut state = HashSet::new();

	state.insert(rows.clone());

	loop {
		let first = rows[row].pop_front().unwrap();

		let mut next_pos = first;
		let mut next_row = row + 1;
		next_row %= 4;

		while rows[next_row].len() * 2 < next_pos as usize {
			next_pos -= rows[next_row].len() as u32 * 2;
		}

		if rows[next_row].len() < next_pos as usize{
			next_pos = rows[next_row].len() as u32 * 2 - next_pos + 1;
		} else {
			next_pos -= 1;
		}

		rows[next_row].insert(next_pos as usize, first);

		row += 1;
		row %= 4;

		let str = rows.iter().map(|x| x[0].to_string()).collect::<String>();
		let num = str.parse::<usize>().unwrap();

		res = res.max(num);

		if state.contains(&rows) {
			break;
		}

		state.insert(rows.clone());
	}

	println!("Part 3: {}", res);
}