use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest9/input1.txt").unwrap();
    let input = parse(file);
    let part1 = part1(input);
    println!("Part 1: {}", part1);

    let file = read_to_string("input/quest9/input2.txt").unwrap();
    let input = parse(file);
    let part2 = part2and3(input);
    println!("Part 2: {}", part2);

    let file = read_to_string("input/quest9/input3.txt").unwrap();
    let input = parse(file);
    let part3 = part2and3(input);
    println!("Part 3: {}", part3);
}

fn parse(file: String) -> Vec<(i64, i64, i64)> {
    file.lines()
        .map(|line| {
            let mut split = line.split(", ");
            let id = split.next().unwrap().replace("id=", "").parse::<i64>().unwrap();
            let growth = split.next().unwrap().replace("growth=", "").parse::<i64>().unwrap();
            let limit = split.next().unwrap().replace("limit=", "").parse::<i64>().unwrap();
            (id, growth, limit)
        }).collect::<Vec<_>>()
}

fn part1(input: Vec<(i64, i64, i64)>) -> u32 {
    input.into_iter()
        .map(|(_, growth, limit)| {
            let mut seconds = 1;
            let mut elements = vec![1];
            let mut splits = 0;
            while splits + elements.iter().sum::<i64>() < limit {
                seconds += 1;

                let mut new_elements = Vec::new();
                for element in elements {
                    let element = element + 1;
                    if element >= growth {
                        let element = element / 2;
                        new_elements.push(element);
                        new_elements.push(element);
                        new_elements.push(1);
                        splits += 1;
                    } else {
                        new_elements.push(element);
                    }
                }
                elements = new_elements;
            }
            seconds
        }).sum::<u32>()
}


fn part2and3(input: Vec<(i64, i64, i64)>) -> i64 {
    input.into_iter()
        .map(|(id, growth, limit)| {
            let mut seconds = 1;
            let mut elements = vec![(1, 1i64)];
            let mut splits = 0;
            while splits + 1 < limit {
                let seconds_used = elements.iter()
                    .map(|(x, _)| growth - x)
                    .min()
                    .unwrap();

                seconds += seconds_used;

                let mut new_elements = Vec::new();
                for (element, times) in elements {
                    let element = element + seconds_used;
                    if element == growth {
                        let element = element / 2;
                        new_elements.push((element, times * 2));
                        new_elements.push((1, times));
                        splits += times;
                    } else if element > growth {
                        panic!("Growth should happen before element grows");
                    } else {
                        new_elements.push((element, times));
                    }
                }
                new_elements.sort();
                elements = new_elements.into_iter()
                    .as_slice()
                    .chunk_by(|x1, x2| x1.0 == x2.0)
                    .map(|chunk| (chunk[0].0, chunk.iter().map(|&(_, x)| x).sum()))
                    .collect();
            }
            seconds
        }).sum::<i64>()
}