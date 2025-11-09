use event_2025_the_song_of_ducks_and_dragons::solve;

enum Direction {
    Right(i32),
    Left(i32)
}

fn main() {
    solve("1", part1, part2, part3);
}

fn part1(file: &str) -> String {
    let (names, instructions) = parse(&file);

    let mut curr = 0;
    for instruction in instructions {
        match instruction {
            Direction::Right(n) => curr += n,
            Direction::Left(n) => curr -= n
        }
        if curr < 0 {
            curr = 0;
        }
        if curr >= names.len() as i32 {
            curr = names.len() as i32 - 1;
        }
    }
    names[curr as usize].to_string()
}

fn part2(file: &str) -> String {
    let (names, instructions) = parse(&file);

    let mut curr = 0;
    for instruction in instructions {
        match instruction {
            Direction::Right(n) => curr += n,
            Direction::Left(n) => curr -= n
        }
        while curr < 0 {
            curr += names.len() as i32;
        }
        while curr >= names.len() as i32 {
            curr -= names.len() as i32;
        }
    }
    names[curr as usize].to_string()
}

fn part3(file: &str) -> String {
    let (names, instructions) = parse(&file);

    let mut names = names;

    for instruction in instructions {
        let mut replace = match instruction {
            Direction::Right(n) => n,
            Direction::Left(n) => -n
        };
        while replace < 0 {
            replace += names.len() as i32;
        }
        while replace >= names.len() as i32 {
            replace -= names.len() as i32;
        }
        let temp = names[0];
        names[0] = names[replace as usize];
        names[replace as usize] = temp;
    }
    names[0].to_string()
}

fn parse(file: &str) -> (Vec<&str>, Vec<Direction>) {
    let mut split = file.split("\n\n");
    let names = split.next().unwrap().split(",").collect::<Vec<_>>();
    let inst = split.next().unwrap()
        .split(",")
        .map(|inst| {
            let chs = inst.chars().collect::<Vec<_>>();
            let num = inst.replace("R", "")
                .replace("L", "")
                .parse::<i32>().unwrap();
            match chs[0] {
                'R' => Direction::Right(num),
                'L' => Direction::Left(num),
                _ => panic!("Unknown direction: {}", inst)
            }
        })
        .collect::<Vec<_>>();
    (names, inst)
}