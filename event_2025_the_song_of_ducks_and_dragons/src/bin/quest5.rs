use std::cmp::Ordering;
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("5", part1, part2, part3);
}

#[derive(PartialEq, Eq, Debug)]
struct Sword {
    id: usize,
    nums: Vec<i64>,
    fish_bone: FishBone,
    quality: i64,
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.quality < other.quality {
            return Ordering::Less;
        }

        if self.quality > other.quality {
            return Ordering::Greater;
        }

        let mut i = 0;

        loop {
            let self_quality = self.fish_bone.segment_quality(i);
            let other_quality = other.fish_bone.segment_quality(i);

            if self_quality < other_quality {
                return Ordering::Less;
            }

            if self_quality > other_quality {
                return Ordering::Greater;
            }

            if self_quality.is_none() && other_quality.is_none() {
                return Ordering::Equal;
            }

            i += 1;
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct FishBoneSegment {
    left: Option<i64>,
    center: i64,
    right: Option<i64>,
}

impl FishBoneSegment {
    fn new(num: i64) -> FishBoneSegment {
        FishBoneSegment {
            left: None,
            center: num,
            right: None,
        }
    }

    fn quality(&self) -> i64 {
        let mut vec = Vec::new();
        if let Some(left) = self.left {
            vec.push(left);
        }
        vec.push(self.center);
        if let Some(right) = self.right {
            vec.push(right);
        }
        vec.into_iter()
            .map(|i| i.to_string())
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct FishBone {
    segments: Vec<FishBoneSegment>,
}

impl FishBone {
    fn new(nums: &Vec<i64>) -> FishBone {
        let mut fish_bone = FishBone { segments: vec![] };
        for num in nums {
            fish_bone.insert(*num);
        }
        fish_bone
    }

    fn insert(&mut self, num: i64) {
        for segment in &mut self.segments {
            if segment.left.is_none() && num < segment.center {
                segment.left = Some(num);
                return;
            }
            if segment.right.is_none() && num > segment.center {
                segment.right = Some(num);
                return;
            }
        }
        self.segments.push(FishBoneSegment::new(num))
    }

    fn centers(&self) -> Vec<i64> {
        self.segments.iter()
            .map(|s| s.center)
            .collect()
    }

    fn quality(&self) -> i64 {
        self.centers().into_iter()
            .map(|num| num.to_string())
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    }

    fn segment_quality(&self, i: usize) -> Option<i64> {
        self.segments.get(i)
            .map(|s| s.quality())
    }
}

impl Sword {
    fn parse(file: &str) -> Sword {
        let mut split = file.split(':');
        let id = split.next().unwrap().parse::<usize>().unwrap();
        let nums = split.next().unwrap()
            .split(",")
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        let fish_bone = FishBone::new(&nums);
        let quality = fish_bone.quality();
        Sword { id, nums, fish_bone, quality }
    }
}

fn part1(file: &str) -> i64 {
    let sword = Sword::parse(file);
    sword.quality
}

fn part2(file: &str) -> i64 {
    let qualities = file.split("\n")
        .map(Sword::parse)
        .map(|s| s.quality)
        .collect::<Vec<_>>();
    qualities.iter().max().unwrap() - qualities.iter().min().unwrap()
}

fn part3(file: &str) -> usize {
    let mut swords = file.split("\n")
        .map(Sword::parse)
        .collect::<Vec<_>>();

    swords.sort();
    swords.reverse();

    swords.into_iter().enumerate()
        .map(|(i, sword)| (i+1) * sword.id)
        .sum()
}