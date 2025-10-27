use std::fs::read_to_string;

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest17_p1.txt").unwrap();

	let res = solve_1a2(file);
	println!("Part 1: {}", res);
}

fn least(xs: &Vec<(usize, usize)>, ys: &Vec<(usize, usize)>) -> usize {
	xs.iter().flat_map(|x| ys.iter().map(|y| dist(*x, *y))).min().unwrap()
}

fn dist(x: (usize, usize), y: (usize, usize)) -> usize {
	x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}

fn part2() {
	let file = read_to_string("input/quest17_p2.txt").unwrap();

	let res = solve_1a2(file);
	println!("Part 2: {}", res);
}

fn solve_1a2(file: String) -> usize {
	let stars = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().filter_map(move |(x, c)| if c == '*' { Some((x, y)) } else {None})
	}).collect::<Vec<_>>();

	let starlen = stars.len();

	let mut groups = stars.into_iter().map(|x| vec![x]).collect::<Vec<_>>();

	let mut res = 0;

	while groups.len() > 1 {
		let group1 = groups.get(0).unwrap();
		let nearest = groups[1..].iter().min_by_key(|xs| least(group1, xs)).unwrap();

		let size = least(group1, nearest);
		res += size;
		
		let pos = groups.iter().position(|x| x == nearest).unwrap();
		let nearest = nearest.clone();
		let group1 = groups.get_mut(0).unwrap();

		for x in nearest {
			group1.push(x.clone());
		}

		groups.remove(pos);
	}

	res + starlen
}

fn part3() {
	let file = read_to_string("input/quest17_p3.txt").unwrap();
	let stars = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().filter_map(move |(x, c)| if c == '*' { Some((x, y)) } else {None})
	}).collect::<Vec<_>>();

	let mut groups = stars.into_iter().map(|x| vec![x]).collect::<Vec<_>>();

	let mut all = Vec::new();

	let mut i = 0;
	let mut local = 0;

	while i < groups.len()-1 {
		let group1 = groups.get(i).unwrap();
		let nearest = groups[i+1..].iter().min_by_key(|xs| least(group1, xs)).unwrap();

		let size = least(group1, nearest);
		if size >= 6 {
			i += 1;
			all.push(local + group1.len());
			local = 0;
			continue;
		}
		local += size;
		
		let pos = groups.iter().position(|x| x == nearest).unwrap();
		let nearest = nearest.clone();
		let group1 = groups.get_mut(i).unwrap();

		for x in nearest {
			group1.push(x.clone());
		}

		groups.remove(pos);
	}
	all.push(local + groups[i].len());

	all.sort();
	all.reverse();
	
	let res = all[0] * all[1] * all[2];

	println!("Part 3: {}", res);
}