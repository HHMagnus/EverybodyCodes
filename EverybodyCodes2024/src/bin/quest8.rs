use std::fs::read_to_string;

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest8_p1.txt").unwrap();

	let blocks = file.parse::<usize>().unwrap();

	let mut width = 1;
	let mut total = 1;

	while total < blocks {
		width += 2;
		total += width;
	}

	let res = (total - blocks) * width;
	println!("Part 1: {}", res);
}

fn part2() {
	let file = read_to_string("input/quest8_p2.txt").unwrap();

	let nullpointer_priests = file.parse::<usize>().unwrap();
	let priest_acolytes = 1111;
	let blocks_available = 20240000;
	let mut width = 1;
	let mut thickness = 1;
	let mut total = 1;

	while total < blocks_available {
		thickness *= nullpointer_priests;
		thickness %= priest_acolytes;

		width += 2;

		total += width * thickness;
	}

	let res = (total - blocks_available) * width;
	println!("Part 2: {}", res);
}

fn part3() {
	let file = read_to_string("input/quest8_p3.txt").unwrap();

	let nullpointer_priests = file.parse::<usize>().unwrap();
	let blocks_available = 202400000;
	let high_priest_acolytes = 10;

	let mut width = 1;
	let mut thickness = 1;
	let mut total = 1;

	let mut columns = vec![1 as usize];

	let mut res = 0;

	while blocks_available > res {
		thickness *= nullpointer_priests;
		thickness %= high_priest_acolytes;
		thickness += high_priest_acolytes;

		width += 2;

		total += width * thickness;

		columns = columns.into_iter().map(|x| (x + thickness)).collect();
		columns.push(thickness);

		res = total - removed3(&columns, width, nullpointer_priests, high_priest_acolytes);
	}

	println!("Part 3: {}", res - blocks_available);
}

fn removed3(columns: &Vec<usize>, width: usize, nullpointer_priests: usize, high_priest_acolytes: usize) -> usize {
	if columns.len() == 0 {
		return 0;
	}
	let mut removed = columns.into_iter().map(|x| ((x * width * nullpointer_priests) % high_priest_acolytes) * 2).collect::<Vec<_>>();
	removed[0] /= 2;
	removed.pop();
	removed.iter().sum::<usize>()
}