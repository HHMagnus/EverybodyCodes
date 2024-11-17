use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string, path::MAIN_SEPARATOR};
use itertools::Itertools;

fn main() {
	part1();
	part2();
	part3();
}

fn part1() {
	let file = read_to_string("input/quest7_p1.txt").unwrap();

	let mut parsed = file.lines().map(|line| {
		let mut split = line.split(":");
		let plan = split.next().unwrap();
		let list = split.next().unwrap().split(",").collect::<Vec<_>>();
		(plan, list)
	}).collect::<Vec<_>>();

	parsed.sort_by_key(|plan| {
		let mut power = 10;
		let mut total = 0;

		let mut actions = plan.1.iter().cycle();
		for _ in 0..10 {
			let action = *actions.next().unwrap();

			if action == "+" {
				power += 1;
			} else if action == "-" {
				power -= 1;
			}

			total += power;
		}

		total
	});

	parsed.reverse();

	let res = parsed.iter().map(|x| x.0).collect::<String>();

	println!("Part 1: {}", res);
}

fn part2() {
	let file = read_to_string("input/quest7_p2.txt").unwrap();

	let mut parsed = file.lines().map(|line| {
		let mut split = line.split(":");
		let plan = split.next().unwrap();
		let list = split.next().unwrap().split(",").collect::<Vec<_>>();
		(plan, list)
	}).collect::<Vec<_>>();

	let mut track = String::new();
	track.push_str("-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--");
	track.push_str("-=++==-");
	track.push_str(&"--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-".chars().rev().collect::<String>());
	track.push_str("-=+=+=-S");

	// Test track
	// track = "+===++-=+=-S".to_string();

	parsed.sort_by_key(|plan| {
		let mut power = 10;
		let mut total = 0;

		let mut actions = plan.1.iter().cycle();
		for _ in 0..10 {
			for c in track.chars() {
				let action = *actions.next().unwrap();

				if c == '+' {
					power += 1;
				} else if c == '-' {
					power -= 1;
				} else if action == "+" {
					power += 1;
				} else if action == "-" {
					power -= 1;
				}
				total += power;
			}
		}

		total
	});

	parsed.reverse();

	let res = parsed.iter().map(|x| x.0).collect::<String>();

	println!("Part 2: {}", res);
}

fn part3() {
	let file = read_to_string("input/quest7_p3.txt").unwrap();

	let parsed = file.lines().map(|line| {
		let mut split = line.split(":");
		let plan = split.next().unwrap();
		let list = split.next().unwrap().split(",").collect::<Vec<_>>();
		(plan, list)
	}).collect::<Vec<_>>();

	let track = &track3();

	let plans = "+++++---===";

	let plans = plans.chars().into_iter().permutations(11).unique().collect::<Vec<_>>();

	let rival = parsed[0].1.clone().iter().map(|x| x.chars().next().unwrap()).collect::<Vec<_>>();

	let rival = solve3(&rival, track);

	let res = plans.iter().map(|x| solve3(x, track)).filter(|&x| x > rival).count();

	println!("Part 3: {}", res);
}

fn solve3(plan: &Vec<char>, track: &str) -> usize {
	let mut power = 10;
	let mut total = 0;

	let mut actions = plan.iter().cycle();
	for _ in 0..2024 {
		for c in track.chars() {
			let action = *actions.next().unwrap();

			if c == '+' {
				power += 1;
			} else if c == '-' {
				power -= 1;
			} else if action == '+' {
				power += 1;
			} else if action == '-' {
				power -= 1;
			}
			total += power;
		}
	}

	total
}

fn track3() -> String {
	let track =
"S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-";
	let map = track.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter_map(|(x, c)| if c != ' ' {Some(((x, y), c))} else { None }).collect::<Vec<_>>()).collect::<HashMap<_,_>>();

	let mut visited = HashSet::new();
	visited.insert((1, 0));

	let mut curr = (2, 0);

	let mut res = String::new();
	res.push('+');

	loop {
		visited.insert(curr);
		res.push(map[&curr]);

		if map[&curr] == 'S' {
			break;
		}

		let next = [
			(curr.0 - 1, curr.1),
			(curr.0 + 1, curr.1),
			(curr.0, curr.1 - 1),
			(curr.0, curr.1 + 1)
		].into_iter().filter(|x| !visited.contains(x) && map.contains_key(x)).collect::<Vec<_>>();

		curr = next[0];
	}

	res
}