use std::cmp::Reverse;
use std::fs::read_to_string;
use std::ops::Range;
use regex::Regex;

fn main() {
    let file1 = read_to_string("input/quest4/input1.txt").unwrap();
    let input1 = parse(file1);

    let mut scoring = input1.into_iter()
        .as_slice()
        .chunk_by(|ch, ch2| ch[0] == ch2[0])
        .map(|chunked| chunked.iter().map(|c| c[3]).sum::<u32>())
        .collect::<Vec<_>>();

    scoring.sort();

    let part1 = scoring
        .into_iter()
        .enumerate()
        .map(|(pos, total)| (pos as u32 + 1) * total)
        .sum::<u32>();

    println!("Part 1: {}", part1);

    let file2 = read_to_string("input/quest4/input2.txt").unwrap();
    let input2 = parse(file2);

    let mut scoring2 = input2.into_iter()
        .as_slice()
        .chunk_by(|ch, ch2| ch[0] == ch2[0])
        .map(|chunked| (Reverse(chunked.len()), chunked.iter().map(|c| c[3]).sum::<u32>()))
        .collect::<Vec<_>>();

    scoring2.sort();

    let part2 = scoring2
        .into_iter()
        .enumerate()
        .map(|(pos, (_, total))| (pos as u32 + 1) * total)
        .sum::<u32>();

    println!("Part 2: {}", part2);

    let file3 = read_to_string("input/quest4/input3.txt").unwrap();
    let mut input3 = parse(file3);
    input3.sort_by_key(|ch| (ch[1], ch[2]));

    let mut scoring = input3.into_iter()
        .as_slice()
        .chunk_by(|ch, ch2| ch[1] == ch2[1])
        .flat_map(|quest_chunk| {
            let scores1 = point_scores(quest_chunk, vec![1], 0..51);
            let mut scores2 = point_scores(quest_chunk, vec![1,2], 0..101);
            let mut scores3 = point_scores(quest_chunk, vec![1,2,3], 0..151);

            let mut scores = scores1;
            scores.append(&mut scores2);
            scores.append(&mut scores3);

            scores.sort_by_key(|ch| ch.0);

            scores.into_iter()
                .as_slice()
                .chunk_by(|ch, ch2| ch.0 == ch2.0)
                .map(|chunk| (chunk[0].0, chunk.into_iter().map(|ch| ch.1).sum::<u32>()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    scoring.sort();

    let mut scoring = scoring.into_iter()
        .as_slice()
        .chunk_by(|ch, ch2| ch.0 == ch2.0)
        .map(|chunk| chunk.into_iter().map(|ch| ch.1).sum::<u32>())
        .collect::<Vec<_>>();
    scoring.sort();
    scoring.reverse();

    let mut pos = 1;
    let part3 = scoring.into_iter()
        .as_slice()
        .chunk_by(|ch, ch2| ch == ch2)
        .map(|chunk| {
            let position = pos;
            for _ in 0..chunk.len() {
                pos += 1;
            }
            chunk.into_iter().map(|ch| position * ch).sum::<u32>()
        })
        .sum::<u32>();

    println!("Part 3: {}", part3);
}

fn point_scores(quest_chunk: &[Vec<u32>], parts: Vec<u32>, scores: Range<u32>) -> Vec<(u32, u32)> {
    let mut scores = scores.into_iter().collect::<Vec<u32>>();
    scores.reverse();
    let mut scores = scores.into_iter();
    let contains_last = quest_chunk.iter()
        .filter(|ch| parts.last().unwrap() == &ch[2])
        .map(|ch| ch[0])
        .collect::<Vec<u32>>();
    let mut scoring = quest_chunk.into_iter()
        .filter(|ch| parts.contains(&ch[2]) && contains_last.contains(&ch[0]))
        .collect::<Vec<_>>();
    scoring.sort();
    let mut scoring = scoring.into_iter()
        .as_slice()
        .chunk_by(|ch, ch2| ch[0] == ch2[0])
        .map(|chunked| (chunked[0][0], chunked.iter()
            .map(|c| c[3]).sum::<u32>()))
        .collect::<Vec<_>>();
    scoring.sort_by_key(|ch| ch.1);
    scoring.into_iter()
        .as_slice()
        .chunk_by(|ch, ch2| ch.1 == ch2.1)
        .flat_map(|chunk| {
            let score = scores.next().unwrap_or(0);
            for _ in 1..chunk.len() {
                scores.next();
            }
            chunk.into_iter().map(move |ch| (ch.0, score))
        })
        .collect::<Vec<_>>()
}

fn parse(file: String) -> Vec<Vec<u32>> {
    let regex = Regex::new(r"\d+").unwrap();
    let mut parsed = file.lines()
        .map(|line| regex.find_iter(line)
            .filter_map(|m| m.as_str().parse::<u32>().ok())
            .collect::<Vec<u32>>())
        .collect::<Vec<_>>();
    parsed.sort();
    parsed
}