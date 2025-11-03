use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/quest1_p1.txt").unwrap();

	let part1 = file.chars().into_iter().map(|x| 
		if x == 'A' { 0 } else if x == 'B' { 1 } else { 3 }
	).sum::<i32>();

	println!("Part 1: {}", part1);

	let file = read_to_string("input/quest1_p2.txt").unwrap();

	let vec = file.chars().into_iter().collect::<Vec<char>>();

	let part2 = vec.as_slice().chunks(2).map(|xs| {
		let x = xs[0];
		let y = xs[1];
		let sum = num(x) + num(y);
		return if x != 'x' && y != 'x' { sum + 2 } else { sum }
	}).sum::<usize>();

	println!("Part 2: {}", part2);

	let file = read_to_string("input/quest1_p3.txt").unwrap();

	let vec = file.chars().into_iter().collect::<Vec<char>>();

	let part3 = vec.as_slice().chunks(3).map(|chars| {
		let x = chars[0];
		let y = chars[1];
		let z = chars[2];
		let sum = num(x) + num(y) + num(z);
		let xs = chars.iter().filter(|&&x| x == 'x').count();
		if xs == 0 {
			return sum + 6;
		}
		if xs == 1 {
			return sum + 2;
		}
		sum
	}).sum::<usize>();

	println!("Part 3: {}", part3);
}

fn num(x: char) -> usize {
	if x == 'B' { 1 } else if x == 'C' { 3 } else if x == 'D' { 5 } else { 0 }
}